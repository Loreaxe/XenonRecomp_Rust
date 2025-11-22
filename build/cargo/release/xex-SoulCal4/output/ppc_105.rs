pub fn sub_82656B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656B00 size=108
    let mut pc: u32 = 0x82656B00;
    'dispatch: loop {
        match pc {
            0x82656B00 => {
    //   block [0x82656B00..0x82656B6C)
	// 82656B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656B0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656B14: 38EBB79C  addi r7, r11, -0x4864
	ctx.r[7].s64 = ctx.r[11].s64 + -18532;
	// 82656B18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656B1C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82656B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656B28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656B30: 386A6D18  addi r3, r10, 0x6d18
	ctx.r[3].s64 = ctx.r[10].s64 + 27928;
	// 82656B34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656B58: 4BE102C9  bl 0x82466e20
	ctx.lr = 0x82656B5C;
	sub_82466E20(ctx, base);
	// 82656B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656B70 size=108
    let mut pc: u32 = 0x82656B70;
    'dispatch: loop {
        match pc {
            0x82656B70 => {
    //   block [0x82656B70..0x82656BDC)
	// 82656B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656B7C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656B84: 38EBB7CC  addi r7, r11, -0x4834
	ctx.r[7].s64 = ctx.r[11].s64 + -18484;
	// 82656B88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82656B8C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82656B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656BA0: 386A6D48  addi r3, r10, 0x6d48
	ctx.r[3].s64 = ctx.r[10].s64 + 27976;
	// 82656BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656BC8: 4BE10259  bl 0x82466e20
	ctx.lr = 0x82656BCC;
	sub_82466E20(ctx, base);
	// 82656BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656BE0 size=108
    let mut pc: u32 = 0x82656BE0;
    'dispatch: loop {
        match pc {
            0x82656BE0 => {
    //   block [0x82656BE0..0x82656C4C)
	// 82656BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656BEC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656BF4: 38EBB7E4  addi r7, r11, -0x481c
	ctx.r[7].s64 = ctx.r[11].s64 + -18460;
	// 82656BF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656BFC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82656C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656C10: 386A6D78  addi r3, r10, 0x6d78
	ctx.r[3].s64 = ctx.r[10].s64 + 28024;
	// 82656C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656C38: 4BE101E9  bl 0x82466e20
	ctx.lr = 0x82656C3C;
	sub_82466E20(ctx, base);
	// 82656C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656C50 size=108
    let mut pc: u32 = 0x82656C50;
    'dispatch: loop {
        match pc {
            0x82656C50 => {
    //   block [0x82656C50..0x82656CBC)
	// 82656C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656C5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656C64: 38EBB818  addi r7, r11, -0x47e8
	ctx.r[7].s64 = ctx.r[11].s64 + -18408;
	// 82656C68: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82656C6C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82656C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656C78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656C80: 386A6DA8  addi r3, r10, 0x6da8
	ctx.r[3].s64 = ctx.r[10].s64 + 28072;
	// 82656C84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656CA8: 4BE10179  bl 0x82466e20
	ctx.lr = 0x82656CAC;
	sub_82466E20(ctx, base);
	// 82656CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656CC0 size=108
    let mut pc: u32 = 0x82656CC0;
    'dispatch: loop {
        match pc {
            0x82656CC0 => {
    //   block [0x82656CC0..0x82656D2C)
	// 82656CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656CCC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656CD4: 38EBB8C0  addi r7, r11, -0x4740
	ctx.r[7].s64 = ctx.r[11].s64 + -18240;
	// 82656CD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656CDC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82656CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656CF0: 386A6DD8  addi r3, r10, 0x6dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 28120;
	// 82656CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656D18: 4BE10109  bl 0x82466e20
	ctx.lr = 0x82656D1C;
	sub_82466E20(ctx, base);
	// 82656D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656D30 size=108
    let mut pc: u32 = 0x82656D30;
    'dispatch: loop {
        match pc {
            0x82656D30 => {
    //   block [0x82656D30..0x82656D9C)
	// 82656D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656D3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656D44: 38EBB8F0  addi r7, r11, -0x4710
	ctx.r[7].s64 = ctx.r[11].s64 + -18192;
	// 82656D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82656D4C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 82656D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656D60: 386A6E08  addi r3, r10, 0x6e08
	ctx.r[3].s64 = ctx.r[10].s64 + 28168;
	// 82656D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656D88: 4BE10099  bl 0x82466e20
	ctx.lr = 0x82656D8C;
	sub_82466E20(ctx, base);
	// 82656D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656DA0 size=108
    let mut pc: u32 = 0x82656DA0;
    'dispatch: loop {
        match pc {
            0x82656DA0 => {
    //   block [0x82656DA0..0x82656E0C)
	// 82656DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656DAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656DB4: 38EBB908  addi r7, r11, -0x46f8
	ctx.r[7].s64 = ctx.r[11].s64 + -18168;
	// 82656DB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656DBC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82656DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656DD0: 386A6E38  addi r3, r10, 0x6e38
	ctx.r[3].s64 = ctx.r[10].s64 + 28216;
	// 82656DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656DF8: 4BE10029  bl 0x82466e20
	ctx.lr = 0x82656DFC;
	sub_82466E20(ctx, base);
	// 82656DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656E10 size=112
    let mut pc: u32 = 0x82656E10;
    'dispatch: loop {
        match pc {
            0x82656E10 => {
    //   block [0x82656E10..0x82656E80)
	// 82656E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656E20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656E24: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 82656E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656E2C: 390BB938  addi r8, r11, -0x46c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18120;
	// 82656E30: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82656E34: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82656E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656E3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656E48: 386A6E68  addi r3, r10, 0x6e68
	ctx.r[3].s64 = ctx.r[10].s64 + 28264;
	// 82656E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82656E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656E6C: 4BE0FFB5  bl 0x82466e20
	ctx.lr = 0x82656E70;
	sub_82466E20(ctx, base);
	// 82656E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656E80 size=24
    let mut pc: u32 = 0x82656E80;
    'dispatch: loop {
        match pc {
            0x82656E80 => {
    //   block [0x82656E80..0x82656E98)
	// 82656E80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656E84: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656E88: 394A1A40  addi r10, r10, 0x1a40
	ctx.r[10].s64 = ctx.r[10].s64 + 6720;
	// 82656E8C: 816BB814  lwz r11, -0x47ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18412 as u32) ) } as u64;
	// 82656E90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82656E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656E98 size=112
    let mut pc: u32 = 0x82656E98;
    'dispatch: loop {
        match pc {
            0x82656E98 => {
    //   block [0x82656E98..0x82656F08)
	// 82656E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656EA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656EA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656EAC: 392AD110  addi r9, r10, -0x2ef0
	ctx.r[9].s64 = ctx.r[10].s64 + -12016;
	// 82656EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656EB4: 390B1A40  addi r8, r11, 0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + 6720;
	// 82656EB8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82656EBC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82656EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656EC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656ED0: 386A6E98  addi r3, r10, 0x6e98
	ctx.r[3].s64 = ctx.r[10].s64 + 28312;
	// 82656ED4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656ED8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656EF4: 4BE0FF2D  bl 0x82466e20
	ctx.lr = 0x82656EF8;
	sub_82466E20(ctx, base);
	// 82656EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656F08 size=108
    let mut pc: u32 = 0x82656F08;
    'dispatch: loop {
        match pc {
            0x82656F08 => {
    //   block [0x82656F08..0x82656F74)
	// 82656F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656F14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656F1C: 38EBB9E4  addi r7, r11, -0x461c
	ctx.r[7].s64 = ctx.r[11].s64 + -17948;
	// 82656F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656F24: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82656F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656F2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656F38: 386A6EC8  addi r3, r10, 0x6ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 28360;
	// 82656F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656F60: 4BE0FEC1  bl 0x82466e20
	ctx.lr = 0x82656F64;
	sub_82466E20(ctx, base);
	// 82656F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656F78 size=116
    let mut pc: u32 = 0x82656F78;
    'dispatch: loop {
        match pc {
            0x82656F78 => {
    //   block [0x82656F78..0x82656FEC)
	// 82656F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656F84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656F88: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656F8C: 390BBA18  addi r8, r11, -0x45e8
	ctx.r[8].s64 = ctx.r[11].s64 + -17896;
	// 82656F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656F94: 392AD154  addi r9, r10, -0x2eac
	ctx.r[9].s64 = ctx.r[10].s64 + -11948;
	// 82656F98: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656F9C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82656FA0: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 82656FA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656FAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656FBC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82656FC0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82656FC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656FC8: 386B6EF8  addi r3, r11, 0x6ef8
	ctx.r[3].s64 = ctx.r[11].s64 + 28408;
	// 82656FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656FD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656FD8: 4BE0FE49  bl 0x82466e20
	ctx.lr = 0x82656FDC;
	sub_82466E20(ctx, base);
	// 82656FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656FF0 size=24
    let mut pc: u32 = 0x82656FF0;
    'dispatch: loop {
        match pc {
            0x82656FF0 => {
    //   block [0x82656FF0..0x82657008)
	// 82656FF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656FF4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656FF8: 394A1AB8  addi r10, r10, 0x1ab8
	ctx.r[10].s64 = ctx.r[10].s64 + 6840;
	// 82656FFC: 816BBA14  lwz r11, -0x45ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17900 as u32) ) } as u64;
	// 82657000: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82657004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657008 size=112
    let mut pc: u32 = 0x82657008;
    'dispatch: loop {
        match pc {
            0x82657008 => {
    //   block [0x82657008..0x82657078)
	// 82657008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657014: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82657018: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265701C: 392AD190  addi r9, r10, -0x2e70
	ctx.r[9].s64 = ctx.r[10].s64 + -11888;
	// 82657020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657024: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 82657028: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265702C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82657030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265703C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657040: 386A6F28  addi r3, r10, 0x6f28
	ctx.r[3].s64 = ctx.r[10].s64 + 28456;
	// 82657044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82657048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265704C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265705C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657064: 4BE0FDBD  bl 0x82466e20
	ctx.lr = 0x82657068;
	sub_82466E20(ctx, base);
	// 82657068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265706C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657078 size=108
    let mut pc: u32 = 0x82657078;
    'dispatch: loop {
        match pc {
            0x82657078 => {
    //   block [0x82657078..0x826570E4)
	// 82657078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657084: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265708C: 38EBBAD8  addi r7, r11, -0x4528
	ctx.r[7].s64 = ctx.r[11].s64 + -17704;
	// 82657090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657094: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82657098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265709C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826570A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826570A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826570A8: 386A6F58  addi r3, r10, 0x6f58
	ctx.r[3].s64 = ctx.r[10].s64 + 28504;
	// 826570AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826570B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826570B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826570B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826570BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826570C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826570C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826570C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826570CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826570D0: 4BE0FD51  bl 0x82466e20
	ctx.lr = 0x826570D4;
	sub_82466E20(ctx, base);
	// 826570D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826570D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826570DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826570E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826570E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826570E8 size=108
    let mut pc: u32 = 0x826570E8;
    'dispatch: loop {
        match pc {
            0x826570E8 => {
    //   block [0x826570E8..0x82657154)
	// 826570E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826570EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826570F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826570F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826570F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826570FC: 38EBBAF0  addi r7, r11, -0x4510
	ctx.r[7].s64 = ctx.r[11].s64 + -17680;
	// 82657100: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82657104: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82657108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265710C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657118: 386A6F88  addi r3, r10, 0x6f88
	ctx.r[3].s64 = ctx.r[10].s64 + 28552;
	// 8265711C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265712C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265713C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657140: 4BE0FCE1  bl 0x82466e20
	ctx.lr = 0x82657144;
	sub_82466E20(ctx, base);
	// 82657144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82657158 size=24
    let mut pc: u32 = 0x82657158;
    'dispatch: loop {
        match pc {
            0x82657158 => {
    //   block [0x82657158..0x82657170)
	// 82657158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265715C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82657160: 394A1B00  addi r10, r10, 0x1b00
	ctx.r[10].s64 = ctx.r[10].s64 + 6912;
	// 82657164: 816BBB20  lwz r11, -0x44e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17632 as u32) ) } as u64;
	// 82657168: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657170 size=112
    let mut pc: u32 = 0x82657170;
    'dispatch: loop {
        match pc {
            0x82657170 => {
    //   block [0x82657170..0x826571E0)
	// 82657170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265717C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82657180: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657184: 392AD1CC  addi r9, r10, -0x2e34
	ctx.r[9].s64 = ctx.r[10].s64 + -11828;
	// 82657188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265718C: 390B1B00  addi r8, r11, 0x1b00
	ctx.r[8].s64 = ctx.r[11].s64 + 6912;
	// 82657190: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82657194: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82657198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265719C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826571A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826571A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826571A8: 386A6FB8  addi r3, r10, 0x6fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 28600;
	// 826571AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826571B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826571B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826571B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826571BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826571C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826571C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826571C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826571CC: 4BE0FC55  bl 0x82466e20
	ctx.lr = 0x826571D0;
	sub_82466E20(ctx, base);
	// 826571D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826571D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826571D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826571DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826571E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826571E0 size=112
    let mut pc: u32 = 0x826571E0;
    'dispatch: loop {
        match pc {
            0x826571E0 => {
    //   block [0x826571E0..0x82657250)
	// 826571E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826571E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826571E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826571EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826571F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826571F4: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 826571F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826571FC: 390BBB24  addi r8, r11, -0x44dc
	ctx.r[8].s64 = ctx.r[11].s64 + -17628;
	// 82657200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82657204: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82657208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265720C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657218: 386A6FE8  addi r3, r10, 0x6fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 28648;
	// 8265721C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265722C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265723C: 4BE0FBE5  bl 0x82466e20
	ctx.lr = 0x82657240;
	sub_82466E20(ctx, base);
	// 82657240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265724C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657250 size=108
    let mut pc: u32 = 0x82657250;
    'dispatch: loop {
        match pc {
            0x82657250 => {
    //   block [0x82657250..0x826572BC)
	// 82657250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265725C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657264: 38EBBB54  addi r7, r11, -0x44ac
	ctx.r[7].s64 = ctx.r[11].s64 + -17580;
	// 82657268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265726C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82657270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265727C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657280: 386A7018  addi r3, r10, 0x7018
	ctx.r[3].s64 = ctx.r[10].s64 + 28696;
	// 82657284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265728C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265729C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826572A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826572A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826572A8: 4BE0FB79  bl 0x82466e20
	ctx.lr = 0x826572AC;
	sub_82466E20(ctx, base);
	// 826572AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826572B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826572B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826572B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826572C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826572C0 size=108
    let mut pc: u32 = 0x826572C0;
    'dispatch: loop {
        match pc {
            0x826572C0 => {
    //   block [0x826572C0..0x8265732C)
	// 826572C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826572C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826572C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826572CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826572D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826572D4: 38EBBB88  addi r7, r11, -0x4478
	ctx.r[7].s64 = ctx.r[11].s64 + -17528;
	// 826572D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826572DC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826572E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826572E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826572E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826572EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826572F0: 386A7048  addi r3, r10, 0x7048
	ctx.r[3].s64 = ctx.r[10].s64 + 28744;
	// 826572F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826572F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826572FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265730C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657318: 4BE0FB09  bl 0x82466e20
	ctx.lr = 0x8265731C;
	sub_82466E20(ctx, base);
	// 8265731C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657330 size=108
    let mut pc: u32 = 0x82657330;
    'dispatch: loop {
        match pc {
            0x82657330 => {
    //   block [0x82657330..0x8265739C)
	// 82657330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265733C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657344: 38EBBBE8  addi r7, r11, -0x4418
	ctx.r[7].s64 = ctx.r[11].s64 + -17432;
	// 82657348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265734C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82657350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265735C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657360: 386A7078  addi r3, r10, 0x7078
	ctx.r[3].s64 = ctx.r[10].s64 + 28792;
	// 82657364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265736C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265737C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657388: 4BE0FA99  bl 0x82466e20
	ctx.lr = 0x8265738C;
	sub_82466E20(ctx, base);
	// 8265738C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826573A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826573A0 size=108
    let mut pc: u32 = 0x826573A0;
    'dispatch: loop {
        match pc {
            0x826573A0 => {
    //   block [0x826573A0..0x8265740C)
	// 826573A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826573A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826573A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826573AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826573B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826573B4: 38EBBC18  addi r7, r11, -0x43e8
	ctx.r[7].s64 = ctx.r[11].s64 + -17384;
	// 826573B8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826573BC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826573C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826573C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826573C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826573CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826573D0: 386A70A8  addi r3, r10, 0x70a8
	ctx.r[3].s64 = ctx.r[10].s64 + 28840;
	// 826573D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826573D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826573DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826573E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826573E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826573E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826573EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826573F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826573F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826573F8: 4BE0FA29  bl 0x82466e20
	ctx.lr = 0x826573FC;
	sub_82466E20(ctx, base);
	// 826573FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657410 size=108
    let mut pc: u32 = 0x82657410;
    'dispatch: loop {
        match pc {
            0x82657410 => {
    //   block [0x82657410..0x8265747C)
	// 82657410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265741C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657424: 38EBBD38  addi r7, r11, -0x42c8
	ctx.r[7].s64 = ctx.r[11].s64 + -17096;
	// 82657428: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265742C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82657430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265743C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657440: 386A70D8  addi r3, r10, 0x70d8
	ctx.r[3].s64 = ctx.r[10].s64 + 28888;
	// 82657444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265744C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265745C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657468: 4BE0F9B9  bl 0x82466e20
	ctx.lr = 0x8265746C;
	sub_82466E20(ctx, base);
	// 8265746C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657480 size=108
    let mut pc: u32 = 0x82657480;
    'dispatch: loop {
        match pc {
            0x82657480 => {
    //   block [0x82657480..0x826574EC)
	// 82657480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265748C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657494: 38EBBD50  addi r7, r11, -0x42b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17072;
	// 82657498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265749C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826574A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826574A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826574A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826574AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826574B0: 386A7108  addi r3, r10, 0x7108
	ctx.r[3].s64 = ctx.r[10].s64 + 28936;
	// 826574B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826574B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826574BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826574C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826574C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826574C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826574CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826574D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826574D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826574D8: 4BE0F949  bl 0x82466e20
	ctx.lr = 0x826574DC;
	sub_82466E20(ctx, base);
	// 826574DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826574E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826574E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826574E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826574F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826574F0 size=108
    let mut pc: u32 = 0x826574F0;
    'dispatch: loop {
        match pc {
            0x826574F0 => {
    //   block [0x826574F0..0x8265755C)
	// 826574F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826574F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826574F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826574FC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657504: 38EBBD68  addi r7, r11, -0x4298
	ctx.r[7].s64 = ctx.r[11].s64 + -17048;
	// 82657508: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265750C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82657510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657514: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265751C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657520: 386A7138  addi r3, r10, 0x7138
	ctx.r[3].s64 = ctx.r[10].s64 + 28984;
	// 82657524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265752C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657548: 4BE0F8D9  bl 0x82466e20
	ctx.lr = 0x8265754C;
	sub_82466E20(ctx, base);
	// 8265754C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657560 size=108
    let mut pc: u32 = 0x82657560;
    'dispatch: loop {
        match pc {
            0x82657560 => {
    //   block [0x82657560..0x826575CC)
	// 82657560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265756C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657574: 38EBBD80  addi r7, r11, -0x4280
	ctx.r[7].s64 = ctx.r[11].s64 + -17024;
	// 82657578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265757C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82657580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657584: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265758C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657590: 386A7168  addi r3, r10, 0x7168
	ctx.r[3].s64 = ctx.r[10].s64 + 29032;
	// 82657594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265759C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826575A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826575A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826575A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826575AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826575B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826575B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826575B8: 4BE0F869  bl 0x82466e20
	ctx.lr = 0x826575BC;
	sub_82466E20(ctx, base);
	// 826575BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826575C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826575C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826575C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826575D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826575D0 size=108
    let mut pc: u32 = 0x826575D0;
    'dispatch: loop {
        match pc {
            0x826575D0 => {
    //   block [0x826575D0..0x8265763C)
	// 826575D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826575D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826575D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826575DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826575E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826575E4: 38EBBD98  addi r7, r11, -0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + -17000;
	// 826575E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826575EC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826575F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826575F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826575F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826575FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657600: 386A7198  addi r3, r10, 0x7198
	ctx.r[3].s64 = ctx.r[10].s64 + 29080;
	// 82657604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265760C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265761C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657628: 4BE0F7F9  bl 0x82466e20
	ctx.lr = 0x8265762C;
	sub_82466E20(ctx, base);
	// 8265762C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657640 size=108
    let mut pc: u32 = 0x82657640;
    'dispatch: loop {
        match pc {
            0x82657640 => {
    //   block [0x82657640..0x826576AC)
	// 82657640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265764C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657654: 38EBBDB0  addi r7, r11, -0x4250
	ctx.r[7].s64 = ctx.r[11].s64 + -16976;
	// 82657658: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265765C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82657660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657664: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657670: 386A71C8  addi r3, r10, 0x71c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29128;
	// 82657674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265767C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265768C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657698: 4BE0F789  bl 0x82466e20
	ctx.lr = 0x8265769C;
	sub_82466E20(ctx, base);
	// 8265769C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826576A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826576A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826576A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826576B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826576B0 size=108
    let mut pc: u32 = 0x826576B0;
    'dispatch: loop {
        match pc {
            0x826576B0 => {
    //   block [0x826576B0..0x8265771C)
	// 826576B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826576B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826576B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826576BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826576C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826576C4: 38EBBDC8  addi r7, r11, -0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + -16952;
	// 826576C8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826576CC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826576D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826576D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826576D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826576DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826576E0: 386A71F8  addi r3, r10, 0x71f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29176;
	// 826576E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826576E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826576EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826576F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826576F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826576F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826576FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657708: 4BE0F719  bl 0x82466e20
	ctx.lr = 0x8265770C;
	sub_82466E20(ctx, base);
	// 8265770C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657720 size=108
    let mut pc: u32 = 0x82657720;
    'dispatch: loop {
        match pc {
            0x82657720 => {
    //   block [0x82657720..0x8265778C)
	// 82657720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265772C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657734: 38EBBE58  addi r7, r11, -0x41a8
	ctx.r[7].s64 = ctx.r[11].s64 + -16808;
	// 82657738: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8265773C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82657740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265774C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657750: 386A7228  addi r3, r10, 0x7228
	ctx.r[3].s64 = ctx.r[10].s64 + 29224;
	// 82657754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265775C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265776C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657778: 4BE0F6A9  bl 0x82466e20
	ctx.lr = 0x8265777C;
	sub_82466E20(ctx, base);
	// 8265777C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657790 size=108
    let mut pc: u32 = 0x82657790;
    'dispatch: loop {
        match pc {
            0x82657790 => {
    //   block [0x82657790..0x826577FC)
	// 82657790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265779C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826577A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826577A4: 38EBBF18  addi r7, r11, -0x40e8
	ctx.r[7].s64 = ctx.r[11].s64 + -16616;
	// 826577A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826577AC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826577B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826577B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826577B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826577BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826577C0: 386A7258  addi r3, r10, 0x7258
	ctx.r[3].s64 = ctx.r[10].s64 + 29272;
	// 826577C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826577C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826577CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826577D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826577D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826577D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826577DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826577E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826577E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826577E8: 4BE0F639  bl 0x82466e20
	ctx.lr = 0x826577EC;
	sub_82466E20(ctx, base);
	// 826577EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826577F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826577F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826577F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657800 size=108
    let mut pc: u32 = 0x82657800;
    'dispatch: loop {
        match pc {
            0x82657800 => {
    //   block [0x82657800..0x8265786C)
	// 82657800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265780C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657814: 38EBBFF0  addi r7, r11, -0x4010
	ctx.r[7].s64 = ctx.r[11].s64 + -16400;
	// 82657818: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8265781C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82657820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265782C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657830: 386A7288  addi r3, r10, 0x7288
	ctx.r[3].s64 = ctx.r[10].s64 + 29320;
	// 82657834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265783C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265784C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657858: 4BE0F5C9  bl 0x82466e20
	ctx.lr = 0x8265785C;
	sub_82466E20(ctx, base);
	// 8265785C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657870 size=108
    let mut pc: u32 = 0x82657870;
    'dispatch: loop {
        match pc {
            0x82657870 => {
    //   block [0x82657870..0x826578DC)
	// 82657870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265787C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657884: 38EBC0B0  addi r7, r11, -0x3f50
	ctx.r[7].s64 = ctx.r[11].s64 + -16208;
	// 82657888: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8265788C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82657890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265789C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826578A0: 386A72B8  addi r3, r10, 0x72b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29368;
	// 826578A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826578A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826578AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826578B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826578B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826578B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826578BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826578C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826578C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826578C8: 4BE0F559  bl 0x82466e20
	ctx.lr = 0x826578CC;
	sub_82466E20(ctx, base);
	// 826578CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826578D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826578D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826578D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826578E0 size=112
    let mut pc: u32 = 0x826578E0;
    'dispatch: loop {
        match pc {
            0x826578E0 => {
    //   block [0x826578E0..0x82657950)
	// 826578E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826578E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826578E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826578EC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826578F0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826578F4: 38EAC158  addi r7, r10, -0x3ea8
	ctx.r[7].s64 = ctx.r[10].s64 + -16040;
	// 826578F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826578FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82657900: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82657904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657908: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265790C: 396BD1E0  addi r11, r11, -0x2e20
	ctx.r[11].s64 = ctx.r[11].s64 + -11808;
	// 82657910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657918: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265791C: 386A72E8  addi r3, r10, 0x72e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29416;
	// 82657920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657924: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82657928: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265792C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82657930: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657934: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657938: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265793C: 4BE0F4E5  bl 0x82466e20
	ctx.lr = 0x82657940;
	sub_82466E20(ctx, base);
	// 82657940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265794C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657950 size=108
    let mut pc: u32 = 0x82657950;
    'dispatch: loop {
        match pc {
            0x82657950 => {
    //   block [0x82657950..0x826579BC)
	// 82657950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265795C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657964: 38EBC278  addi r7, r11, -0x3d88
	ctx.r[7].s64 = ctx.r[11].s64 + -15752;
	// 82657968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265796C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82657970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265797C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657980: 386A7318  addi r3, r10, 0x7318
	ctx.r[3].s64 = ctx.r[10].s64 + 29464;
	// 82657984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265798C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265799C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826579A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826579A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826579A8: 4BE0F479  bl 0x82466e20
	ctx.lr = 0x826579AC;
	sub_82466E20(ctx, base);
	// 826579AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826579B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826579B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826579B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826579C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826579C0 size=108
    let mut pc: u32 = 0x826579C0;
    'dispatch: loop {
        match pc {
            0x826579C0 => {
    //   block [0x826579C0..0x82657A2C)
	// 826579C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826579C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826579C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826579CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826579D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826579D4: 38EBC2D8  addi r7, r11, -0x3d28
	ctx.r[7].s64 = ctx.r[11].s64 + -15656;
	// 826579D8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826579DC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826579E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826579E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826579E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826579EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826579F0: 386A7348  addi r3, r10, 0x7348
	ctx.r[3].s64 = ctx.r[10].s64 + 29512;
	// 826579F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826579F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826579FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657A18: 4BE0F409  bl 0x82466e20
	ctx.lr = 0x82657A1C;
	sub_82466E20(ctx, base);
	// 82657A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657A30 size=108
    let mut pc: u32 = 0x82657A30;
    'dispatch: loop {
        match pc {
            0x82657A30 => {
    //   block [0x82657A30..0x82657A9C)
	// 82657A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657A3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657A44: 38EBC3E0  addi r7, r11, -0x3c20
	ctx.r[7].s64 = ctx.r[11].s64 + -15392;
	// 82657A48: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82657A4C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82657A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657A54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657A60: 386A7378  addi r3, r10, 0x7378
	ctx.r[3].s64 = ctx.r[10].s64 + 29560;
	// 82657A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657A88: 4BE0F399  bl 0x82466e20
	ctx.lr = 0x82657A8C;
	sub_82466E20(ctx, base);
	// 82657A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657AA0 size=108
    let mut pc: u32 = 0x82657AA0;
    'dispatch: loop {
        match pc {
            0x82657AA0 => {
    //   block [0x82657AA0..0x82657B0C)
	// 82657AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657AAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657AB4: 38EBC4B8  addi r7, r11, -0x3b48
	ctx.r[7].s64 = ctx.r[11].s64 + -15176;
	// 82657AB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657ABC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82657AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657AC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657AC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657AD0: 386A73A8  addi r3, r10, 0x73a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29608;
	// 82657AD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657AF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657AF8: 4BE0F329  bl 0x82466e20
	ctx.lr = 0x82657AFC;
	sub_82466E20(ctx, base);
	// 82657AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657B10 size=108
    let mut pc: u32 = 0x82657B10;
    'dispatch: loop {
        match pc {
            0x82657B10 => {
    //   block [0x82657B10..0x82657B7C)
	// 82657B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657B1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657B24: 38EBC500  addi r7, r11, -0x3b00
	ctx.r[7].s64 = ctx.r[11].s64 + -15104;
	// 82657B28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657B2C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82657B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657B34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657B40: 386A73D8  addi r3, r10, 0x73d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29656;
	// 82657B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657B68: 4BE0F2B9  bl 0x82466e20
	ctx.lr = 0x82657B6C;
	sub_82466E20(ctx, base);
	// 82657B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657B80 size=112
    let mut pc: u32 = 0x82657B80;
    'dispatch: loop {
        match pc {
            0x82657B80 => {
    //   block [0x82657B80..0x82657BF0)
	// 82657B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657B90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657B94: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82657B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657B9C: 390BC518  addi r8, r11, -0x3ae8
	ctx.r[8].s64 = ctx.r[11].s64 + -15080;
	// 82657BA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82657BA4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82657BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657BB8: 386A7408  addi r3, r10, 0x7408
	ctx.r[3].s64 = ctx.r[10].s64 + 29704;
	// 82657BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657BDC: 4BE0F245  bl 0x82466e20
	ctx.lr = 0x82657BE0;
	sub_82466E20(ctx, base);
	// 82657BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657BF0 size=112
    let mut pc: u32 = 0x82657BF0;
    'dispatch: loop {
        match pc {
            0x82657BF0 => {
    //   block [0x82657BF0..0x82657C60)
	// 82657BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657C04: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82657C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657C0C: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 82657C10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82657C14: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82657C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657C1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657C28: 386A7438  addi r3, r10, 0x7438
	ctx.r[3].s64 = ctx.r[10].s64 + 29752;
	// 82657C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657C4C: 4BE0F1D5  bl 0x82466e20
	ctx.lr = 0x82657C50;
	sub_82466E20(ctx, base);
	// 82657C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657C60 size=108
    let mut pc: u32 = 0x82657C60;
    'dispatch: loop {
        match pc {
            0x82657C60 => {
    //   block [0x82657C60..0x82657CCC)
	// 82657C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657C6C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657C74: 38EBC5C0  addi r7, r11, -0x3a40
	ctx.r[7].s64 = ctx.r[11].s64 + -14912;
	// 82657C78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657C7C: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 82657C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657C84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657C90: 386A7468  addi r3, r10, 0x7468
	ctx.r[3].s64 = ctx.r[10].s64 + 29800;
	// 82657C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657CB8: 4BE0F169  bl 0x82466e20
	ctx.lr = 0x82657CBC;
	sub_82466E20(ctx, base);
	// 82657CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82657CD0 size=24
    let mut pc: u32 = 0x82657CD0;
    'dispatch: loop {
        match pc {
            0x82657CD0 => {
    //   block [0x82657CD0..0x82657CE8)
	// 82657CD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657CD4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82657CD8: 394A1B78  addi r10, r10, 0x1b78
	ctx.r[10].s64 = ctx.r[10].s64 + 7032;
	// 82657CDC: 816BBB84  lwz r11, -0x447c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17532 as u32) ) } as u64;
	// 82657CE0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82657CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657CE8 size=112
    let mut pc: u32 = 0x82657CE8;
    'dispatch: loop {
        match pc {
            0x82657CE8 => {
    //   block [0x82657CE8..0x82657D58)
	// 82657CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657CF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657CF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657CFC: 38AA7678  addi r5, r10, 0x7678
	ctx.r[5].s64 = ctx.r[10].s64 + 30328;
	// 82657D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657D04: 390B1B78  addi r8, r11, 0x1b78
	ctx.r[8].s64 = ctx.r[11].s64 + 7032;
	// 82657D08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82657D0C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82657D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657D14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657D20: 386A7498  addi r3, r10, 0x7498
	ctx.r[3].s64 = ctx.r[10].s64 + 29848;
	// 82657D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657D44: 4BE0F0DD  bl 0x82466e20
	ctx.lr = 0x82657D48;
	sub_82466E20(ctx, base);
	// 82657D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657D58 size=108
    let mut pc: u32 = 0x82657D58;
    'dispatch: loop {
        match pc {
            0x82657D58 => {
    //   block [0x82657D58..0x82657DC4)
	// 82657D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657D64: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657D6C: 38EBC5D8  addi r7, r11, -0x3a28
	ctx.r[7].s64 = ctx.r[11].s64 + -14888;
	// 82657D70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82657D74: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 82657D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657D88: 386A74C8  addi r3, r10, 0x74c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29896;
	// 82657D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657DB0: 4BE0F071  bl 0x82466e20
	ctx.lr = 0x82657DB4;
	sub_82466E20(ctx, base);
	// 82657DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657DC8 size=112
    let mut pc: u32 = 0x82657DC8;
    'dispatch: loop {
        match pc {
            0x82657DC8 => {
    //   block [0x82657DC8..0x82657E38)
	// 82657DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657DD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657DD8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657DDC: 38AA7678  addi r5, r10, 0x7678
	ctx.r[5].s64 = ctx.r[10].s64 + 30328;
	// 82657DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657DE4: 390BC638  addi r8, r11, -0x39c8
	ctx.r[8].s64 = ctx.r[11].s64 + -14792;
	// 82657DE8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82657DEC: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82657DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657E00: 386A74F8  addi r3, r10, 0x74f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29944;
	// 82657E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657E24: 4BE0EFFD  bl 0x82466e20
	ctx.lr = 0x82657E28;
	sub_82466E20(ctx, base);
	// 82657E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657E38 size=108
    let mut pc: u32 = 0x82657E38;
    'dispatch: loop {
        match pc {
            0x82657E38 => {
    //   block [0x82657E38..0x82657EA4)
	// 82657E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657E44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657E4C: 38EBC6F8  addi r7, r11, -0x3908
	ctx.r[7].s64 = ctx.r[11].s64 + -14600;
	// 82657E50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82657E54: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 82657E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657E68: 386A7528  addi r3, r10, 0x7528
	ctx.r[3].s64 = ctx.r[10].s64 + 29992;
	// 82657E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657E90: 4BE0EF91  bl 0x82466e20
	ctx.lr = 0x82657E94;
	sub_82466E20(ctx, base);
	// 82657E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657EA8 size=108
    let mut pc: u32 = 0x82657EA8;
    'dispatch: loop {
        match pc {
            0x82657EA8 => {
    //   block [0x82657EA8..0x82657F14)
	// 82657EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657EB4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657EBC: 38EBC770  addi r7, r11, -0x3890
	ctx.r[7].s64 = ctx.r[11].s64 + -14480;
	// 82657EC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657EC4: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82657EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657ED8: 386A7558  addi r3, r10, 0x7558
	ctx.r[3].s64 = ctx.r[10].s64 + 30040;
	// 82657EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657F00: 4BE0EF21  bl 0x82466e20
	ctx.lr = 0x82657F04;
	sub_82466E20(ctx, base);
	// 82657F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657F18 size=108
    let mut pc: u32 = 0x82657F18;
    'dispatch: loop {
        match pc {
            0x82657F18 => {
    //   block [0x82657F18..0x82657F84)
	// 82657F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657F24: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657F2C: 38EBC7B8  addi r7, r11, -0x3848
	ctx.r[7].s64 = ctx.r[11].s64 + -14408;
	// 82657F30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657F34: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82657F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657F48: 386A7588  addi r3, r10, 0x7588
	ctx.r[3].s64 = ctx.r[10].s64 + 30088;
	// 82657F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657F70: 4BE0EEB1  bl 0x82466e20
	ctx.lr = 0x82657F74;
	sub_82466E20(ctx, base);
	// 82657F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657F88 size=112
    let mut pc: u32 = 0x82657F88;
    'dispatch: loop {
        match pc {
            0x82657F88 => {
    //   block [0x82657F88..0x82657FF8)
	// 82657F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657F94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657F98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657F9C: 38AA7588  addi r5, r10, 0x7588
	ctx.r[5].s64 = ctx.r[10].s64 + 30088;
	// 82657FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657FA4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 82657FA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82657FAC: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82657FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657FB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657FC0: 386A75B8  addi r3, r10, 0x75b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30136;
	// 82657FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657FE4: 4BE0EE3D  bl 0x82466e20
	ctx.lr = 0x82657FE8;
	sub_82466E20(ctx, base);
	// 82657FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657FF8 size=108
    let mut pc: u32 = 0x82657FF8;
    'dispatch: loop {
        match pc {
            0x82657FF8 => {
    //   block [0x82657FF8..0x82658064)
	// 82657FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658004: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265800C: 38EBC878  addi r7, r11, -0x3788
	ctx.r[7].s64 = ctx.r[11].s64 + -14216;
	// 82658010: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82658014: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82658018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265801C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658028: 386A75E8  addi r3, r10, 0x75e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30184;
	// 8265802C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265803C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265804C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658050: 4BE0EDD1  bl 0x82466e20
	ctx.lr = 0x82658054;
	sub_82466E20(ctx, base);
	// 82658054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265805C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658068 size=108
    let mut pc: u32 = 0x82658068;
    'dispatch: loop {
        match pc {
            0x82658068 => {
    //   block [0x82658068..0x826580D4)
	// 82658068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265806C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658074: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265807C: 38EBC8C0  addi r7, r11, -0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + -14144;
	// 82658080: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82658084: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 82658088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265808C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658098: 386A7618  addi r3, r10, 0x7618
	ctx.r[3].s64 = ctx.r[10].s64 + 30232;
	// 8265809C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826580A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826580A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826580A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826580AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826580B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826580B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826580B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826580BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826580C0: 4BE0ED61  bl 0x82466e20
	ctx.lr = 0x826580C4;
	sub_82466E20(ctx, base);
	// 826580C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826580C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826580CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826580D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826580D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826580D8 size=108
    let mut pc: u32 = 0x826580D8;
    'dispatch: loop {
        match pc {
            0x826580D8 => {
    //   block [0x826580D8..0x82658144)
	// 826580D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826580DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826580E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826580E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826580E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826580EC: 38EBC980  addi r7, r11, -0x3680
	ctx.r[7].s64 = ctx.r[11].s64 + -13952;
	// 826580F0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 826580F4: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 826580F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826580FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658108: 386A7648  addi r3, r10, 0x7648
	ctx.r[3].s64 = ctx.r[10].s64 + 30280;
	// 8265810C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265811C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265812C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658130: 4BE0ECF1  bl 0x82466e20
	ctx.lr = 0x82658134;
	sub_82466E20(ctx, base);
	// 82658134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265813C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658148 size=112
    let mut pc: u32 = 0x82658148;
    'dispatch: loop {
        match pc {
            0x82658148 => {
    //   block [0x82658148..0x826581B8)
	// 82658148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265814C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658154: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265815C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658164: 390BCB00  addi r8, r11, -0x3500
	ctx.r[8].s64 = ctx.r[11].s64 + -13568;
	// 82658168: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265816C: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 82658170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265817C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658180: 386A7678  addi r3, r10, 0x7678
	ctx.r[3].s64 = ctx.r[10].s64 + 30328;
	// 82658184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265818C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265819C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826581A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826581A4: 4BE0EC7D  bl 0x82466e20
	ctx.lr = 0x826581A8;
	sub_82466E20(ctx, base);
	// 826581A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826581AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826581B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826581B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826581B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826581B8 size=108
    let mut pc: u32 = 0x826581B8;
    'dispatch: loop {
        match pc {
            0x826581B8 => {
    //   block [0x826581B8..0x82658224)
	// 826581B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826581BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826581C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826581C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826581C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826581CC: 38EBCB60  addi r7, r11, -0x34a0
	ctx.r[7].s64 = ctx.r[11].s64 + -13472;
	// 826581D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826581D4: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 826581D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826581DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826581E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826581E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826581E8: 386A76A8  addi r3, r10, 0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30376;
	// 826581EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826581F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826581F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826581F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826581FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265820C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658210: 4BE0EC11  bl 0x82466e20
	ctx.lr = 0x82658214;
	sub_82466E20(ctx, base);
	// 82658214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265821C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658228 size=112
    let mut pc: u32 = 0x82658228;
    'dispatch: loop {
        match pc {
            0x82658228 => {
    //   block [0x82658228..0x82658298)
	// 82658228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658234: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658238: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265823C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658244: 390BCBD8  addi r8, r11, -0x3428
	ctx.r[8].s64 = ctx.r[11].s64 + -13352;
	// 82658248: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265824C: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 82658250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265825C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658260: 386A76D8  addi r3, r10, 0x76d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30424;
	// 82658264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265826C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265827C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658284: 4BE0EB9D  bl 0x82466e20
	ctx.lr = 0x82658288;
	sub_82466E20(ctx, base);
	// 82658288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658298 size=108
    let mut pc: u32 = 0x82658298;
    'dispatch: loop {
        match pc {
            0x82658298 => {
    //   block [0x82658298..0x82658304)
	// 82658298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826582A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826582A4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826582A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826582AC: 38EBCC20  addi r7, r11, -0x33e0
	ctx.r[7].s64 = ctx.r[11].s64 + -13280;
	// 826582B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826582B4: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 826582B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826582BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826582C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826582C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826582C8: 386A7708  addi r3, r10, 0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + 30472;
	// 826582CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826582D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826582D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826582D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826582DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826582E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826582E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826582E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826582EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826582F0: 4BE0EB31  bl 0x82466e20
	ctx.lr = 0x826582F4;
	sub_82466E20(ctx, base);
	// 826582F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826582F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826582FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658308 size=108
    let mut pc: u32 = 0x82658308;
    'dispatch: loop {
        match pc {
            0x82658308 => {
    //   block [0x82658308..0x82658374)
	// 82658308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658314: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658318: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265831C: 38EBCC80  addi r7, r11, -0x3380
	ctx.r[7].s64 = ctx.r[11].s64 + -13184;
	// 82658320: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82658324: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82658328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265832C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658338: 386A7738  addi r3, r10, 0x7738
	ctx.r[3].s64 = ctx.r[10].s64 + 30520;
	// 8265833C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265834C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265835C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658360: 4BE0EAC1  bl 0x82466e20
	ctx.lr = 0x82658364;
	sub_82466E20(ctx, base);
	// 82658364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265836C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658378 size=108
    let mut pc: u32 = 0x82658378;
    'dispatch: loop {
        match pc {
            0x82658378 => {
    //   block [0x82658378..0x826583E4)
	// 82658378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658384: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265838C: 38EBCCC8  addi r7, r11, -0x3338
	ctx.r[7].s64 = ctx.r[11].s64 + -13112;
	// 82658390: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82658394: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 82658398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265839C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826583A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826583A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826583A8: 386A7768  addi r3, r10, 0x7768
	ctx.r[3].s64 = ctx.r[10].s64 + 30568;
	// 826583AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826583B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826583B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826583B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826583BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826583C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826583C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826583C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826583CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826583D0: 4BE0EA51  bl 0x82466e20
	ctx.lr = 0x826583D4;
	sub_82466E20(ctx, base);
	// 826583D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826583D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826583DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826583E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826583E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826583E8 size=108
    let mut pc: u32 = 0x826583E8;
    'dispatch: loop {
        match pc {
            0x826583E8 => {
    //   block [0x826583E8..0x82658454)
	// 826583E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826583EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826583F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826583F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826583F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826583FC: 38EBCD88  addi r7, r11, -0x3278
	ctx.r[7].s64 = ctx.r[11].s64 + -12920;
	// 82658400: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82658404: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82658408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265840C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658418: 386A7798  addi r3, r10, 0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + 30616;
	// 8265841C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265842C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265843C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658440: 4BE0E9E1  bl 0x82466e20
	ctx.lr = 0x82658444;
	sub_82466E20(ctx, base);
	// 82658444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265844C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658458 size=108
    let mut pc: u32 = 0x82658458;
    'dispatch: loop {
        match pc {
            0x82658458 => {
    //   block [0x82658458..0x826584C4)
	// 82658458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265845C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658464: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265846C: 38EBCE18  addi r7, r11, -0x31e8
	ctx.r[7].s64 = ctx.r[11].s64 + -12776;
	// 82658470: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82658474: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 82658478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265847C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658488: 386A77C8  addi r3, r10, 0x77c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30664;
	// 8265848C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265849C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826584A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826584A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826584A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826584AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826584B0: 4BE0E971  bl 0x82466e20
	ctx.lr = 0x826584B4;
	sub_82466E20(ctx, base);
	// 826584B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826584B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826584BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826584C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826584C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826584C8 size=108
    let mut pc: u32 = 0x826584C8;
    'dispatch: loop {
        match pc {
            0x826584C8 => {
    //   block [0x826584C8..0x82658534)
	// 826584C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826584CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826584D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826584D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826584D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826584DC: 38EBCF50  addi r7, r11, -0x30b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12464;
	// 826584E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826584E4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826584E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826584EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826584F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826584F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826584F8: 386A77F8  addi r3, r10, 0x77f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30712;
	// 826584FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265850C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265851C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658520: 4BE0E901  bl 0x82466e20
	ctx.lr = 0x82658524;
	sub_82466E20(ctx, base);
	// 82658524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265852C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658538 size=116
    let mut pc: u32 = 0x82658538;
    'dispatch: loop {
        match pc {
            0x82658538 => {
    //   block [0x82658538..0x826585AC)
	// 82658538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658544: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658548: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265854C: 390BCFB8  addi r8, r11, -0x3048
	ctx.r[8].s64 = ctx.r[11].s64 + -12360;
	// 82658550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658554: 392AD294  addi r9, r10, -0x2d6c
	ctx.r[9].s64 = ctx.r[10].s64 + -11628;
	// 82658558: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265855C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82658560: 38AA77F8  addi r5, r10, 0x77f8
	ctx.r[5].s64 = ctx.r[10].s64 + 30712;
	// 82658564: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265856C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265857C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658580: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82658584: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658588: 386B7828  addi r3, r11, 0x7828
	ctx.r[3].s64 = ctx.r[11].s64 + 30760;
	// 8265858C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658590: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658598: 4BE0E889  bl 0x82466e20
	ctx.lr = 0x8265859C;
	sub_82466E20(ctx, base);
	// 8265859C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826585A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826585A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826585A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826585B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826585B0 size=96
    let mut pc: u32 = 0x826585B0;
    'dispatch: loop {
        match pc {
            0x826585B0 => {
    //   block [0x826585B0..0x82658610)
	// 826585B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826585B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826585B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826585BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826585C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826585C4: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826585C8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826585CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826585D0: 386A7858  addi r3, r10, 0x7858
	ctx.r[3].s64 = ctx.r[10].s64 + 30808;
	// 826585D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826585D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826585DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826585E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826585E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826585E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826585EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826585F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826585F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826585F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826585FC: 4BE0E825  bl 0x82466e20
	ctx.lr = 0x82658600;
	sub_82466E20(ctx, base);
	// 82658600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658610 size=112
    let mut pc: u32 = 0x82658610;
    'dispatch: loop {
        match pc {
            0x82658610 => {
    //   block [0x82658610..0x82658680)
	// 82658610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265861C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82658620: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658624: 38AA9928  addi r5, r10, -0x66d8
	ctx.r[5].s64 = ctx.r[10].s64 + -26328;
	// 82658628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265862C: 390BD030  addi r8, r11, -0x2fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -12240;
	// 82658630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82658634: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82658638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265863C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658648: 386A7888  addi r3, r10, 0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + 30856;
	// 8265864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265866C: 4BE0E7B5  bl 0x82466e20
	ctx.lr = 0x82658670;
	sub_82466E20(ctx, base);
	// 82658670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658680 size=96
    let mut pc: u32 = 0x82658680;
    'dispatch: loop {
        match pc {
            0x82658680 => {
    //   block [0x82658680..0x826586E0)
	// 82658680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265868C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658694: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82658698: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265869C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826586A0: 386A78B8  addi r3, r10, 0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30904;
	// 826586A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826586A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826586AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826586B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826586B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826586B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826586BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826586C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826586C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826586C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826586CC: 4BE0E755  bl 0x82466e20
	ctx.lr = 0x826586D0;
	sub_82466E20(ctx, base);
	// 826586D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826586D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826586D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826586DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826586E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826586E0 size=24
    let mut pc: u32 = 0x826586E0;
    'dispatch: loop {
        match pc {
            0x826586E0 => {
    //   block [0x826586E0..0x826586F8)
	// 826586E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826586E4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826586E8: 394A1C38  addi r10, r10, 0x1c38
	ctx.r[10].s64 = ctx.r[10].s64 + 7224;
	// 826586EC: 816BCFB4  lwz r11, -0x304c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12364 as u32) ) } as u64;
	// 826586F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826586F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826586F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826586F8 size=116
    let mut pc: u32 = 0x826586F8;
    'dispatch: loop {
        match pc {
            0x826586F8 => {
    //   block [0x826586F8..0x8265876C)
	// 826586F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826586FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658704: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658708: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265870C: 390B1C38  addi r8, r11, 0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + 7224;
	// 82658710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658714: 392AD2D0  addi r9, r10, -0x2d30
	ctx.r[9].s64 = ctx.r[10].s64 + -11568;
	// 82658718: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265871C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82658720: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658724: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265872C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658734: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82658738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265873C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658740: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82658744: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658748: 386B78E8  addi r3, r11, 0x78e8
	ctx.r[3].s64 = ctx.r[11].s64 + 30952;
	// 8265874C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658750: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658758: 4BE0E6C9  bl 0x82466e20
	ctx.lr = 0x8265875C;
	sub_82466E20(ctx, base);
	// 8265875C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658770 size=104
    let mut pc: u32 = 0x82658770;
    'dispatch: loop {
        match pc {
            0x82658770 => {
    //   block [0x82658770..0x826587D8)
	// 82658770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265877C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658784: 392AD2FC  addi r9, r10, -0x2d04
	ctx.r[9].s64 = ctx.r[10].s64 + -11524;
	// 82658788: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265878C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658790: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658794: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265879C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826587A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826587A4: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 826587A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826587AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826587B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826587B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826587B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826587BC: 386A7918  addi r3, r10, 0x7918
	ctx.r[3].s64 = ctx.r[10].s64 + 31000;
	// 826587C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826587C4: 4BE0E65D  bl 0x82466e20
	ctx.lr = 0x826587C8;
	sub_82466E20(ctx, base);
	// 826587C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826587CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826587D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826587D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826587D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826587D8 size=96
    let mut pc: u32 = 0x826587D8;
    'dispatch: loop {
        match pc {
            0x826587D8 => {
    //   block [0x826587D8..0x82658838)
	// 826587D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826587DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826587E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826587E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826587E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826587EC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826587F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826587F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826587F8: 386A7948  addi r3, r10, 0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + 31048;
	// 826587FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658804: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658818: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265881C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658824: 4BE0E5FD  bl 0x82466e20
	ctx.lr = 0x82658828;
	sub_82466E20(ctx, base);
	// 82658828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265882C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658838 size=100
    let mut pc: u32 = 0x82658838;
    'dispatch: loop {
        match pc {
            0x82658838 => {
    //   block [0x82658838..0x8265889C)
	// 82658838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265884C: 38AA7918  addi r5, r10, 0x7918
	ctx.r[5].s64 = ctx.r[10].s64 + 31000;
	// 82658850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658858: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8265885C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265886C: 386A7978  addi r3, r10, 0x7978
	ctx.r[3].s64 = ctx.r[10].s64 + 31096;
	// 82658870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265887C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658888: 4BE0E599  bl 0x82466e20
	ctx.lr = 0x8265888C;
	sub_82466E20(ctx, base);
	// 8265888C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826588A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826588A0 size=112
    let mut pc: u32 = 0x826588A0;
    'dispatch: loop {
        match pc {
            0x826588A0 => {
    //   block [0x826588A0..0x82658910)
	// 826588A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826588A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826588A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826588AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826588B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826588B4: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 826588B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826588BC: 390BD098  addi r8, r11, -0x2f68
	ctx.r[8].s64 = ctx.r[11].s64 + -12136;
	// 826588C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826588C4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826588C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826588CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826588D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826588D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826588D8: 386A79A8  addi r3, r10, 0x79a8
	ctx.r[3].s64 = ctx.r[10].s64 + 31144;
	// 826588DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826588E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826588E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826588E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826588EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826588F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826588F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826588F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826588FC: 4BE0E525  bl 0x82466e20
	ctx.lr = 0x82658900;
	sub_82466E20(ctx, base);
	// 82658900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265890C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658910 size=112
    let mut pc: u32 = 0x82658910;
    'dispatch: loop {
        match pc {
            0x82658910 => {
    //   block [0x82658910..0x82658980)
	// 82658910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265891C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658920: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658924: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 82658928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265892C: 390BD0E0  addi r8, r11, -0x2f20
	ctx.r[8].s64 = ctx.r[11].s64 + -12064;
	// 82658930: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658934: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82658938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265893C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658948: 386A79D8  addi r3, r10, 0x79d8
	ctx.r[3].s64 = ctx.r[10].s64 + 31192;
	// 8265894C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265895C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265896C: 4BE0E4B5  bl 0x82466e20
	ctx.lr = 0x82658970;
	sub_82466E20(ctx, base);
	// 82658970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265897C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658980 size=100
    let mut pc: u32 = 0x82658980;
    'dispatch: loop {
        match pc {
            0x82658980 => {
    //   block [0x82658980..0x826589E4)
	// 82658980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265898C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658994: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 82658998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265899C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826589A0: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826589A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826589A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826589AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826589B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826589B4: 386A7A08  addi r3, r10, 0x7a08
	ctx.r[3].s64 = ctx.r[10].s64 + 31240;
	// 826589B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826589BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826589C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826589C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826589C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826589CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826589D0: 4BE0E451  bl 0x82466e20
	ctx.lr = 0x826589D4;
	sub_82466E20(ctx, base);
	// 826589D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826589D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826589DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826589E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826589E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826589E8 size=96
    let mut pc: u32 = 0x826589E8;
    'dispatch: loop {
        match pc {
            0x826589E8 => {
    //   block [0x826589E8..0x82658A48)
	// 826589E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826589EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826589F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826589F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826589F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826589FC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82658A00: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658A08: 386A7A38  addi r3, r10, 0x7a38
	ctx.r[3].s64 = ctx.r[10].s64 + 31288;
	// 82658A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658A14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658A28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658A30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658A34: 4BE0E3ED  bl 0x82466e20
	ctx.lr = 0x82658A38;
	sub_82466E20(ctx, base);
	// 82658A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658A48 size=112
    let mut pc: u32 = 0x82658A48;
    'dispatch: loop {
        match pc {
            0x82658A48 => {
    //   block [0x82658A48..0x82658AB8)
	// 82658A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658A54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658A5C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658A64: 390BD0F8  addi r8, r11, -0x2f08
	ctx.r[8].s64 = ctx.r[11].s64 + -12040;
	// 82658A68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82658A6C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82658A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658A74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658A80: 386A7A68  addi r3, r10, 0x7a68
	ctx.r[3].s64 = ctx.r[10].s64 + 31336;
	// 82658A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658AA4: 4BE0E37D  bl 0x82466e20
	ctx.lr = 0x82658AA8;
	sub_82466E20(ctx, base);
	// 82658AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658AB8 size=96
    let mut pc: u32 = 0x82658AB8;
    'dispatch: loop {
        match pc {
            0x82658AB8 => {
    //   block [0x82658AB8..0x82658B18)
	// 82658AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658AC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658ACC: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82658AD0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658AD8: 386A7A98  addi r3, r10, 0x7a98
	ctx.r[3].s64 = ctx.r[10].s64 + 31384;
	// 82658ADC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658AE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658B04: 4BE0E31D  bl 0x82466e20
	ctx.lr = 0x82658B08;
	sub_82466E20(ctx, base);
	// 82658B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658B18 size=112
    let mut pc: u32 = 0x82658B18;
    'dispatch: loop {
        match pc {
            0x82658B18 => {
    //   block [0x82658B18..0x82658B88)
	// 82658B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658B28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658B2C: 38AA7A98  addi r5, r10, 0x7a98
	ctx.r[5].s64 = ctx.r[10].s64 + 31384;
	// 82658B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658B34: 390BD128  addi r8, r11, -0x2ed8
	ctx.r[8].s64 = ctx.r[11].s64 + -11992;
	// 82658B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658B3C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 82658B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658B50: 386A7AC8  addi r3, r10, 0x7ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 31432;
	// 82658B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658B74: 4BE0E2AD  bl 0x82466e20
	ctx.lr = 0x82658B78;
	sub_82466E20(ctx, base);
	// 82658B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658B88 size=108
    let mut pc: u32 = 0x82658B88;
    'dispatch: loop {
        match pc {
            0x82658B88 => {
    //   block [0x82658B88..0x82658BF4)
	// 82658B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658B94: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658B9C: 38EBD140  addi r7, r11, -0x2ec0
	ctx.r[7].s64 = ctx.r[11].s64 + -11968;
	// 82658BA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82658BA4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82658BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658BB8: 386A7AF8  addi r3, r10, 0x7af8
	ctx.r[3].s64 = ctx.r[10].s64 + 31480;
	// 82658BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658BE0: 4BE0E241  bl 0x82466e20
	ctx.lr = 0x82658BE4;
	sub_82466E20(ctx, base);
	// 82658BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658BF8 size=112
    let mut pc: u32 = 0x82658BF8;
    'dispatch: loop {
        match pc {
            0x82658BF8 => {
    //   block [0x82658BF8..0x82658C68)
	// 82658BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C08: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658C0C: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658C14: 390BD1A0  addi r8, r11, -0x2e60
	ctx.r[8].s64 = ctx.r[11].s64 + -11872;
	// 82658C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658C1C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82658C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658C30: 386A7B28  addi r3, r10, 0x7b28
	ctx.r[3].s64 = ctx.r[10].s64 + 31528;
	// 82658C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658C54: 4BE0E1CD  bl 0x82466e20
	ctx.lr = 0x82658C58;
	sub_82466E20(ctx, base);
	// 82658C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658C68 size=112
    let mut pc: u32 = 0x82658C68;
    'dispatch: loop {
        match pc {
            0x82658C68 => {
    //   block [0x82658C68..0x82658CD8)
	// 82658C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658C7C: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658C84: 390BD1B8  addi r8, r11, -0x2e48
	ctx.r[8].s64 = ctx.r[11].s64 + -11848;
	// 82658C88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82658C8C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82658C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658CA0: 386A7B58  addi r3, r10, 0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + 31576;
	// 82658CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658CC4: 4BE0E15D  bl 0x82466e20
	ctx.lr = 0x82658CC8;
	sub_82466E20(ctx, base);
	// 82658CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658CD8 size=100
    let mut pc: u32 = 0x82658CD8;
    'dispatch: loop {
        match pc {
            0x82658CD8 => {
    //   block [0x82658CD8..0x82658D3C)
	// 82658CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658CEC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658CF8: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82658CFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658D0C: 386A7B88  addi r3, r10, 0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + 31624;
	// 82658D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658D28: 4BE0E0F9  bl 0x82466e20
	ctx.lr = 0x82658D2C;
	sub_82466E20(ctx, base);
	// 82658D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658D40 size=116
    let mut pc: u32 = 0x82658D40;
    'dispatch: loop {
        match pc {
            0x82658D40 => {
    //   block [0x82658D40..0x82658DB4)
	// 82658D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658D4C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658D50: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658D54: 390BD1E8  addi r8, r11, -0x2e18
	ctx.r[8].s64 = ctx.r[11].s64 + -11800;
	// 82658D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658D5C: 392AD328  addi r9, r10, -0x2cd8
	ctx.r[9].s64 = ctx.r[10].s64 + -11480;
	// 82658D60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658D64: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82658D68: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658D6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658D84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658D88: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82658D8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658D90: 386B7BB8  addi r3, r11, 0x7bb8
	ctx.r[3].s64 = ctx.r[11].s64 + 31672;
	// 82658D94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658D98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658DA0: 4BE0E081  bl 0x82466e20
	ctx.lr = 0x82658DA4;
	sub_82466E20(ctx, base);
	// 82658DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658DB8 size=112
    let mut pc: u32 = 0x82658DB8;
    'dispatch: loop {
        match pc {
            0x82658DB8 => {
    //   block [0x82658DB8..0x82658E28)
	// 82658DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658DC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658DCC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658DD4: 390BD218  addi r8, r11, -0x2de8
	ctx.r[8].s64 = ctx.r[11].s64 + -11752;
	// 82658DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658DDC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82658DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658DF0: 386A7BE8  addi r3, r10, 0x7be8
	ctx.r[3].s64 = ctx.r[10].s64 + 31720;
	// 82658DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658E04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658E14: 4BE0E00D  bl 0x82466e20
	ctx.lr = 0x82658E18;
	sub_82466E20(ctx, base);
	// 82658E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658E28 size=116
    let mut pc: u32 = 0x82658E28;
    'dispatch: loop {
        match pc {
            0x82658E28 => {
    //   block [0x82658E28..0x82658E9C)
	// 82658E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658E34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658E38: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658E3C: 390BD234  addi r8, r11, -0x2dcc
	ctx.r[8].s64 = ctx.r[11].s64 + -11724;
	// 82658E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658E44: 392AD354  addi r9, r10, -0x2cac
	ctx.r[9].s64 = ctx.r[10].s64 + -11436;
	// 82658E48: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82658E4C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82658E50: 38AA8278  addi r5, r10, -0x7d88
	ctx.r[5].s64 = ctx.r[10].s64 + -32136;
	// 82658E54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658E5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658E6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658E70: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82658E74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658E78: 386B7C18  addi r3, r11, 0x7c18
	ctx.r[3].s64 = ctx.r[11].s64 + 31768;
	// 82658E7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658E80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658E88: 4BE0DF99  bl 0x82466e20
	ctx.lr = 0x82658E8C;
	sub_82466E20(ctx, base);
	// 82658E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658EA0 size=112
    let mut pc: u32 = 0x82658EA0;
    'dispatch: loop {
        match pc {
            0x82658EA0 => {
    //   block [0x82658EA0..0x82658F10)
	// 82658EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658EAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658EB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658EB4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658EBC: 390BD250  addi r8, r11, -0x2db0
	ctx.r[8].s64 = ctx.r[11].s64 + -11696;
	// 82658EC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82658EC4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82658EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658ED8: 386A7C48  addi r3, r10, 0x7c48
	ctx.r[3].s64 = ctx.r[10].s64 + 31816;
	// 82658EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658EEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658EFC: 4BE0DF25  bl 0x82466e20
	ctx.lr = 0x82658F00;
	sub_82466E20(ctx, base);
	// 82658F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658F10 size=112
    let mut pc: u32 = 0x82658F10;
    'dispatch: loop {
        match pc {
            0x82658F10 => {
    //   block [0x82658F10..0x82658F80)
	// 82658F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658F1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658F24: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82658F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658F2C: 390BD2C8  addi r8, r11, -0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + -11576;
	// 82658F30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82658F34: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82658F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658F48: 386A7C78  addi r3, r10, 0x7c78
	ctx.r[3].s64 = ctx.r[10].s64 + 31864;
	// 82658F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658F6C: 4BE0DEB5  bl 0x82466e20
	ctx.lr = 0x82658F70;
	sub_82466E20(ctx, base);
	// 82658F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658F80 size=112
    let mut pc: u32 = 0x82658F80;
    'dispatch: loop {
        match pc {
            0x82658F80 => {
    //   block [0x82658F80..0x82658FF0)
	// 82658F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658F94: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658F9C: 390BD310  addi r8, r11, -0x2cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -11504;
	// 82658FA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82658FA4: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82658FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658FAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658FB8: 386A7CA8  addi r3, r10, 0x7ca8
	ctx.r[3].s64 = ctx.r[10].s64 + 31912;
	// 82658FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658FDC: 4BE0DE45  bl 0x82466e20
	ctx.lr = 0x82658FE0;
	sub_82466E20(ctx, base);
	// 82658FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658FF0 size=112
    let mut pc: u32 = 0x82658FF0;
    'dispatch: loop {
        match pc {
            0x82658FF0 => {
    //   block [0x82658FF0..0x82659060)
	// 82658FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658FFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659000: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659004: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265900C: 390BD358  addi r8, r11, -0x2ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -11432;
	// 82659010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82659014: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82659018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265901C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659028: 386A7CD8  addi r3, r10, 0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31960;
	// 8265902C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265903C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265904C: 4BE0DDD5  bl 0x82466e20
	ctx.lr = 0x82659050;
	sub_82466E20(ctx, base);
	// 82659050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265905C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659060 size=108
    let mut pc: u32 = 0x82659060;
    'dispatch: loop {
        match pc {
            0x82659060 => {
    //   block [0x82659060..0x826590CC)
	// 82659060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265906C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659074: 38EBD3A0  addi r7, r11, -0x2c60
	ctx.r[7].s64 = ctx.r[11].s64 + -11360;
	// 82659078: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265907C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82659080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265908C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659090: 386A7D08  addi r3, r10, 0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + 32008;
	// 82659094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265909C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826590A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826590A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826590A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826590AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826590B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826590B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826590B8: 4BE0DD69  bl 0x82466e20
	ctx.lr = 0x826590BC;
	sub_82466E20(ctx, base);
	// 826590BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826590C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826590C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826590C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826590D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826590D0 size=112
    let mut pc: u32 = 0x826590D0;
    'dispatch: loop {
        match pc {
            0x826590D0 => {
    //   block [0x826590D0..0x82659140)
	// 826590D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826590D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826590D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826590DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826590E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826590E4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 826590E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826590EC: 390BD3E8  addi r8, r11, -0x2c18
	ctx.r[8].s64 = ctx.r[11].s64 + -11288;
	// 826590F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826590F4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826590F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826590FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659108: 386A7D38  addi r3, r10, 0x7d38
	ctx.r[3].s64 = ctx.r[10].s64 + 32056;
	// 8265910C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265911C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265912C: 4BE0DCF5  bl 0x82466e20
	ctx.lr = 0x82659130;
	sub_82466E20(ctx, base);
	// 82659130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265913C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659140 size=116
    let mut pc: u32 = 0x82659140;
    'dispatch: loop {
        match pc {
            0x82659140 => {
    //   block [0x82659140..0x826591B4)
	// 82659140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265914C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659150: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659154: 392BD390  addi r9, r11, -0x2c70
	ctx.r[9].s64 = ctx.r[11].s64 + -11376;
	// 82659158: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 8265915C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659160: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82659164: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82659168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265916C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82659170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659174: 396BD460  addi r11, r11, -0x2ba0
	ctx.r[11].s64 = ctx.r[11].s64 + -11168;
	// 82659178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265917C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82659184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659188: 386A7D68  addi r3, r10, 0x7d68
	ctx.r[3].s64 = ctx.r[10].s64 + 32104;
	// 8265918C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82659190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82659194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265919C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826591A0: 4BE0DC81  bl 0x82466e20
	ctx.lr = 0x826591A4;
	sub_82466E20(ctx, base);
	// 826591A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826591A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826591AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826591B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826591B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826591B8 size=36
    let mut pc: u32 = 0x826591B8;
    'dispatch: loop {
        match pc {
            0x826591B8 => {
    //   block [0x826591B8..0x826591DC)
	// 826591B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591BC: 814BD4F8  lwz r10, -0x2b08(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11016 as u32) ) } as u64;
	// 826591C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591C4: 396B1C68  addi r11, r11, 0x1c68
	ctx.r[11].s64 = ctx.r[11].s64 + 7272;
	// 826591C8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826591CC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826591D0: 814AD4F0  lwz r10, -0x2b10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11024 as u32) ) } as u64;
	// 826591D4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826591D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826591E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826591E0 size=108
    let mut pc: u32 = 0x826591E0;
    'dispatch: loop {
        match pc {
            0x826591E0 => {
    //   block [0x826591E0..0x8265924C)
	// 826591E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826591E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826591E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826591EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826591F4: 38EB1C68  addi r7, r11, 0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + 7272;
	// 826591F8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826591FC: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82659200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265920C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659210: 386A7D98  addi r3, r10, 0x7d98
	ctx.r[3].s64 = ctx.r[10].s64 + 32152;
	// 82659214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265921C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265922C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659238: 4BE0DBE9  bl 0x82466e20
	ctx.lr = 0x8265923C;
	sub_82466E20(ctx, base);
	// 8265923C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659250 size=24
    let mut pc: u32 = 0x82659250;
    'dispatch: loop {
        match pc {
            0x82659250 => {
    //   block [0x82659250..0x82659268)
	// 82659250: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659254: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659258: 394A1D10  addi r10, r10, 0x1d10
	ctx.r[10].s64 = ctx.r[10].s64 + 7440;
	// 8265925C: 816BD4F0  lwz r11, -0x2b10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11024 as u32) ) } as u64;
	// 82659260: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82659264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659268 size=116
    let mut pc: u32 = 0x82659268;
    'dispatch: loop {
        match pc {
            0x82659268 => {
    //   block [0x82659268..0x826592DC)
	// 82659268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659274: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659278: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8265927C: 390A1D10  addi r8, r10, 0x1d10
	ctx.r[8].s64 = ctx.r[10].s64 + 7440;
	// 82659280: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659284: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659288: 38AA7D98  addi r5, r10, 0x7d98
	ctx.r[5].s64 = ctx.r[10].s64 + 32152;
	// 8265928C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659290: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82659294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265929C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 826592A0: 396BD44C  addi r11, r11, -0x2bb4
	ctx.r[11].s64 = ctx.r[11].s64 + -11188;
	// 826592A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826592A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826592AC: 386A7DC8  addi r3, r10, 0x7dc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32200;
	// 826592B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826592B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826592B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826592BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826592C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826592C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826592C8: 4BE0DB59  bl 0x82466e20
	ctx.lr = 0x826592CC;
	sub_82466E20(ctx, base);
	// 826592CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826592D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826592D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826592D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826592E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826592E0 size=112
    let mut pc: u32 = 0x826592E0;
    'dispatch: loop {
        match pc {
            0x826592E0 => {
    //   block [0x826592E0..0x82659350)
	// 826592E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826592E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826592E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826592EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826592F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826592F4: 38AA7D98  addi r5, r10, 0x7d98
	ctx.r[5].s64 = ctx.r[10].s64 + 32152;
	// 826592F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826592FC: 390BD500  addi r8, r11, -0x2b00
	ctx.r[8].s64 = ctx.r[11].s64 + -11008;
	// 82659300: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82659304: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82659308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265930C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659318: 386A7DF8  addi r3, r10, 0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + 32248;
	// 8265931C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265932C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265933C: 4BE0DAE5  bl 0x82466e20
	ctx.lr = 0x82659340;
	sub_82466E20(ctx, base);
	// 82659340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659350 size=24
    let mut pc: u32 = 0x82659350;
    'dispatch: loop {
        match pc {
            0x82659350 => {
    //   block [0x82659350..0x82659368)
	// 82659350: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659354: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659358: 394A1E00  addi r10, r10, 0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + 7680;
	// 8265935C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 82659360: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82659364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659368 size=116
    let mut pc: u32 = 0x82659368;
    'dispatch: loop {
        match pc {
            0x82659368 => {
    //   block [0x82659368..0x826593DC)
	// 82659368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659374: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659378: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265937C: 392BD410  addi r9, r11, -0x2bf0
	ctx.r[9].s64 = ctx.r[11].s64 + -11248;
	// 82659380: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659388: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8265938C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82659390: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659394: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 82659398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265939C: 396B1E00  addi r11, r11, 0x1e00
	ctx.r[11].s64 = ctx.r[11].s64 + 7680;
	// 826593A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826593A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826593A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826593AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826593B0: 386A7E28  addi r3, r10, 0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + 32296;
	// 826593B4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826593B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826593BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826593C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826593C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826593C8: 4BE0DA59  bl 0x82466e20
	ctx.lr = 0x826593CC;
	sub_82466E20(ctx, base);
	// 826593CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826593D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826593D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826593D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826593E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826593E0 size=100
    let mut pc: u32 = 0x826593E0;
    'dispatch: loop {
        match pc {
            0x826593E0 => {
    //   block [0x826593E0..0x82659444)
	// 826593E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826593E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826593E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826593EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826593F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826593F4: 38AA7F78  addi r5, r10, 0x7f78
	ctx.r[5].s64 = ctx.r[10].s64 + 32632;
	// 826593F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826593FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659400: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82659404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265940C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659414: 386A7E58  addi r3, r10, 0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + 32344;
	// 82659418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265941C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265942C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659430: 4BE0D9F1  bl 0x82466e20
	ctx.lr = 0x82659434;
	sub_82466E20(ctx, base);
	// 82659434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659448 size=100
    let mut pc: u32 = 0x82659448;
    'dispatch: loop {
        match pc {
            0x82659448 => {
    //   block [0x82659448..0x826594AC)
	// 82659448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265945C: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659468: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8265946C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265947C: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 82659480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659488: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265948C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659498: 4BE0D989  bl 0x82466e20
	ctx.lr = 0x8265949C;
	sub_82466E20(ctx, base);
	// 8265949C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826594A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826594A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826594A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826594B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826594B0 size=108
    let mut pc: u32 = 0x826594B0;
    'dispatch: loop {
        match pc {
            0x826594B0 => {
    //   block [0x826594B0..0x8265951C)
	// 826594B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826594B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826594B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826594BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826594C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826594C4: 38EBD578  addi r7, r11, -0x2a88
	ctx.r[7].s64 = ctx.r[11].s64 + -10888;
	// 826594C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826594CC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826594D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826594D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826594D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826594DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826594E0: 386A7EB8  addi r3, r10, 0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32440;
	// 826594E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826594E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826594EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826594F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826594F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826594F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826594FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659508: 4BE0D919  bl 0x82466e20
	ctx.lr = 0x8265950C;
	sub_82466E20(ctx, base);
	// 8265950C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659520 size=112
    let mut pc: u32 = 0x82659520;
    'dispatch: loop {
        match pc {
            0x82659520 => {
    //   block [0x82659520..0x82659590)
	// 82659520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265952C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659530: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659534: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265953C: 390BD5D8  addi r8, r11, -0x2a28
	ctx.r[8].s64 = ctx.r[11].s64 + -10792;
	// 82659540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659544: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82659548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265954C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659558: 386A7EE8  addi r3, r10, 0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 32488;
	// 8265955C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265956C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265957C: 4BE0D8A5  bl 0x82466e20
	ctx.lr = 0x82659580;
	sub_82466E20(ctx, base);
	// 82659580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659590 size=108
    let mut pc: u32 = 0x82659590;
    'dispatch: loop {
        match pc {
            0x82659590 => {
    //   block [0x82659590..0x826595FC)
	// 82659590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265959C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826595A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826595A4: 38EBD638  addi r7, r11, -0x29c8
	ctx.r[7].s64 = ctx.r[11].s64 + -10696;
	// 826595A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826595AC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826595B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826595B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826595B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826595BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826595C0: 386A7F18  addi r3, r10, 0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + 32536;
	// 826595C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826595C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826595CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826595D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826595D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826595D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826595DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826595E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826595E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826595E8: 4BE0D839  bl 0x82466e20
	ctx.lr = 0x826595EC;
	sub_82466E20(ctx, base);
	// 826595EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826595F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826595F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826595F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659600 size=28
    let mut pc: u32 = 0x82659600;
    'dispatch: loop {
        match pc {
            0x82659600 => {
    //   block [0x82659600..0x8265961C)
	// 82659600: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659604: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659608: 394A1ED8  addi r10, r10, 0x1ed8
	ctx.r[10].s64 = ctx.r[10].s64 + 7896;
	// 8265960C: 816BD4FC  lwz r11, -0x2b04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11012 as u32) ) } as u64;
	// 82659610: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82659614: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82659618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659620 size=112
    let mut pc: u32 = 0x82659620;
    'dispatch: loop {
        match pc {
            0x82659620 => {
    //   block [0x82659620..0x82659690)
	// 82659620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265962C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659630: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82659634: 38EA1ED8  addi r7, r10, 0x1ed8
	ctx.r[7].s64 = ctx.r[10].s64 + 7896;
	// 82659638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265963C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659640: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82659644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659648: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265964C: 396BD508  addi r11, r11, -0x2af8
	ctx.r[11].s64 = ctx.r[11].s64 + -11000;
	// 82659650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659658: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265965C: 386A7F48  addi r3, r10, 0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + 32584;
	// 82659660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659664: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82659668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265966C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82659670: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659674: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659678: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265967C: 4BE0D7A5  bl 0x82466e20
	ctx.lr = 0x82659680;
	sub_82466E20(ctx, base);
	// 82659680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265968C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659690 size=24
    let mut pc: u32 = 0x82659690;
    'dispatch: loop {
        match pc {
            0x82659690 => {
    //   block [0x82659690..0x826596A8)
	// 82659690: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659694: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659698: 394A2040  addi r10, r10, 0x2040
	ctx.r[10].s64 = ctx.r[10].s64 + 8256;
	// 8265969C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 826596A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826596A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826596A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826596A8 size=116
    let mut pc: u32 = 0x826596A8;
    'dispatch: loop {
        match pc {
            0x826596A8 => {
    //   block [0x826596A8..0x8265971C)
	// 826596A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826596AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826596B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826596B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826596B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826596BC: 392BD4E0  addi r9, r11, -0x2b20
	ctx.r[9].s64 = ctx.r[11].s64 + -11040;
	// 826596C0: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 826596C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826596C8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826596CC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826596D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826596D4: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826596D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826596DC: 396B2040  addi r11, r11, 0x2040
	ctx.r[11].s64 = ctx.r[11].s64 + 8256;
	// 826596E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826596E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826596E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826596EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826596F0: 386A7F78  addi r3, r10, 0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + 32632;
	// 826596F4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826596F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826596FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659700: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82659704: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659708: 4BE0D719  bl 0x82466e20
	ctx.lr = 0x8265970C;
	sub_82466E20(ctx, base);
	// 8265970C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659720 size=112
    let mut pc: u32 = 0x82659720;
    'dispatch: loop {
        match pc {
            0x82659720 => {
    //   block [0x82659720..0x82659790)
	// 82659720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265972C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659730: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659734: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 82659738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265973C: 390BD658  addi r8, r11, -0x29a8
	ctx.r[8].s64 = ctx.r[11].s64 + -10664;
	// 82659740: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659744: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82659748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265974C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659758: 386A7FA8  addi r3, r10, 0x7fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 32680;
	// 8265975C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265976C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265977C: 4BE0D6A5  bl 0x82466e20
	ctx.lr = 0x82659780;
	sub_82466E20(ctx, base);
	// 82659780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265978C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659790 size=24
    let mut pc: u32 = 0x82659790;
    'dispatch: loop {
        match pc {
            0x82659790 => {
    //   block [0x82659790..0x826597A8)
	// 82659790: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659794: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659798: 394A20E8  addi r10, r10, 0x20e8
	ctx.r[10].s64 = ctx.r[10].s64 + 8424;
	// 8265979C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 826597A0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826597A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826597A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826597A8 size=116
    let mut pc: u32 = 0x826597A8;
    'dispatch: loop {
        match pc {
            0x826597A8 => {
    //   block [0x826597A8..0x8265981C)
	// 826597A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826597AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826597B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826597B4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826597B8: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826597BC: 390A20E8  addi r8, r10, 0x20e8
	ctx.r[8].s64 = ctx.r[10].s64 + 8424;
	// 826597C0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826597C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826597C8: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 826597CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826597D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826597D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826597D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826597DC: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 826597E0: 396BD568  addi r11, r11, -0x2a98
	ctx.r[11].s64 = ctx.r[11].s64 + -10904;
	// 826597E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826597E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826597EC: 386A7FD8  addi r3, r10, 0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32728;
	// 826597F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826597F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826597F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826597FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659808: 4BE0D619  bl 0x82466e20
	ctx.lr = 0x8265980C;
	sub_82466E20(ctx, base);
	// 8265980C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659820 size=112
    let mut pc: u32 = 0x82659820;
    'dispatch: loop {
        match pc {
            0x82659820 => {
    //   block [0x82659820..0x82659890)
	// 82659820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265982C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659830: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659834: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265983C: 390BD6E8  addi r8, r11, -0x2918
	ctx.r[8].s64 = ctx.r[11].s64 + -10520;
	// 82659840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659844: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82659848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265984C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659858: 386A8008  addi r3, r10, -0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -32760;
	// 8265985C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265986C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82659870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265987C: 4BE0D5A5  bl 0x82466e20
	ctx.lr = 0x82659880;
	sub_82466E20(ctx, base);
	// 82659880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659890 size=108
    let mut pc: u32 = 0x82659890;
    'dispatch: loop {
        match pc {
            0x82659890 => {
    //   block [0x82659890..0x826598FC)
	// 82659890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265989C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826598A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826598A4: 38EBD718  addi r7, r11, -0x28e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10472;
	// 826598A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826598AC: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826598B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826598B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826598B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826598BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826598C0: 386A8038  addi r3, r10, -0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -32712;
	// 826598C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826598C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826598CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826598D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826598D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826598D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826598DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826598E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826598E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826598E8: 4BE0D539  bl 0x82466e20
	ctx.lr = 0x826598EC;
	sub_82466E20(ctx, base);
	// 826598EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826598F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826598F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826598F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659900 size=112
    let mut pc: u32 = 0x82659900;
    'dispatch: loop {
        match pc {
            0x82659900 => {
    //   block [0x82659900..0x82659970)
	// 82659900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265990C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659910: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659914: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265991C: 390BD748  addi r8, r11, -0x28b8
	ctx.r[8].s64 = ctx.r[11].s64 + -10424;
	// 82659920: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659924: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82659928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265992C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659938: 386A8068  addi r3, r10, -0x7f98
	ctx.r[3].s64 = ctx.r[10].s64 + -32664;
	// 8265993C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265994C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265995C: 4BE0D4C5  bl 0x82466e20
	ctx.lr = 0x82659960;
	sub_82466E20(ctx, base);
	// 82659960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265996C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659970 size=112
    let mut pc: u32 = 0x82659970;
    'dispatch: loop {
        match pc {
            0x82659970 => {
    //   block [0x82659970..0x826599E0)
	// 82659970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265997C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659980: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659984: 38AA8278  addi r5, r10, -0x7d88
	ctx.r[5].s64 = ctx.r[10].s64 + -32136;
	// 82659988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265998C: 390BD778  addi r8, r11, -0x2888
	ctx.r[8].s64 = ctx.r[11].s64 + -10376;
	// 82659990: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659994: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82659998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265999C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826599A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826599A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826599A8: 386A8098  addi r3, r10, -0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + -32616;
	// 826599AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826599B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826599B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826599B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826599BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826599C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826599C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826599C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826599CC: 4BE0D455  bl 0x82466e20
	ctx.lr = 0x826599D0;
	sub_82466E20(ctx, base);
	// 826599D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826599D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826599D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826599DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826599E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826599E0 size=108
    let mut pc: u32 = 0x826599E0;
    'dispatch: loop {
        match pc {
            0x826599E0 => {
    //   block [0x826599E0..0x82659A4C)
	// 826599E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826599E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826599E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826599EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826599F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826599F4: 38EBD7A8  addi r7, r11, -0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + -10328;
	// 826599F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826599FC: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82659A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659A04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659A10: 386A80C8  addi r3, r10, -0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + -32568;
	// 82659A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659A38: 4BE0D3E9  bl 0x82466e20
	ctx.lr = 0x82659A3C;
	sub_82466E20(ctx, base);
	// 82659A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659A50 size=108
    let mut pc: u32 = 0x82659A50;
    'dispatch: loop {
        match pc {
            0x82659A50 => {
    //   block [0x82659A50..0x82659ABC)
	// 82659A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659A5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82659A64: 38EBD7F0  addi r7, r11, -0x2810
	ctx.r[7].s64 = ctx.r[11].s64 + -10256;
	// 82659A68: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82659A6C: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 82659A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659A74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659A80: 386A80F8  addi r3, r10, -0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + -32520;
	// 82659A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659AA8: 4BE0D379  bl 0x82466e20
	ctx.lr = 0x82659AAC;
	sub_82466E20(ctx, base);
	// 82659AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659AC0 size=112
    let mut pc: u32 = 0x82659AC0;
    'dispatch: loop {
        match pc {
            0x82659AC0 => {
    //   block [0x82659AC0..0x82659B30)
	// 82659AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659ACC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659AD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659AD4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659ADC: 390BD850  addi r8, r11, -0x27b0
	ctx.r[8].s64 = ctx.r[11].s64 + -10160;
	// 82659AE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659AE4: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82659AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659AEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659AF8: 386A8128  addi r3, r10, -0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -32472;
	// 82659AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659B1C: 4BE0D305  bl 0x82466e20
	ctx.lr = 0x82659B20;
	sub_82466E20(ctx, base);
	// 82659B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659B30 size=100
    let mut pc: u32 = 0x82659B30;
    'dispatch: loop {
        match pc {
            0x82659B30 => {
    //   block [0x82659B30..0x82659B94)
	// 82659B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659B44: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659B50: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82659B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659B64: 386A8158  addi r3, r10, -0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -32424;
	// 82659B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659B80: 4BE0D2A1  bl 0x82466e20
	ctx.lr = 0x82659B84;
	sub_82466E20(ctx, base);
	// 82659B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659B98 size=112
    let mut pc: u32 = 0x82659B98;
    'dispatch: loop {
        match pc {
            0x82659B98 => {
    //   block [0x82659B98..0x82659C08)
	// 82659B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659BA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659BA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659BAC: 38AA7E88  addi r5, r10, 0x7e88
	ctx.r[5].s64 = ctx.r[10].s64 + 32392;
	// 82659BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659BB4: 390BD8B0  addi r8, r11, -0x2750
	ctx.r[8].s64 = ctx.r[11].s64 + -10064;
	// 82659BB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82659BBC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82659BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659BD0: 386A8188  addi r3, r10, -0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + -32376;
	// 82659BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659BF4: 4BE0D22D  bl 0x82466e20
	ctx.lr = 0x82659BF8;
	sub_82466E20(ctx, base);
	// 82659BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659C08 size=112
    let mut pc: u32 = 0x82659C08;
    'dispatch: loop {
        match pc {
            0x82659C08 => {
    //   block [0x82659C08..0x82659C78)
	// 82659C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659C14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659C18: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659C1C: 38AA7E88  addi r5, r10, 0x7e88
	ctx.r[5].s64 = ctx.r[10].s64 + 32392;
	// 82659C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659C24: 390BD8F8  addi r8, r11, -0x2708
	ctx.r[8].s64 = ctx.r[11].s64 + -9992;
	// 82659C28: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82659C2C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82659C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659C40: 386A81B8  addi r3, r10, -0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + -32328;
	// 82659C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659C64: 4BE0D1BD  bl 0x82466e20
	ctx.lr = 0x82659C68;
	sub_82466E20(ctx, base);
	// 82659C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659C78 size=108
    let mut pc: u32 = 0x82659C78;
    'dispatch: loop {
        match pc {
            0x82659C78 => {
    //   block [0x82659C78..0x82659CE4)
	// 82659C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659C84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659C8C: 38EBD9A0  addi r7, r11, -0x2660
	ctx.r[7].s64 = ctx.r[11].s64 + -9824;
	// 82659C90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82659C94: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82659C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659C9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659CA8: 386A81E8  addi r3, r10, -0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + -32280;
	// 82659CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659CD0: 4BE0D151  bl 0x82466e20
	ctx.lr = 0x82659CD4;
	sub_82466E20(ctx, base);
	// 82659CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659CE8 size=24
    let mut pc: u32 = 0x82659CE8;
    'dispatch: loop {
        match pc {
            0x82659CE8 => {
    //   block [0x82659CE8..0x82659D00)
	// 82659CE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659CEC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659CF0: 394A2220  addi r10, r10, 0x2220
	ctx.r[10].s64 = ctx.r[10].s64 + 8736;
	// 82659CF4: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 82659CF8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82659CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659D00 size=116
    let mut pc: u32 = 0x82659D00;
    'dispatch: loop {
        match pc {
            0x82659D00 => {
    //   block [0x82659D00..0x82659D74)
	// 82659D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659D0C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659D10: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82659D14: 390A2220  addi r8, r10, 0x2220
	ctx.r[8].s64 = ctx.r[10].s64 + 8736;
	// 82659D18: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659D1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659D20: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659D28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82659D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659D34: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82659D38: 396BD5A0  addi r11, r11, -0x2a60
	ctx.r[11].s64 = ctx.r[11].s64 + -10848;
	// 82659D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659D40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659D44: 386A8218  addi r3, r10, -0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + -32232;
	// 82659D48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82659D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659D50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82659D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659D60: 4BE0D0C1  bl 0x82466e20
	ctx.lr = 0x82659D64;
	sub_82466E20(ctx, base);
	// 82659D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659D78 size=100
    let mut pc: u32 = 0x82659D78;
    'dispatch: loop {
        match pc {
            0x82659D78 => {
    //   block [0x82659D78..0x82659DDC)
	// 82659D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659D8C: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659D98: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82659D9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659DAC: 386A8248  addi r3, r10, -0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + -32184;
	// 82659DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659DC8: 4BE0D059  bl 0x82466e20
	ctx.lr = 0x82659DCC;
	sub_82466E20(ctx, base);
	// 82659DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659DE0 size=100
    let mut pc: u32 = 0x82659DE0;
    'dispatch: loop {
        match pc {
            0x82659DE0 => {
    //   block [0x82659DE0..0x82659E44)
	// 82659DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659DEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659DF4: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659E00: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 82659E04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659E14: 386A8278  addi r3, r10, -0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + -32136;
	// 82659E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659E20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659E28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659E30: 4BE0CFF1  bl 0x82466e20
	ctx.lr = 0x82659E34;
	sub_82466E20(ctx, base);
	// 82659E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659E48 size=112
    let mut pc: u32 = 0x82659E48;
    'dispatch: loop {
        match pc {
            0x82659E48 => {
    //   block [0x82659E48..0x82659EB8)
	// 82659E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659E58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659E5C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659E64: 390BDA00  addi r8, r11, -0x2600
	ctx.r[8].s64 = ctx.r[11].s64 + -9728;
	// 82659E68: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659E6C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 82659E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659E80: 386A82A8  addi r3, r10, -0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + -32088;
	// 82659E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659EA4: 4BE0CF7D  bl 0x82466e20
	ctx.lr = 0x82659EA8;
	sub_82466E20(ctx, base);
	// 82659EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659EB8 size=112
    let mut pc: u32 = 0x82659EB8;
    'dispatch: loop {
        match pc {
            0x82659EB8 => {
    //   block [0x82659EB8..0x82659F28)
	// 82659EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659EC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659EC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659ECC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659ED4: 390BDA90  addi r8, r11, -0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + -9584;
	// 82659ED8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659EDC: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82659EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659EE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659EF0: 386A82D8  addi r3, r10, -0x7d28
	ctx.r[3].s64 = ctx.r[10].s64 + -32040;
	// 82659EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659F14: 4BE0CF0D  bl 0x82466e20
	ctx.lr = 0x82659F18;
	sub_82466E20(ctx, base);
	// 82659F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659F28 size=112
    let mut pc: u32 = 0x82659F28;
    'dispatch: loop {
        match pc {
            0x82659F28 => {
    //   block [0x82659F28..0x82659F98)
	// 82659F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659F38: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659F3C: 38AA7E28  addi r5, r10, 0x7e28
	ctx.r[5].s64 = ctx.r[10].s64 + 32296;
	// 82659F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659F44: 390BDAF0  addi r8, r11, -0x2510
	ctx.r[8].s64 = ctx.r[11].s64 + -9488;
	// 82659F48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659F4C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 82659F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659F54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659F60: 386A8308  addi r3, r10, -0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -31992;
	// 82659F64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659F84: 4BE0CE9D  bl 0x82466e20
	ctx.lr = 0x82659F88;
	sub_82466E20(ctx, base);
	// 82659F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659F98 size=112
    let mut pc: u32 = 0x82659F98;
    'dispatch: loop {
        match pc {
            0x82659F98 => {
    //   block [0x82659F98..0x8265A008)
	// 82659F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659FA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659FAC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659FB4: 390BDB20  addi r8, r11, -0x24e0
	ctx.r[8].s64 = ctx.r[11].s64 + -9440;
	// 82659FB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659FBC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82659FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659FC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659FD0: 386A8338  addi r3, r10, -0x7cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -31944;
	// 82659FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659FF4: 4BE0CE2D  bl 0x82466e20
	ctx.lr = 0x82659FF8;
	sub_82466E20(ctx, base);
	// 82659FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A008 size=112
    let mut pc: u32 = 0x8265A008;
    'dispatch: loop {
        match pc {
            0x8265A008 => {
    //   block [0x8265A008..0x8265A078)
	// 8265A008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A018: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A01C: 38AA7F78  addi r5, r10, 0x7f78
	ctx.r[5].s64 = ctx.r[10].s64 + 32632;
	// 8265A020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A024: 390BDBB0  addi r8, r11, -0x2450
	ctx.r[8].s64 = ctx.r[11].s64 + -9296;
	// 8265A028: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A02C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8265A030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A040: 386A8368  addi r3, r10, -0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + -31896;
	// 8265A044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A064: 4BE0CDBD  bl 0x82466e20
	ctx.lr = 0x8265A068;
	sub_82466E20(ctx, base);
	// 8265A068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A078 size=112
    let mut pc: u32 = 0x8265A078;
    'dispatch: loop {
        match pc {
            0x8265A078 => {
    //   block [0x8265A078..0x8265A0E8)
	// 8265A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A088: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A08C: 38AA81B8  addi r5, r10, -0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + -32328;
	// 8265A090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A094: 390BDBC8  addi r8, r11, -0x2438
	ctx.r[8].s64 = ctx.r[11].s64 + -9272;
	// 8265A098: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A09C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8265A0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A0A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A0A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A0B0: 386A8398  addi r3, r10, -0x7c68
	ctx.r[3].s64 = ctx.r[10].s64 + -31848;
	// 8265A0B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A0D4: 4BE0CD4D  bl 0x82466e20
	ctx.lr = 0x8265A0D8;
	sub_82466E20(ctx, base);
	// 8265A0D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A0E8 size=112
    let mut pc: u32 = 0x8265A0E8;
    'dispatch: loop {
        match pc {
            0x8265A0E8 => {
    //   block [0x8265A0E8..0x8265A158)
	// 8265A0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A0F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A0FC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 8265A100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A104: 390BDBF8  addi r8, r11, -0x2408
	ctx.r[8].s64 = ctx.r[11].s64 + -9224;
	// 8265A108: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A10C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8265A110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A120: 386A83C8  addi r3, r10, -0x7c38
	ctx.r[3].s64 = ctx.r[10].s64 + -31800;
	// 8265A124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A144: 4BE0CCDD  bl 0x82466e20
	ctx.lr = 0x8265A148;
	sub_82466E20(ctx, base);
	// 8265A148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265A158 size=24
    let mut pc: u32 = 0x8265A158;
    'dispatch: loop {
        match pc {
            0x8265A158 => {
    //   block [0x8265A158..0x8265A170)
	// 8265A158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A15C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A160: 394A2298  addi r10, r10, 0x2298
	ctx.r[10].s64 = ctx.r[10].s64 + 8856;
	// 8265A164: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 8265A168: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8265A16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A170 size=116
    let mut pc: u32 = 0x8265A170;
    'dispatch: loop {
        match pc {
            0x8265A170 => {
    //   block [0x8265A170..0x8265A1E4)
	// 8265A170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A17C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A180: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265A184: 390A2298  addi r8, r10, 0x2298
	ctx.r[8].s64 = ctx.r[10].s64 + 8856;
	// 8265A188: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A18C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265A190: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 8265A194: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A198: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A1A4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8265A1A8: 396BD5B8  addi r11, r11, -0x2a48
	ctx.r[11].s64 = ctx.r[11].s64 + -10824;
	// 8265A1AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A1B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A1B4: 386A83F8  addi r3, r10, -0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + -31752;
	// 8265A1B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265A1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A1C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A1D0: 4BE0CC51  bl 0x82466e20
	ctx.lr = 0x8265A1D4;
	sub_82466E20(ctx, base);
	// 8265A1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A1E8 size=112
    let mut pc: u32 = 0x8265A1E8;
    'dispatch: loop {
        match pc {
            0x8265A1E8 => {
    //   block [0x8265A1E8..0x8265A258)
	// 8265A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A1F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A1F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A1FC: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 8265A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A204: 390BDC40  addi r8, r11, -0x23c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9152;
	// 8265A208: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A20C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8265A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A220: 386A8428  addi r3, r10, -0x7bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -31704;
	// 8265A224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A244: 4BE0CBDD  bl 0x82466e20
	ctx.lr = 0x8265A248;
	sub_82466E20(ctx, base);
	// 8265A248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A258 size=112
    let mut pc: u32 = 0x8265A258;
    'dispatch: loop {
        match pc {
            0x8265A258 => {
    //   block [0x8265A258..0x8265A2C8)
	// 8265A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A268: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A26C: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 8265A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A274: 390BDC70  addi r8, r11, -0x2390
	ctx.r[8].s64 = ctx.r[11].s64 + -9104;
	// 8265A278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A27C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8265A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A290: 386A8458  addi r3, r10, -0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -31656;
	// 8265A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A2B4: 4BE0CB6D  bl 0x82466e20
	ctx.lr = 0x8265A2B8;
	sub_82466E20(ctx, base);
	// 8265A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A2C8 size=100
    let mut pc: u32 = 0x8265A2C8;
    'dispatch: loop {
        match pc {
            0x8265A2C8 => {
    //   block [0x8265A2C8..0x8265A32C)
	// 8265A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A2D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265A2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A2DC: 392AD628  addi r9, r10, -0x29d8
	ctx.r[9].s64 = ctx.r[10].s64 + -10712;
	// 8265A2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A2E8: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8265A2EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A2FC: 386A8488  addi r3, r10, -0x7b78
	ctx.r[3].s64 = ctx.r[10].s64 + -31608;
	// 8265A300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A304: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8265A308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265A30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265A314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A318: 4BE0CB09  bl 0x82466e20
	ctx.lr = 0x8265A31C;
	sub_82466E20(ctx, base);
	// 8265A31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265A330 size=24
    let mut pc: u32 = 0x8265A330;
    'dispatch: loop {
        match pc {
            0x8265A330 => {
    //   block [0x8265A330..0x8265A348)
	// 8265A330: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A334: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A338: 394A2340  addi r10, r10, 0x2340
	ctx.r[10].s64 = ctx.r[10].s64 + 9024;
	// 8265A33C: 816BDCA8  lwz r11, -0x2358(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9048 as u32) ) } as u64;
	// 8265A340: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A348 size=112
    let mut pc: u32 = 0x8265A348;
    'dispatch: loop {
        match pc {
            0x8265A348 => {
    //   block [0x8265A348..0x8265A3B8)
	// 8265A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A354: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265A358: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A35C: 392AD768  addi r9, r10, -0x2898
	ctx.r[9].s64 = ctx.r[10].s64 + -10392;
	// 8265A360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A364: 390B2340  addi r8, r11, 0x2340
	ctx.r[8].s64 = ctx.r[11].s64 + 9024;
	// 8265A368: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265A36C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8265A370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A380: 386A84B8  addi r3, r10, -0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + -31560;
	// 8265A384: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A388: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8265A38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A39C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A3A4: 4BE0CA7D  bl 0x82466e20
	ctx.lr = 0x8265A3A8;
	sub_82466E20(ctx, base);
	// 8265A3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A3B8 size=112
    let mut pc: u32 = 0x8265A3B8;
    'dispatch: loop {
        match pc {
            0x8265A3B8 => {
    //   block [0x8265A3B8..0x8265A428)
	// 8265A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A3C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A3C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A3CC: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A3D4: 390BDCB0  addi r8, r11, -0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + -9040;
	// 8265A3D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A3DC: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8265A3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A3E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A3F0: 386A84E8  addi r3, r10, -0x7b18
	ctx.r[3].s64 = ctx.r[10].s64 + -31512;
	// 8265A3F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A414: 4BE0CA0D  bl 0x82466e20
	ctx.lr = 0x8265A418;
	sub_82466E20(ctx, base);
	// 8265A418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A428 size=108
    let mut pc: u32 = 0x8265A428;
    'dispatch: loop {
        match pc {
            0x8265A428 => {
    //   block [0x8265A428..0x8265A494)
	// 8265A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A434: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A43C: 38EBDCE0  addi r7, r11, -0x2320
	ctx.r[7].s64 = ctx.r[11].s64 + -8992;
	// 8265A440: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265A444: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8265A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A44C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A458: 386A8518  addi r3, r10, -0x7ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -31464;
	// 8265A45C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A480: 4BE0C9A1  bl 0x82466e20
	ctx.lr = 0x8265A484;
	sub_82466E20(ctx, base);
	// 8265A484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A498 size=112
    let mut pc: u32 = 0x8265A498;
    'dispatch: loop {
        match pc {
            0x8265A498 => {
    //   block [0x8265A498..0x8265A508)
	// 8265A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A4A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A4A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A4AC: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A4B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265A4B4: 390BDCF8  addi r8, r11, -0x2308
	ctx.r[8].s64 = ctx.r[11].s64 + -8968;
	// 8265A4B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265A4BC: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 8265A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A4C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A4D0: 386A8548  addi r3, r10, -0x7ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -31416;
	// 8265A4D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A4F4: 4BE0C92D  bl 0x82466e20
	ctx.lr = 0x8265A4F8;
	sub_82466E20(ctx, base);
	// 8265A4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A508 size=100
    let mut pc: u32 = 0x8265A508;
    'dispatch: loop {
        match pc {
            0x8265A508 => {
    //   block [0x8265A508..0x8265A56C)
	// 8265A508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A51C: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A528: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8265A52C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A53C: 386A8578  addi r3, r10, -0x7a88
	ctx.r[3].s64 = ctx.r[10].s64 + -31368;
	// 8265A540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A544: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A548: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265A54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A550: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A558: 4BE0C8C9  bl 0x82466e20
	ctx.lr = 0x8265A55C;
	sub_82466E20(ctx, base);
	// 8265A55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A570 size=112
    let mut pc: u32 = 0x8265A570;
    'dispatch: loop {
        match pc {
            0x8265A570 => {
    //   block [0x8265A570..0x8265A5E0)
	// 8265A570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A57C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A580: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A584: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A58C: 390BDD58  addi r8, r11, -0x22a8
	ctx.r[8].s64 = ctx.r[11].s64 + -8872;
	// 8265A590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A594: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8265A598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A59C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A5A8: 386A85A8  addi r3, r10, -0x7a58
	ctx.r[3].s64 = ctx.r[10].s64 + -31320;
	// 8265A5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A5CC: 4BE0C855  bl 0x82466e20
	ctx.lr = 0x8265A5D0;
	sub_82466E20(ctx, base);
	// 8265A5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A5E0 size=112
    let mut pc: u32 = 0x8265A5E0;
    'dispatch: loop {
        match pc {
            0x8265A5E0 => {
    //   block [0x8265A5E0..0x8265A650)
	// 8265A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A5EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A5F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A5F4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A5FC: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 8265A600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A604: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8265A608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A618: 386A85D8  addi r3, r10, -0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + -31272;
	// 8265A61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A63C: 4BE0C7E5  bl 0x82466e20
	ctx.lr = 0x8265A640;
	sub_82466E20(ctx, base);
	// 8265A640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A650 size=112
    let mut pc: u32 = 0x8265A650;
    'dispatch: loop {
        match pc {
            0x8265A650 => {
    //   block [0x8265A650..0x8265A6C0)
	// 8265A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A65C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A664: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A66C: 390BDDA0  addi r8, r11, -0x2260
	ctx.r[8].s64 = ctx.r[11].s64 + -8800;
	// 8265A670: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A674: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8265A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A67C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A688: 386A8608  addi r3, r10, -0x79f8
	ctx.r[3].s64 = ctx.r[10].s64 + -31224;
	// 8265A68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A6AC: 4BE0C775  bl 0x82466e20
	ctx.lr = 0x8265A6B0;
	sub_82466E20(ctx, base);
	// 8265A6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A6C0 size=112
    let mut pc: u32 = 0x8265A6C0;
    'dispatch: loop {
        match pc {
            0x8265A6C0 => {
    //   block [0x8265A6C0..0x8265A730)
	// 8265A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A6CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A6D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A6D4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A6DC: 390BDDD0  addi r8, r11, -0x2230
	ctx.r[8].s64 = ctx.r[11].s64 + -8752;
	// 8265A6E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A6E4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8265A6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A6EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A6F8: 386A8638  addi r3, r10, -0x79c8
	ctx.r[3].s64 = ctx.r[10].s64 + -31176;
	// 8265A6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A71C: 4BE0C705  bl 0x82466e20
	ctx.lr = 0x8265A720;
	sub_82466E20(ctx, base);
	// 8265A720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A730 size=112
    let mut pc: u32 = 0x8265A730;
    'dispatch: loop {
        match pc {
            0x8265A730 => {
    //   block [0x8265A730..0x8265A7A0)
	// 8265A730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A73C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A740: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A744: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A74C: 390BDE00  addi r8, r11, -0x2200
	ctx.r[8].s64 = ctx.r[11].s64 + -8704;
	// 8265A750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A754: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8265A758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A768: 386A8668  addi r3, r10, -0x7998
	ctx.r[3].s64 = ctx.r[10].s64 + -31128;
	// 8265A76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A78C: 4BE0C695  bl 0x82466e20
	ctx.lr = 0x8265A790;
	sub_82466E20(ctx, base);
	// 8265A790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A7A0 size=112
    let mut pc: u32 = 0x8265A7A0;
    'dispatch: loop {
        match pc {
            0x8265A7A0 => {
    //   block [0x8265A7A0..0x8265A810)
	// 8265A7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A7AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A7B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A7B4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A7BC: 390BDE18  addi r8, r11, -0x21e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8680;
	// 8265A7C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A7C4: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8265A7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A7D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A7D8: 386A8698  addi r3, r10, -0x7968
	ctx.r[3].s64 = ctx.r[10].s64 + -31080;
	// 8265A7DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A7FC: 4BE0C625  bl 0x82466e20
	ctx.lr = 0x8265A800;
	sub_82466E20(ctx, base);
	// 8265A800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A810 size=112
    let mut pc: u32 = 0x8265A810;
    'dispatch: loop {
        match pc {
            0x8265A810 => {
    //   block [0x8265A810..0x8265A880)
	// 8265A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A81C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A820: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A824: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A82C: 390BDE30  addi r8, r11, -0x21d0
	ctx.r[8].s64 = ctx.r[11].s64 + -8656;
	// 8265A830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A834: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8265A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A848: 386A86C8  addi r3, r10, -0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + -31032;
	// 8265A84C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A86C: 4BE0C5B5  bl 0x82466e20
	ctx.lr = 0x8265A870;
	sub_82466E20(ctx, base);
	// 8265A870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A880 size=112
    let mut pc: u32 = 0x8265A880;
    'dispatch: loop {
        match pc {
            0x8265A880 => {
    //   block [0x8265A880..0x8265A8F0)
	// 8265A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A88C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A890: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A894: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A89C: 390BDE78  addi r8, r11, -0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + -8584;
	// 8265A8A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A8A4: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8265A8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A8B8: 386A86F8  addi r3, r10, -0x7908
	ctx.r[3].s64 = ctx.r[10].s64 + -30984;
	// 8265A8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A8DC: 4BE0C545  bl 0x82466e20
	ctx.lr = 0x8265A8E0;
	sub_82466E20(ctx, base);
	// 8265A8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A8F0 size=112
    let mut pc: u32 = 0x8265A8F0;
    'dispatch: loop {
        match pc {
            0x8265A8F0 => {
    //   block [0x8265A8F0..0x8265A960)
	// 8265A8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A8FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A900: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A904: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A90C: 390BDEC0  addi r8, r11, -0x2140
	ctx.r[8].s64 = ctx.r[11].s64 + -8512;
	// 8265A910: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A914: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8265A918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A91C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A928: 386A8728  addi r3, r10, -0x78d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30936;
	// 8265A92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A94C: 4BE0C4D5  bl 0x82466e20
	ctx.lr = 0x8265A950;
	sub_82466E20(ctx, base);
	// 8265A950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A960 size=112
    let mut pc: u32 = 0x8265A960;
    'dispatch: loop {
        match pc {
            0x8265A960 => {
    //   block [0x8265A960..0x8265A9D0)
	// 8265A960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A96C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A970: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A974: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A97C: 390BDED8  addi r8, r11, -0x2128
	ctx.r[8].s64 = ctx.r[11].s64 + -8488;
	// 8265A980: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A984: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8265A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A98C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A998: 386A8758  addi r3, r10, -0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30888;
	// 8265A99C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A9BC: 4BE0C465  bl 0x82466e20
	ctx.lr = 0x8265A9C0;
	sub_82466E20(ctx, base);
	// 8265A9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A9D0 size=116
    let mut pc: u32 = 0x8265A9D0;
    'dispatch: loop {
        match pc {
            0x8265A9D0 => {
    //   block [0x8265A9D0..0x8265AA44)
	// 8265A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A9DC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A9E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265A9E4: 390ADF08  addi r8, r10, -0x20f8
	ctx.r[8].s64 = ctx.r[10].s64 + -8440;
	// 8265A9E8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A9EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265A9F0: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A9F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A9F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AA04: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8265AA08: 396BD790  addi r11, r11, -0x2870
	ctx.r[11].s64 = ctx.r[11].s64 + -10352;
	// 8265AA0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AA14: 386A8788  addi r3, r10, -0x7878
	ctx.r[3].s64 = ctx.r[10].s64 + -30840;
	// 8265AA18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265AA1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AA20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265AA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AA30: 4BE0C3F1  bl 0x82466e20
	ctx.lr = 0x8265AA34;
	sub_82466E20(ctx, base);
	// 8265AA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AA48 size=116
    let mut pc: u32 = 0x8265AA48;
    'dispatch: loop {
        match pc {
            0x8265AA48 => {
    //   block [0x8265AA48..0x8265AABC)
	// 8265AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AA54: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265AA58: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265AA5C: 390ADF80  addi r8, r10, -0x2080
	ctx.r[8].s64 = ctx.r[10].s64 + -8320;
	// 8265AA60: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265AA68: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AA6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AA70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265AA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AA7C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8265AA80: 396BD7A8  addi r11, r11, -0x2858
	ctx.r[11].s64 = ctx.r[11].s64 + -10328;
	// 8265AA84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AA8C: 386A87B8  addi r3, r10, -0x7848
	ctx.r[3].s64 = ctx.r[10].s64 + -30792;
	// 8265AA90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265AA94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AA98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265AA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AAA8: 4BE0C379  bl 0x82466e20
	ctx.lr = 0x8265AAAC;
	sub_82466E20(ctx, base);
	// 8265AAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265AAC0 size=24
    let mut pc: u32 = 0x8265AAC0;
    'dispatch: loop {
        match pc {
            0x8265AAC0 => {
    //   block [0x8265AAC0..0x8265AAD8)
	// 8265AAC0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AAC4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265AAC8: 394A2358  addi r10, r10, 0x2358
	ctx.r[10].s64 = ctx.r[10].s64 + 9048;
	// 8265AACC: 816BE010  lwz r11, -0x1ff0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8176 as u32) ) } as u64;
	// 8265AAD0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8265AAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AAD8 size=116
    let mut pc: u32 = 0x8265AAD8;
    'dispatch: loop {
        match pc {
            0x8265AAD8 => {
    //   block [0x8265AAD8..0x8265AB4C)
	// 8265AAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AAE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265AAE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AAEC: 392BD7D4  addi r9, r11, -0x282c
	ctx.r[9].s64 = ctx.r[11].s64 + -10284;
	// 8265AAF0: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AAF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AAF8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8265AAFC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8265AB00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AB04: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8265AB08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AB0C: 396B2358  addi r11, r11, 0x2358
	ctx.r[11].s64 = ctx.r[11].s64 + 9048;
	// 8265AB10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265AB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265AB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AB20: 386A87E8  addi r3, r10, -0x7818
	ctx.r[3].s64 = ctx.r[10].s64 + -30744;
	// 8265AB24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265AB28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265AB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AB30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265AB34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265AB38: 4BE0C2E9  bl 0x82466e20
	ctx.lr = 0x8265AB3C;
	sub_82466E20(ctx, base);
	// 8265AB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AB50 size=112
    let mut pc: u32 = 0x8265AB50;
    'dispatch: loop {
        match pc {
            0x8265AB50 => {
    //   block [0x8265AB50..0x8265ABC0)
	// 8265AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AB5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AB64: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AB6C: 390BE018  addi r8, r11, -0x1fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -8168;
	// 8265AB70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265AB74: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8265AB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AB7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AB88: 386A8818  addi r3, r10, -0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30696;
	// 8265AB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ABA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ABA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ABA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ABAC: 4BE0C275  bl 0x82466e20
	ctx.lr = 0x8265ABB0;
	sub_82466E20(ctx, base);
	// 8265ABB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ABC0 size=112
    let mut pc: u32 = 0x8265ABC0;
    'dispatch: loop {
        match pc {
            0x8265ABC0 => {
    //   block [0x8265ABC0..0x8265AC30)
	// 8265ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ABC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ABCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ABD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ABD4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265ABD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ABDC: 390BE078  addi r8, r11, -0x1f88
	ctx.r[8].s64 = ctx.r[11].s64 + -8072;
	// 8265ABE0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265ABE4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8265ABE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ABEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ABF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ABF8: 386A8848  addi r3, r10, -0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30648;
	// 8265ABFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AC1C: 4BE0C205  bl 0x82466e20
	ctx.lr = 0x8265AC20;
	sub_82466E20(ctx, base);
	// 8265AC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AC30 size=112
    let mut pc: u32 = 0x8265AC30;
    'dispatch: loop {
        match pc {
            0x8265AC30 => {
    //   block [0x8265AC30..0x8265ACA0)
	// 8265AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AC3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AC40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AC44: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AC4C: 390BE120  addi r8, r11, -0x1ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -7904;
	// 8265AC50: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265AC54: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8265AC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AC5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AC68: 386A8878  addi r3, r10, -0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + -30600;
	// 8265AC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AC8C: 4BE0C195  bl 0x82466e20
	ctx.lr = 0x8265AC90;
	sub_82466E20(ctx, base);
	// 8265AC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ACA0 size=112
    let mut pc: u32 = 0x8265ACA0;
    'dispatch: loop {
        match pc {
            0x8265ACA0 => {
    //   block [0x8265ACA0..0x8265AD10)
	// 8265ACA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ACA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ACA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ACAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ACB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ACB4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265ACB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ACBC: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 8265ACC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265ACC4: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8265ACC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ACCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ACD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ACD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ACD8: 386A88A8  addi r3, r10, -0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + -30552;
	// 8265ACDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ACE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ACE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ACE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ACEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ACF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ACF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ACF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ACFC: 4BE0C125  bl 0x82466e20
	ctx.lr = 0x8265AD00;
	sub_82466E20(ctx, base);
	// 8265AD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AD10 size=112
    let mut pc: u32 = 0x8265AD10;
    'dispatch: loop {
        match pc {
            0x8265AD10 => {
    //   block [0x8265AD10..0x8265AD80)
	// 8265AD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AD24: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AD2C: 390BE1E0  addi r8, r11, -0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + -7712;
	// 8265AD30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265AD34: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8265AD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AD3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AD48: 386A88D8  addi r3, r10, -0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + -30504;
	// 8265AD4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AD6C: 4BE0C0B5  bl 0x82466e20
	ctx.lr = 0x8265AD70;
	sub_82466E20(ctx, base);
	// 8265AD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AD80 size=112
    let mut pc: u32 = 0x8265AD80;
    'dispatch: loop {
        match pc {
            0x8265AD80 => {
    //   block [0x8265AD80..0x8265ADF0)
	// 8265AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AD8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AD94: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AD98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AD9C: 390BE270  addi r8, r11, -0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + -7568;
	// 8265ADA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265ADA4: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8265ADA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ADAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ADB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ADB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ADB8: 386A8908  addi r3, r10, -0x76f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30456;
	// 8265ADBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ADC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ADC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ADC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ADCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ADD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ADD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ADD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ADDC: 4BE0C045  bl 0x82466e20
	ctx.lr = 0x8265ADE0;
	sub_82466E20(ctx, base);
	// 8265ADE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ADEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ADF0 size=112
    let mut pc: u32 = 0x8265ADF0;
    'dispatch: loop {
        match pc {
            0x8265ADF0 => {
    //   block [0x8265ADF0..0x8265AE60)
	// 8265ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ADF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ADFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AE04: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AE0C: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 8265AE10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265AE14: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8265AE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AE1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AE28: 386A8938  addi r3, r10, -0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30408;
	// 8265AE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AE4C: 4BE0BFD5  bl 0x82466e20
	ctx.lr = 0x8265AE50;
	sub_82466E20(ctx, base);
	// 8265AE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AE60 size=112
    let mut pc: u32 = 0x8265AE60;
    'dispatch: loop {
        match pc {
            0x8265AE60 => {
    //   block [0x8265AE60..0x8265AED0)
	// 8265AE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AE74: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AE78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AE7C: 390BE330  addi r8, r11, -0x1cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -7376;
	// 8265AE80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265AE84: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8265AE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AE8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AE98: 386A8968  addi r3, r10, -0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + -30360;
	// 8265AE9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AEBC: 4BE0BF65  bl 0x82466e20
	ctx.lr = 0x8265AEC0;
	sub_82466E20(ctx, base);
	// 8265AEC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AED0 size=112
    let mut pc: u32 = 0x8265AED0;
    'dispatch: loop {
        match pc {
            0x8265AED0 => {
    //   block [0x8265AED0..0x8265AF40)
	// 8265AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AEE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AEE4: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AEEC: 390BE360  addi r8, r11, -0x1ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -7328;
	// 8265AEF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265AEF4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8265AEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AEFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AF08: 386A8998  addi r3, r10, -0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + -30312;
	// 8265AF0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AF2C: 4BE0BEF5  bl 0x82466e20
	ctx.lr = 0x8265AF30;
	sub_82466E20(ctx, base);
	// 8265AF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AF40 size=100
    let mut pc: u32 = 0x8265AF40;
    'dispatch: loop {
        match pc {
            0x8265AF40 => {
    //   block [0x8265AF40..0x8265AFA4)
	// 8265AF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AF4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AF54: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AF5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AF60: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8265AF64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AF74: 386A89C8  addi r3, r10, -0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + -30264;
	// 8265AF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AF7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AF80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265AF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AF88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265AF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AF90: 4BE0BE91  bl 0x82466e20
	ctx.lr = 0x8265AF94;
	sub_82466E20(ctx, base);
	// 8265AF94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AFA8 size=112
    let mut pc: u32 = 0x8265AFA8;
    'dispatch: loop {
        match pc {
            0x8265AFA8 => {
    //   block [0x8265AFA8..0x8265B018)
	// 8265AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AFB8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AFBC: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AFC4: 390BE390  addi r8, r11, -0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + -7280;
	// 8265AFC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265AFCC: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8265AFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AFD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AFE0: 386A89F8  addi r3, r10, -0x7608
	ctx.r[3].s64 = ctx.r[10].s64 + -30216;
	// 8265AFE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B004: 4BE0BE1D  bl 0x82466e20
	ctx.lr = 0x8265B008;
	sub_82466E20(ctx, base);
	// 8265B008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B018 size=108
    let mut pc: u32 = 0x8265B018;
    'dispatch: loop {
        match pc {
            0x8265B018 => {
    //   block [0x8265B018..0x8265B084)
	// 8265B018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B024: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265B02C: 38EBE3A8  addi r7, r11, -0x1c58
	ctx.r[7].s64 = ctx.r[11].s64 + -7256;
	// 8265B030: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265B034: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 8265B038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B03C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B048: 386A8A28  addi r3, r10, -0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30168;
	// 8265B04C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B06C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B070: 4BE0BDB1  bl 0x82466e20
	ctx.lr = 0x8265B074;
	sub_82466E20(ctx, base);
	// 8265B074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B088 size=112
    let mut pc: u32 = 0x8265B088;
    'dispatch: loop {
        match pc {
            0x8265B088 => {
    //   block [0x8265B088..0x8265B0F8)
	// 8265B088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B098: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B09C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B0A4: 390BE3F0  addi r8, r11, -0x1c10
	ctx.r[8].s64 = ctx.r[11].s64 + -7184;
	// 8265B0A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265B0AC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8265B0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B0B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B0C0: 386A8A58  addi r3, r10, -0x75a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30120;
	// 8265B0C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B0E4: 4BE0BD3D  bl 0x82466e20
	ctx.lr = 0x8265B0E8;
	sub_82466E20(ctx, base);
	// 8265B0E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B0F8 size=112
    let mut pc: u32 = 0x8265B0F8;
    'dispatch: loop {
        match pc {
            0x8265B0F8 => {
    //   block [0x8265B0F8..0x8265B168)
	// 8265B0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B108: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B10C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B114: 390BE450  addi r8, r11, -0x1bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -7088;
	// 8265B118: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B11C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8265B120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B130: 386A8A88  addi r3, r10, -0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + -30072;
	// 8265B134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B154: 4BE0BCCD  bl 0x82466e20
	ctx.lr = 0x8265B158;
	sub_82466E20(ctx, base);
	// 8265B158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B168 size=112
    let mut pc: u32 = 0x8265B168;
    'dispatch: loop {
        match pc {
            0x8265B168 => {
    //   block [0x8265B168..0x8265B1D8)
	// 8265B168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B178: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B17C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B184: 390BE468  addi r8, r11, -0x1b98
	ctx.r[8].s64 = ctx.r[11].s64 + -7064;
	// 8265B188: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265B18C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8265B190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B1A0: 386A8AB8  addi r3, r10, -0x7548
	ctx.r[3].s64 = ctx.r[10].s64 + -30024;
	// 8265B1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B1C4: 4BE0BC5D  bl 0x82466e20
	ctx.lr = 0x8265B1C8;
	sub_82466E20(ctx, base);
	// 8265B1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B1D8 size=112
    let mut pc: u32 = 0x8265B1D8;
    'dispatch: loop {
        match pc {
            0x8265B1D8 => {
    //   block [0x8265B1D8..0x8265B248)
	// 8265B1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B1E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B1E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B1EC: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B1F4: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 8265B1F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B1FC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8265B200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B210: 386A8AE8  addi r3, r10, -0x7518
	ctx.r[3].s64 = ctx.r[10].s64 + -29976;
	// 8265B214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B234: 4BE0BBED  bl 0x82466e20
	ctx.lr = 0x8265B238;
	sub_82466E20(ctx, base);
	// 8265B238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B248 size=24
    let mut pc: u32 = 0x8265B248;
    'dispatch: loop {
        match pc {
            0x8265B248 => {
    //   block [0x8265B248..0x8265B260)
	// 8265B248: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B24C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B250: 394A2400  addi r10, r10, 0x2400
	ctx.r[10].s64 = ctx.r[10].s64 + 9216;
	// 8265B254: 816BE014  lwz r11, -0x1fec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8172 as u32) ) } as u64;
	// 8265B258: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265B25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B260 size=112
    let mut pc: u32 = 0x8265B260;
    'dispatch: loop {
        match pc {
            0x8265B260 => {
    //   block [0x8265B260..0x8265B2D0)
	// 8265B260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B26C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B270: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B274: 392AD830  addi r9, r10, -0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + -10192;
	// 8265B278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B27C: 390B2400  addi r8, r11, 0x2400
	ctx.r[8].s64 = ctx.r[11].s64 + 9216;
	// 8265B280: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265B284: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8265B288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B298: 386A8B18  addi r3, r10, -0x74e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29928;
	// 8265B29C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B2A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B2BC: 4BE0BB65  bl 0x82466e20
	ctx.lr = 0x8265B2C0;
	sub_82466E20(ctx, base);
	// 8265B2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B2D0 size=108
    let mut pc: u32 = 0x8265B2D0;
    'dispatch: loop {
        match pc {
            0x8265B2D0 => {
    //   block [0x8265B2D0..0x8265B33C)
	// 8265B2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B2DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B2E4: 38EBE4B0  addi r7, r11, -0x1b50
	ctx.r[7].s64 = ctx.r[11].s64 + -6992;
	// 8265B2E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265B2EC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8265B2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B2F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B300: 386A8B48  addi r3, r10, -0x74b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29880;
	// 8265B304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B328: 4BE0BAF9  bl 0x82466e20
	ctx.lr = 0x8265B32C;
	sub_82466E20(ctx, base);
	// 8265B32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B340 size=108
    let mut pc: u32 = 0x8265B340;
    'dispatch: loop {
        match pc {
            0x8265B340 => {
    //   block [0x8265B340..0x8265B3AC)
	// 8265B340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B34C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B354: 38EBE4C8  addi r7, r11, -0x1b38
	ctx.r[7].s64 = ctx.r[11].s64 + -6968;
	// 8265B358: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265B35C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8265B360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B370: 386A8B78  addi r3, r10, -0x7488
	ctx.r[3].s64 = ctx.r[10].s64 + -29832;
	// 8265B374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B398: 4BE0BA89  bl 0x82466e20
	ctx.lr = 0x8265B39C;
	sub_82466E20(ctx, base);
	// 8265B39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B3B0 size=116
    let mut pc: u32 = 0x8265B3B0;
    'dispatch: loop {
        match pc {
            0x8265B3B0 => {
    //   block [0x8265B3B0..0x8265B424)
	// 8265B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B3BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B3C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B3C4: 390BE514  addi r8, r11, -0x1aec
	ctx.r[8].s64 = ctx.r[11].s64 + -6892;
	// 8265B3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B3CC: 392AD8E8  addi r9, r10, -0x2718
	ctx.r[9].s64 = ctx.r[10].s64 + -10008;
	// 8265B3D0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B3D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265B3D8: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B3DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B3E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B3F4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B3F8: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8265B3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B400: 386B8BA8  addi r3, r11, -0x7458
	ctx.r[3].s64 = ctx.r[11].s64 + -29784;
	// 8265B404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B408: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B410: 4BE0BA11  bl 0x82466e20
	ctx.lr = 0x8265B414;
	sub_82466E20(ctx, base);
	// 8265B414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B428 size=24
    let mut pc: u32 = 0x8265B428;
    'dispatch: loop {
        match pc {
            0x8265B428 => {
    //   block [0x8265B428..0x8265B440)
	// 8265B428: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B42C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B430: 394A2448  addi r10, r10, 0x2448
	ctx.r[10].s64 = ctx.r[10].s64 + 9288;
	// 8265B434: 816BE52C  lwz r11, -0x1ad4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6868 as u32) ) } as u64;
	// 8265B438: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8265B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B440 size=116
    let mut pc: u32 = 0x8265B440;
    'dispatch: loop {
        match pc {
            0x8265B440 => {
    //   block [0x8265B440..0x8265B4B4)
	// 8265B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B44C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B450: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B454: 390B2448  addi r8, r11, 0x2448
	ctx.r[8].s64 = ctx.r[11].s64 + 9288;
	// 8265B458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B45C: 392AD944  addi r9, r10, -0x26bc
	ctx.r[9].s64 = ctx.r[10].s64 + -9916;
	// 8265B460: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B464: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8265B468: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B46C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B474: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B484: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B488: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8265B48C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B490: 386B8BD8  addi r3, r11, -0x7428
	ctx.r[3].s64 = ctx.r[11].s64 + -29736;
	// 8265B494: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8265B498: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B4A0: 4BE0B981  bl 0x82466e20
	ctx.lr = 0x8265B4A4;
	sub_82466E20(ctx, base);
	// 8265B4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B4B8 size=108
    let mut pc: u32 = 0x8265B4B8;
    'dispatch: loop {
        match pc {
            0x8265B4B8 => {
    //   block [0x8265B4B8..0x8265B524)
	// 8265B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B4C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B4CC: 38EBE538  addi r7, r11, -0x1ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -6856;
	// 8265B4D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265B4D4: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8265B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B4DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B4E8: 386A8C08  addi r3, r10, -0x73f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29688;
	// 8265B4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B510: 4BE0B911  bl 0x82466e20
	ctx.lr = 0x8265B514;
	sub_82466E20(ctx, base);
	// 8265B514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B528 size=112
    let mut pc: u32 = 0x8265B528;
    'dispatch: loop {
        match pc {
            0x8265B528 => {
    //   block [0x8265B528..0x8265B598)
	// 8265B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B534: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B538: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B53C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B544: 390BE568  addi r8, r11, -0x1a98
	ctx.r[8].s64 = ctx.r[11].s64 + -6808;
	// 8265B548: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B54C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8265B550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B560: 386A8C38  addi r3, r10, -0x73c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29640;
	// 8265B564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B584: 4BE0B89D  bl 0x82466e20
	ctx.lr = 0x8265B588;
	sub_82466E20(ctx, base);
	// 8265B588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B598 size=112
    let mut pc: u32 = 0x8265B598;
    'dispatch: loop {
        match pc {
            0x8265B598 => {
    //   block [0x8265B598..0x8265B608)
	// 8265B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B5A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B5A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B5AC: 392AD988  addi r9, r10, -0x2678
	ctx.r[9].s64 = ctx.r[10].s64 + -9848;
	// 8265B5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B5B4: 390BE588  addi r8, r11, -0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + -6776;
	// 8265B5B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265B5BC: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8265B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B5D0: 386A8C68  addi r3, r10, -0x7398
	ctx.r[3].s64 = ctx.r[10].s64 + -29592;
	// 8265B5D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B5D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B5DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B5F4: 4BE0B82D  bl 0x82466e20
	ctx.lr = 0x8265B5F8;
	sub_82466E20(ctx, base);
	// 8265B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B608 size=112
    let mut pc: u32 = 0x8265B608;
    'dispatch: loop {
        match pc {
            0x8265B608 => {
    //   block [0x8265B608..0x8265B678)
	// 8265B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B618: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B61C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B624: 390BE5D0  addi r8, r11, -0x1a30
	ctx.r[8].s64 = ctx.r[11].s64 + -6704;
	// 8265B628: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B62C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8265B630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B640: 386A8C98  addi r3, r10, -0x7368
	ctx.r[3].s64 = ctx.r[10].s64 + -29544;
	// 8265B644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B664: 4BE0B7BD  bl 0x82466e20
	ctx.lr = 0x8265B668;
	sub_82466E20(ctx, base);
	// 8265B668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B678 size=112
    let mut pc: u32 = 0x8265B678;
    'dispatch: loop {
        match pc {
            0x8265B678 => {
    //   block [0x8265B678..0x8265B6E8)
	// 8265B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B684: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B688: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B68C: 392AD9B4  addi r9, r10, -0x264c
	ctx.r[9].s64 = ctx.r[10].s64 + -9804;
	// 8265B690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B694: 390BE5E8  addi r8, r11, -0x1a18
	ctx.r[8].s64 = ctx.r[11].s64 + -6680;
	// 8265B698: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265B69C: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8265B6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B6A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B6B0: 386A8CC8  addi r3, r10, -0x7338
	ctx.r[3].s64 = ctx.r[10].s64 + -29496;
	// 8265B6B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B6B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B6D4: 4BE0B74D  bl 0x82466e20
	ctx.lr = 0x8265B6D8;
	sub_82466E20(ctx, base);
	// 8265B6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B6E8 size=112
    let mut pc: u32 = 0x8265B6E8;
    'dispatch: loop {
        match pc {
            0x8265B6E8 => {
    //   block [0x8265B6E8..0x8265B758)
	// 8265B6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B6F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B6F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B6FC: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B704: 390BE678  addi r8, r11, -0x1988
	ctx.r[8].s64 = ctx.r[11].s64 + -6536;
	// 8265B708: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B70C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8265B710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B720: 386A8CF8  addi r3, r10, -0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + -29448;
	// 8265B724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B744: 4BE0B6DD  bl 0x82466e20
	ctx.lr = 0x8265B748;
	sub_82466E20(ctx, base);
	// 8265B748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B758 size=112
    let mut pc: u32 = 0x8265B758;
    'dispatch: loop {
        match pc {
            0x8265B758 => {
    //   block [0x8265B758..0x8265B7C8)
	// 8265B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B768: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B76C: 38AA8D58  addi r5, r10, -0x72a8
	ctx.r[5].s64 = ctx.r[10].s64 + -29352;
	// 8265B770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B774: 390BE690  addi r8, r11, -0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + -6512;
	// 8265B778: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265B77C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8265B780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B790: 386A8D28  addi r3, r10, -0x72d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29400;
	// 8265B794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B7B4: 4BE0B66D  bl 0x82466e20
	ctx.lr = 0x8265B7B8;
	sub_82466E20(ctx, base);
	// 8265B7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B7C8 size=100
    let mut pc: u32 = 0x8265B7C8;
    'dispatch: loop {
        match pc {
            0x8265B7C8 => {
    //   block [0x8265B7C8..0x8265B82C)
	// 8265B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B7D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B7DC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B7E8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8265B7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B7FC: 386A8D58  addi r3, r10, -0x72a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29352;
	// 8265B800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265B80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265B814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B818: 4BE0B609  bl 0x82466e20
	ctx.lr = 0x8265B81C;
	sub_82466E20(ctx, base);
	// 8265B81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B830 size=24
    let mut pc: u32 = 0x8265B830;
    'dispatch: loop {
        match pc {
            0x8265B830 => {
    //   block [0x8265B830..0x8265B848)
	// 8265B830: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B834: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B838: 394A2520  addi r10, r10, 0x2520
	ctx.r[10].s64 = ctx.r[10].s64 + 9504;
	// 8265B83C: 816BE708  lwz r11, -0x18f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6392 as u32) ) } as u64;
	// 8265B840: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8265B844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B848 size=116
    let mut pc: u32 = 0x8265B848;
    'dispatch: loop {
        match pc {
            0x8265B848 => {
    //   block [0x8265B848..0x8265B8BC)
	// 8265B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B854: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B858: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B85C: 390B2520  addi r8, r11, 0x2520
	ctx.r[8].s64 = ctx.r[11].s64 + 9504;
	// 8265B860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B864: 392AD9F0  addi r9, r10, -0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + -9744;
	// 8265B868: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B86C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265B870: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B874: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B87C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B88C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B890: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8265B894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B898: 386B8D88  addi r3, r11, -0x7278
	ctx.r[3].s64 = ctx.r[11].s64 + -29304;
	// 8265B89C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B8A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B8A8: 4BE0B579  bl 0x82466e20
	ctx.lr = 0x8265B8AC;
	sub_82466E20(ctx, base);
	// 8265B8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B8C0 size=108
    let mut pc: u32 = 0x8265B8C0;
    'dispatch: loop {
        match pc {
            0x8265B8C0 => {
    //   block [0x8265B8C0..0x8265B92C)
	// 8265B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B8CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B8D4: 38EBE70C  addi r7, r11, -0x18f4
	ctx.r[7].s64 = ctx.r[11].s64 + -6388;
	// 8265B8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265B8DC: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8265B8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B8F0: 386A8DB8  addi r3, r10, -0x7248
	ctx.r[3].s64 = ctx.r[10].s64 + -29256;
	// 8265B8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B918: 4BE0B509  bl 0x82466e20
	ctx.lr = 0x8265B91C;
	sub_82466E20(ctx, base);
	// 8265B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B930 size=112
    let mut pc: u32 = 0x8265B930;
    'dispatch: loop {
        match pc {
            0x8265B930 => {
    //   block [0x8265B930..0x8265B9A0)
	// 8265B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B93C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B940: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B944: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B94C: 390BE73C  addi r8, r11, -0x18c4
	ctx.r[8].s64 = ctx.r[11].s64 + -6340;
	// 8265B950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B954: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8265B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B95C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B968: 386A8DE8  addi r3, r10, -0x7218
	ctx.r[3].s64 = ctx.r[10].s64 + -29208;
	// 8265B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B98C: 4BE0B495  bl 0x82466e20
	ctx.lr = 0x8265B990;
	sub_82466E20(ctx, base);
	// 8265B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B9A0 size=112
    let mut pc: u32 = 0x8265B9A0;
    'dispatch: loop {
        match pc {
            0x8265B9A0 => {
    //   block [0x8265B9A0..0x8265BA10)
	// 8265B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B9AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B9B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B9B4: 392ADA14  addi r9, r10, -0x25ec
	ctx.r[9].s64 = ctx.r[10].s64 + -9708;
	// 8265B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B9BC: 390BE758  addi r8, r11, -0x18a8
	ctx.r[8].s64 = ctx.r[11].s64 + -6312;
	// 8265B9C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265B9C4: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8265B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B9D8: 386A8E18  addi r3, r10, -0x71e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29160;
	// 8265B9DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B9FC: 4BE0B425  bl 0x82466e20
	ctx.lr = 0x8265BA00;
	sub_82466E20(ctx, base);
	// 8265BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BA10 size=112
    let mut pc: u32 = 0x8265BA10;
    'dispatch: loop {
        match pc {
            0x8265BA10 => {
    //   block [0x8265BA10..0x8265BA80)
	// 8265BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BA20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BA24: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BA2C: 390BE800  addi r8, r11, -0x1800
	ctx.r[8].s64 = ctx.r[11].s64 + -6144;
	// 8265BA30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BA34: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8265BA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BA48: 386A8E48  addi r3, r10, -0x71b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29112;
	// 8265BA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BA6C: 4BE0B3B5  bl 0x82466e20
	ctx.lr = 0x8265BA70;
	sub_82466E20(ctx, base);
	// 8265BA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BA80 size=108
    let mut pc: u32 = 0x8265BA80;
    'dispatch: loop {
        match pc {
            0x8265BA80 => {
    //   block [0x8265BA80..0x8265BAEC)
	// 8265BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BA8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BA94: 38EBE818  addi r7, r11, -0x17e8
	ctx.r[7].s64 = ctx.r[11].s64 + -6120;
	// 8265BA98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265BA9C: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8265BAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BAA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BAB0: 386A8E78  addi r3, r10, -0x7188
	ctx.r[3].s64 = ctx.r[10].s64 + -29064;
	// 8265BAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BAD8: 4BE0B349  bl 0x82466e20
	ctx.lr = 0x8265BADC;
	sub_82466E20(ctx, base);
	// 8265BADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BAF0 size=112
    let mut pc: u32 = 0x8265BAF0;
    'dispatch: loop {
        match pc {
            0x8265BAF0 => {
    //   block [0x8265BAF0..0x8265BB60)
	// 8265BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BAFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BB04: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BB0C: 390BE848  addi r8, r11, -0x17b8
	ctx.r[8].s64 = ctx.r[11].s64 + -6072;
	// 8265BB10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BB14: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8265BB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BB28: 386A8EA8  addi r3, r10, -0x7158
	ctx.r[3].s64 = ctx.r[10].s64 + -29016;
	// 8265BB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BB4C: 4BE0B2D5  bl 0x82466e20
	ctx.lr = 0x8265BB50;
	sub_82466E20(ctx, base);
	// 8265BB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BB60 size=112
    let mut pc: u32 = 0x8265BB60;
    'dispatch: loop {
        match pc {
            0x8265BB60 => {
    //   block [0x8265BB60..0x8265BBD0)
	// 8265BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BB6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265BB70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BB74: 392ADA48  addi r9, r10, -0x25b8
	ctx.r[9].s64 = ctx.r[10].s64 + -9656;
	// 8265BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BB7C: 390BE868  addi r8, r11, -0x1798
	ctx.r[8].s64 = ctx.r[11].s64 + -6040;
	// 8265BB80: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265BB84: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8265BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BB98: 386A8ED8  addi r3, r10, -0x7128
	ctx.r[3].s64 = ctx.r[10].s64 + -28968;
	// 8265BB9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265BBA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265BBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BBB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BBBC: 4BE0B265  bl 0x82466e20
	ctx.lr = 0x8265BBC0;
	sub_82466E20(ctx, base);
	// 8265BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BBD0 size=112
    let mut pc: u32 = 0x8265BBD0;
    'dispatch: loop {
        match pc {
            0x8265BBD0 => {
    //   block [0x8265BBD0..0x8265BC40)
	// 8265BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BBDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BBE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BBE4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BBEC: 390BE910  addi r8, r11, -0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5872;
	// 8265BBF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265BBF4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8265BBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BBFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BC08: 386A8F08  addi r3, r10, -0x70f8
	ctx.r[3].s64 = ctx.r[10].s64 + -28920;
	// 8265BC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BC2C: 4BE0B1F5  bl 0x82466e20
	ctx.lr = 0x8265BC30;
	sub_82466E20(ctx, base);
	// 8265BC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BC40 size=112
    let mut pc: u32 = 0x8265BC40;
    'dispatch: loop {
        match pc {
            0x8265BC40 => {
    //   block [0x8265BC40..0x8265BCB0)
	// 8265BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC50: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BC54: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BC5C: 390BE958  addi r8, r11, -0x16a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5800;
	// 8265BC60: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8265BC64: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8265BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BC6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BC78: 386A8F38  addi r3, r10, -0x70c8
	ctx.r[3].s64 = ctx.r[10].s64 + -28872;
	// 8265BC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BC9C: 4BE0B185  bl 0x82466e20
	ctx.lr = 0x8265BCA0;
	sub_82466E20(ctx, base);
	// 8265BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BCB0 size=100
    let mut pc: u32 = 0x8265BCB0;
    'dispatch: loop {
        match pc {
            0x8265BCB0 => {
    //   block [0x8265BCB0..0x8265BD14)
	// 8265BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BCBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BCC4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BCCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BCD0: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8265BCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BCE4: 386A8F68  addi r3, r10, -0x7098
	ctx.r[3].s64 = ctx.r[10].s64 + -28824;
	// 8265BCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BCEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BCF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265BCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BCF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265BCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BD00: 4BE0B121  bl 0x82466e20
	ctx.lr = 0x8265BD04;
	sub_82466E20(ctx, base);
	// 8265BD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BD18 size=112
    let mut pc: u32 = 0x8265BD18;
    'dispatch: loop {
        match pc {
            0x8265BD18 => {
    //   block [0x8265BD18..0x8265BD88)
	// 8265BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BD24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BD2C: 38AA8BD8  addi r5, r10, -0x7428
	ctx.r[5].s64 = ctx.r[10].s64 + -29736;
	// 8265BD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BD34: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 8265BD38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265BD3C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8265BD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BD44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BD4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BD50: 386A8F98  addi r3, r10, -0x7068
	ctx.r[3].s64 = ctx.r[10].s64 + -28776;
	// 8265BD54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BD74: 4BE0B0AD  bl 0x82466e20
	ctx.lr = 0x8265BD78;
	sub_82466E20(ctx, base);
	// 8265BD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BD88 size=112
    let mut pc: u32 = 0x8265BD88;
    'dispatch: loop {
        match pc {
            0x8265BD88 => {
    //   block [0x8265BD88..0x8265BDF8)
	// 8265BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BD94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BD9C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265BDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BDA4: 390BEA48  addi r8, r11, -0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5560;
	// 8265BDA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BDAC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8265BDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BDB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BDC0: 386A8FC8  addi r3, r10, -0x7038
	ctx.r[3].s64 = ctx.r[10].s64 + -28728;
	// 8265BDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BDE4: 4BE0B03D  bl 0x82466e20
	ctx.lr = 0x8265BDE8;
	sub_82466E20(ctx, base);
	// 8265BDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BDF8 size=108
    let mut pc: u32 = 0x8265BDF8;
    'dispatch: loop {
        match pc {
            0x8265BDF8 => {
    //   block [0x8265BDF8..0x8265BE64)
	// 8265BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BE04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BE0C: 38EBEA60  addi r7, r11, -0x15a0
	ctx.r[7].s64 = ctx.r[11].s64 + -5536;
	// 8265BE10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265BE14: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8265BE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BE1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BE28: 386A8FF8  addi r3, r10, -0x7008
	ctx.r[3].s64 = ctx.r[10].s64 + -28680;
	// 8265BE2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BE4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BE50: 4BE0AFD1  bl 0x82466e20
	ctx.lr = 0x8265BE54;
	sub_82466E20(ctx, base);
	// 8265BE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BE68 size=112
    let mut pc: u32 = 0x8265BE68;
    'dispatch: loop {
        match pc {
            0x8265BE68 => {
    //   block [0x8265BE68..0x8265BED8)
	// 8265BE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BE74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BE7C: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265BE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BE84: 390BEA90  addi r8, r11, -0x1570
	ctx.r[8].s64 = ctx.r[11].s64 + -5488;
	// 8265BE88: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265BE8C: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8265BE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BEA0: 386A9028  addi r3, r10, -0x6fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -28632;
	// 8265BEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BEC4: 4BE0AF5D  bl 0x82466e20
	ctx.lr = 0x8265BEC8;
	sub_82466E20(ctx, base);
	// 8265BEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BED8 size=112
    let mut pc: u32 = 0x8265BED8;
    'dispatch: loop {
        match pc {
            0x8265BED8 => {
    //   block [0x8265BED8..0x8265BF48)
	// 8265BED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BEE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265BEE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BEEC: 392ADA74  addi r9, r10, -0x258c
	ctx.r[9].s64 = ctx.r[10].s64 + -9612;
	// 8265BEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BEF4: 390BEB20  addi r8, r11, -0x14e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5344;
	// 8265BEF8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265BEFC: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8265BF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BF10: 386A9058  addi r3, r10, -0x6fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -28584;
	// 8265BF14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265BF18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265BF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BF34: 4BE0AEED  bl 0x82466e20
	ctx.lr = 0x8265BF38;
	sub_82466E20(ctx, base);
	// 8265BF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BF48 size=112
    let mut pc: u32 = 0x8265BF48;
    'dispatch: loop {
        match pc {
            0x8265BF48 => {
    //   block [0x8265BF48..0x8265BFB8)
	// 8265BF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BF54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BF5C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BF64: 390BEB68  addi r8, r11, -0x1498
	ctx.r[8].s64 = ctx.r[11].s64 + -5272;
	// 8265BF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BF6C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8265BF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BF80: 386A9088  addi r3, r10, -0x6f78
	ctx.r[3].s64 = ctx.r[10].s64 + -28536;
	// 8265BF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BFA4: 4BE0AE7D  bl 0x82466e20
	ctx.lr = 0x8265BFA8;
	sub_82466E20(ctx, base);
	// 8265BFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BFB8 size=108
    let mut pc: u32 = 0x8265BFB8;
    'dispatch: loop {
        match pc {
            0x8265BFB8 => {
    //   block [0x8265BFB8..0x8265C024)
	// 8265BFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BFC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BFCC: 38EBEB80  addi r7, r11, -0x1480
	ctx.r[7].s64 = ctx.r[11].s64 + -5248;
	// 8265BFD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265BFD4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8265BFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BFDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BFE8: 386A90B8  addi r3, r10, -0x6f48
	ctx.r[3].s64 = ctx.r[10].s64 + -28488;
	// 8265BFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C010: 4BE0AE11  bl 0x82466e20
	ctx.lr = 0x8265C014;
	sub_82466E20(ctx, base);
	// 8265C014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C028 size=116
    let mut pc: u32 = 0x8265C028;
    'dispatch: loop {
        match pc {
            0x8265C028 => {
    //   block [0x8265C028..0x8265C09C)
	// 8265C028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C034: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C038: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8265C03C: 390AEC10  addi r8, r10, -0x13f0
	ctx.r[8].s64 = ctx.r[10].s64 + -5104;
	// 8265C040: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C044: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265C048: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265C04C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C050: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C05C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8265C060: 396BDA88  addi r11, r11, -0x2578
	ctx.r[11].s64 = ctx.r[11].s64 + -9592;
	// 8265C064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C068: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C06C: 386A90E8  addi r3, r10, -0x6f18
	ctx.r[3].s64 = ctx.r[10].s64 + -28440;
	// 8265C070: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265C074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C078: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265C07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C088: 4BE0AD99  bl 0x82466e20
	ctx.lr = 0x8265C08C;
	sub_82466E20(ctx, base);
	// 8265C08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C0A0 size=108
    let mut pc: u32 = 0x8265C0A0;
    'dispatch: loop {
        match pc {
            0x8265C0A0 => {
    //   block [0x8265C0A0..0x8265C10C)
	// 8265C0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C0AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C0B4: 38EBECE8  addi r7, r11, -0x1318
	ctx.r[7].s64 = ctx.r[11].s64 + -4888;
	// 8265C0B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265C0BC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8265C0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C0D0: 386A9118  addi r3, r10, -0x6ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -28392;
	// 8265C0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C0F8: 4BE0AD29  bl 0x82466e20
	ctx.lr = 0x8265C0FC;
	sub_82466E20(ctx, base);
	// 8265C0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C110 size=112
    let mut pc: u32 = 0x8265C110;
    'dispatch: loop {
        match pc {
            0x8265C110 => {
    //   block [0x8265C110..0x8265C180)
	// 8265C110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C11C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C120: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C124: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265C128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C12C: 390BED30  addi r8, r11, -0x12d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4816;
	// 8265C130: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265C134: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8265C138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C13C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C148: 386A9148  addi r3, r10, -0x6eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -28344;
	// 8265C14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C16C: 4BE0ACB5  bl 0x82466e20
	ctx.lr = 0x8265C170;
	sub_82466E20(ctx, base);
	// 8265C170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C180 size=112
    let mut pc: u32 = 0x8265C180;
    'dispatch: loop {
        match pc {
            0x8265C180 => {
    //   block [0x8265C180..0x8265C1F0)
	// 8265C180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C18C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C190: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C194: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C19C: 390BEDA8  addi r8, r11, -0x1258
	ctx.r[8].s64 = ctx.r[11].s64 + -4696;
	// 8265C1A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C1A4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8265C1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C1AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C1B8: 386A9178  addi r3, r10, -0x6e88
	ctx.r[3].s64 = ctx.r[10].s64 + -28296;
	// 8265C1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C1DC: 4BE0AC45  bl 0x82466e20
	ctx.lr = 0x8265C1E0;
	sub_82466E20(ctx, base);
	// 8265C1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C1F0 size=108
    let mut pc: u32 = 0x8265C1F0;
    'dispatch: loop {
        match pc {
            0x8265C1F0 => {
    //   block [0x8265C1F0..0x8265C25C)
	// 8265C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C1FC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C204: 38EBEDD8  addi r7, r11, -0x1228
	ctx.r[7].s64 = ctx.r[11].s64 + -4648;
	// 8265C208: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265C20C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8265C210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C220: 386A91A8  addi r3, r10, -0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + -28248;
	// 8265C224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C248: 4BE0ABD9  bl 0x82466e20
	ctx.lr = 0x8265C24C;
	sub_82466E20(ctx, base);
	// 8265C24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C260 size=108
    let mut pc: u32 = 0x8265C260;
    'dispatch: loop {
        match pc {
            0x8265C260 => {
    //   block [0x8265C260..0x8265C2CC)
	// 8265C260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C26C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C274: 38EBEE38  addi r7, r11, -0x11c8
	ctx.r[7].s64 = ctx.r[11].s64 + -4552;
	// 8265C278: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265C27C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8265C280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C290: 386A91D8  addi r3, r10, -0x6e28
	ctx.r[3].s64 = ctx.r[10].s64 + -28200;
	// 8265C294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C2B8: 4BE0AB69  bl 0x82466e20
	ctx.lr = 0x8265C2BC;
	sub_82466E20(ctx, base);
	// 8265C2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C2D0 size=112
    let mut pc: u32 = 0x8265C2D0;
    'dispatch: loop {
        match pc {
            0x8265C2D0 => {
    //   block [0x8265C2D0..0x8265C340)
	// 8265C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C2DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C2E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C2E4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C2EC: 390BEEB0  addi r8, r11, -0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + -4432;
	// 8265C2F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265C2F4: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8265C2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C308: 386A9208  addi r3, r10, -0x6df8
	ctx.r[3].s64 = ctx.r[10].s64 + -28152;
	// 8265C30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C32C: 4BE0AAF5  bl 0x82466e20
	ctx.lr = 0x8265C330;
	sub_82466E20(ctx, base);
	// 8265C330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265C340 size=24
    let mut pc: u32 = 0x8265C340;
    'dispatch: loop {
        match pc {
            0x8265C340 => {
    //   block [0x8265C340..0x8265C358)
	// 8265C340: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C344: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C348: 394A2598  addi r10, r10, 0x2598
	ctx.r[10].s64 = ctx.r[10].s64 + 9624;
	// 8265C34C: 816BEEF8  lwz r11, -0x1108(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4360 as u32) ) } as u64;
	// 8265C350: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265C354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C358 size=116
    let mut pc: u32 = 0x8265C358;
    'dispatch: loop {
        match pc {
            0x8265C358 => {
    //   block [0x8265C358..0x8265C3CC)
	// 8265C358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C364: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C368: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C36C: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 8265C370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C374: 392ADAEC  addi r9, r10, -0x2514
	ctx.r[9].s64 = ctx.r[10].s64 + -9492;
	// 8265C378: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265C37C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265C380: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265C384: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C38C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C39C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265C3A0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8265C3A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C3A8: 386B9238  addi r3, r11, -0x6dc8
	ctx.r[3].s64 = ctx.r[11].s64 + -28104;
	// 8265C3AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C3B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C3B8: 4BE0AA69  bl 0x82466e20
	ctx.lr = 0x8265C3BC;
	sub_82466E20(ctx, base);
	// 8265C3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C3D0 size=112
    let mut pc: u32 = 0x8265C3D0;
    'dispatch: loop {
        match pc {
            0x8265C3D0 => {
    //   block [0x8265C3D0..0x8265C440)
	// 8265C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C3E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C3E4: 38AA9238  addi r5, r10, -0x6dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -28104;
	// 8265C3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C3EC: 390BEEFC  addi r8, r11, -0x1104
	ctx.r[8].s64 = ctx.r[11].s64 + -4356;
	// 8265C3F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C3F4: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8265C3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C3FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C408: 386A9268  addi r3, r10, -0x6d98
	ctx.r[3].s64 = ctx.r[10].s64 + -28056;
	// 8265C40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C42C: 4BE0A9F5  bl 0x82466e20
	ctx.lr = 0x8265C430;
	sub_82466E20(ctx, base);
	// 8265C430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265C440 size=24
    let mut pc: u32 = 0x8265C440;
    'dispatch: loop {
        match pc {
            0x8265C440 => {
    //   block [0x8265C440..0x8265C458)
	// 8265C440: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C444: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C448: 394A25B0  addi r10, r10, 0x25b0
	ctx.r[10].s64 = ctx.r[10].s64 + 9648;
	// 8265C44C: 816BEF2C  lwz r11, -0x10d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4308 as u32) ) } as u64;
	// 8265C450: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8265C454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C458 size=116
    let mut pc: u32 = 0x8265C458;
    'dispatch: loop {
        match pc {
            0x8265C458 => {
    //   block [0x8265C458..0x8265C4CC)
	// 8265C458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C464: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C468: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C46C: 390B25B0  addi r8, r11, 0x25b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9648;
	// 8265C470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C474: 392ADB28  addi r9, r10, -0x24d8
	ctx.r[9].s64 = ctx.r[10].s64 + -9432;
	// 8265C478: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C47C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265C480: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C484: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C48C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C49C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265C4A0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8265C4A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C4A8: 386B9298  addi r3, r11, -0x6d68
	ctx.r[3].s64 = ctx.r[11].s64 + -28008;
	// 8265C4AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C4B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C4B8: 4BE0A969  bl 0x82466e20
	ctx.lr = 0x8265C4BC;
	sub_82466E20(ctx, base);
	// 8265C4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C4D0 size=112
    let mut pc: u32 = 0x8265C4D0;
    'dispatch: loop {
        match pc {
            0x8265C4D0 => {
    //   block [0x8265C4D0..0x8265C540)
	// 8265C4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C4DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C4E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C4E4: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C4EC: 390BEF30  addi r8, r11, -0x10d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4304;
	// 8265C4F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265C4F4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8265C4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C4FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C508: 386A92C8  addi r3, r10, -0x6d38
	ctx.r[3].s64 = ctx.r[10].s64 + -27960;
	// 8265C50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C52C: 4BE0A8F5  bl 0x82466e20
	ctx.lr = 0x8265C530;
	sub_82466E20(ctx, base);
	// 8265C530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C540 size=112
    let mut pc: u32 = 0x8265C540;
    'dispatch: loop {
        match pc {
            0x8265C540 => {
    //   block [0x8265C540..0x8265C5B0)
	// 8265C540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C54C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C550: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C554: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C55C: 390BEF90  addi r8, r11, -0x1070
	ctx.r[8].s64 = ctx.r[11].s64 + -4208;
	// 8265C560: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C564: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8265C568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C56C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C578: 386A92F8  addi r3, r10, -0x6d08
	ctx.r[3].s64 = ctx.r[10].s64 + -27912;
	// 8265C57C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C59C: 4BE0A885  bl 0x82466e20
	ctx.lr = 0x8265C5A0;
	sub_82466E20(ctx, base);
	// 8265C5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C5B0 size=112
    let mut pc: u32 = 0x8265C5B0;
    'dispatch: loop {
        match pc {
            0x8265C5B0 => {
    //   block [0x8265C5B0..0x8265C620)
	// 8265C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C5BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C5C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C5C4: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C5CC: 390BEFC0  addi r8, r11, -0x1040
	ctx.r[8].s64 = ctx.r[11].s64 + -4160;
	// 8265C5D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265C5D4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8265C5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C5DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C5E8: 386A9328  addi r3, r10, -0x6cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -27864;
	// 8265C5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C60C: 4BE0A815  bl 0x82466e20
	ctx.lr = 0x8265C610;
	sub_82466E20(ctx, base);
	// 8265C610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C620 size=108
    let mut pc: u32 = 0x8265C620;
    'dispatch: loop {
        match pc {
            0x8265C620 => {
    //   block [0x8265C620..0x8265C68C)
	// 8265C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C62C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C634: 38EBF008  addi r7, r11, -0xff8
	ctx.r[7].s64 = ctx.r[11].s64 + -4088;
	// 8265C638: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265C63C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8265C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C650: 386A9358  addi r3, r10, -0x6ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -27816;
	// 8265C654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C678: 4BE0A7A9  bl 0x82466e20
	ctx.lr = 0x8265C67C;
	sub_82466E20(ctx, base);
	// 8265C67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C690 size=112
    let mut pc: u32 = 0x8265C690;
    'dispatch: loop {
        match pc {
            0x8265C690 => {
    //   block [0x8265C690..0x8265C700)
	// 8265C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C6A0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C6A4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C6AC: 390BF038  addi r8, r11, -0xfc8
	ctx.r[8].s64 = ctx.r[11].s64 + -4040;
	// 8265C6B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265C6B4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8265C6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C6BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C6C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C6C8: 386A9388  addi r3, r10, -0x6c78
	ctx.r[3].s64 = ctx.r[10].s64 + -27768;
	// 8265C6CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C6EC: 4BE0A735  bl 0x82466e20
	ctx.lr = 0x8265C6F0;
	sub_82466E20(ctx, base);
	// 8265C6F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C700 size=108
    let mut pc: u32 = 0x8265C700;
    'dispatch: loop {
        match pc {
            0x8265C700 => {
    //   block [0x8265C700..0x8265C76C)
	// 8265C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C70C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C714: 38EBF058  addi r7, r11, -0xfa8
	ctx.r[7].s64 = ctx.r[11].s64 + -4008;
	// 8265C718: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265C71C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 8265C720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C724: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C730: 386A93B8  addi r3, r10, -0x6c48
	ctx.r[3].s64 = ctx.r[10].s64 + -27720;
	// 8265C734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C758: 4BE0A6C9  bl 0x82466e20
	ctx.lr = 0x8265C75C;
	sub_82466E20(ctx, base);
	// 8265C75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C770 size=108
    let mut pc: u32 = 0x8265C770;
    'dispatch: loop {
        match pc {
            0x8265C770 => {
    //   block [0x8265C770..0x8265C7DC)
	// 8265C770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C77C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C784: 38EBF0A0  addi r7, r11, -0xf60
	ctx.r[7].s64 = ctx.r[11].s64 + -3936;
	// 8265C788: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265C78C: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 8265C790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C7A0: 386A93E8  addi r3, r10, -0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + -27672;
	// 8265C7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C7C8: 4BE0A659  bl 0x82466e20
	ctx.lr = 0x8265C7CC;
	sub_82466E20(ctx, base);
	// 8265C7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C7E0 size=116
    let mut pc: u32 = 0x8265C7E0;
    'dispatch: loop {
        match pc {
            0x8265C7E0 => {
    //   block [0x8265C7E0..0x8265C854)
	// 8265C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C7EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265C7F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C7F4: 392BDB5C  addi r9, r11, -0x24a4
	ctx.r[9].s64 = ctx.r[11].s64 + -9380;
	// 8265C7F8: 38AA9868  addi r5, r10, -0x6798
	ctx.r[5].s64 = ctx.r[10].s64 + -26520;
	// 8265C7FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C800: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8265C804: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8265C808: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C80C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8265C810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C814: 396BF100  addi r11, r11, -0xf00
	ctx.r[11].s64 = ctx.r[11].s64 + -3840;
	// 8265C818: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265C81C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C820: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265C824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C828: 386A9418  addi r3, r10, -0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + -27624;
	// 8265C82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C830: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265C834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C838: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265C83C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C840: 4BE0A5E1  bl 0x82466e20
	ctx.lr = 0x8265C844;
	sub_82466E20(ctx, base);
	// 8265C844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C858 size=100
    let mut pc: u32 = 0x8265C858;
    'dispatch: loop {
        match pc {
            0x8265C858 => {
    //   block [0x8265C858..0x8265C8BC)
	// 8265C858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265C868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C86C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265C870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C878: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8265C87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C88C: 386A9448  addi r3, r10, -0x6bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -27576;
	// 8265C890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C8A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C8A8: 4BE0A579  bl 0x82466e20
	ctx.lr = 0x8265C8AC;
	sub_82466E20(ctx, base);
	// 8265C8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C8C0 size=100
    let mut pc: u32 = 0x8265C8C0;
    'dispatch: loop {
        match pc {
            0x8265C8C0 => {
    //   block [0x8265C8C0..0x8265C924)
	// 8265C8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C8D4: 38AA94D8  addi r5, r10, -0x6b28
	ctx.r[5].s64 = ctx.r[10].s64 + -27432;
	// 8265C8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C8E0: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8265C8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C8F4: 386A9478  addi r3, r10, -0x6b88
	ctx.r[3].s64 = ctx.r[10].s64 + -27528;
	// 8265C8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C8FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C900: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C908: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C910: 4BE0A511  bl 0x82466e20
	ctx.lr = 0x8265C914;
	sub_82466E20(ctx, base);
	// 8265C914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C928 size=100
    let mut pc: u32 = 0x8265C928;
    'dispatch: loop {
        match pc {
            0x8265C928 => {
    //   block [0x8265C928..0x8265C98C)
	// 8265C928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C93C: 38AA9418  addi r5, r10, -0x6be8
	ctx.r[5].s64 = ctx.r[10].s64 + -27624;
	// 8265C940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C948: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8265C94C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C95C: 386A94A8  addi r3, r10, -0x6b58
	ctx.r[3].s64 = ctx.r[10].s64 + -27480;
	// 8265C960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C978: 4BE0A4A9  bl 0x82466e20
	ctx.lr = 0x8265C97C;
	sub_82466E20(ctx, base);
	// 8265C97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C990 size=104
    let mut pc: u32 = 0x8265C990;
    'dispatch: loop {
        match pc {
            0x8265C990 => {
    //   block [0x8265C990..0x8265C9F8)
	// 8265C990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C99C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C9A4: 392ADBDC  addi r9, r10, -0x2424
	ctx.r[9].s64 = ctx.r[10].s64 + -9252;
	// 8265C9A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C9B0: 38AA9448  addi r5, r10, -0x6bb8
	ctx.r[5].s64 = ctx.r[10].s64 + -27576;
	// 8265C9B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C9C4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8265C9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C9D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C9D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C9DC: 386A94D8  addi r3, r10, -0x6b28
	ctx.r[3].s64 = ctx.r[10].s64 + -27432;
	// 8265C9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C9E4: 4BE0A43D  bl 0x82466e20
	ctx.lr = 0x8265C9E8;
	sub_82466E20(ctx, base);
	// 8265C9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C9F8 size=108
    let mut pc: u32 = 0x8265C9F8;
    'dispatch: loop {
        match pc {
            0x8265C9F8 => {
    //   block [0x8265C9F8..0x8265CA64)
	// 8265C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CA04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CA0C: 38EBF2B0  addi r7, r11, -0xd50
	ctx.r[7].s64 = ctx.r[11].s64 + -3408;
	// 8265CA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265CA14: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8265CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265CA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CA28: 386A9508  addi r3, r10, -0x6af8
	ctx.r[3].s64 = ctx.r[10].s64 + -27384;
	// 8265CA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265CA50: 4BE0A3D1  bl 0x82466e20
	ctx.lr = 0x8265CA54;
	sub_82466E20(ctx, base);
	// 8265CA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CA68 size=112
    let mut pc: u32 = 0x8265CA68;
    'dispatch: loop {
        match pc {
            0x8265CA68 => {
    //   block [0x8265CA68..0x8265CAD8)
	// 8265CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CA7C: 38AA94D8  addi r5, r10, -0x6b28
	ctx.r[5].s64 = ctx.r[10].s64 + -27432;
	// 8265CA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CA84: 390BF2E0  addi r8, r11, -0xd20
	ctx.r[8].s64 = ctx.r[11].s64 + -3360;
	// 8265CA88: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265CA8C: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8265CA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CAA0: 386A9538  addi r3, r10, -0x6ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -27336;
	// 8265CAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CAC4: 4BE0A35D  bl 0x82466e20
	ctx.lr = 0x8265CAC8;
	sub_82466E20(ctx, base);
	// 8265CAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265CAD8 size=24
    let mut pc: u32 = 0x8265CAD8;
    'dispatch: loop {
        match pc {
            0x8265CAD8 => {
    //   block [0x8265CAD8..0x8265CAF0)
	// 8265CAD8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CADC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265CAE0: 394A2628  addi r10, r10, 0x2628
	ctx.r[10].s64 = ctx.r[10].s64 + 9768;
	// 8265CAE4: 816BF388  lwz r11, -0xc78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3192 as u32) ) } as u64;
	// 8265CAE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CAF0 size=116
    let mut pc: u32 = 0x8265CAF0;
    'dispatch: loop {
        match pc {
            0x8265CAF0 => {
    //   block [0x8265CAF0..0x8265CB64)
	// 8265CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CAFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CB00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265CB04: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 8265CB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CB0C: 392ADC40  addi r9, r10, -0x23c0
	ctx.r[9].s64 = ctx.r[10].s64 + -9152;
	// 8265CB10: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265CB14: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8265CB18: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265CB1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CB24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CB34: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265CB38: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8265CB3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265CB40: 386B9568  addi r3, r11, -0x6a98
	ctx.r[3].s64 = ctx.r[11].s64 + -27288;
	// 8265CB44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265CB48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CB50: 4BE0A2D1  bl 0x82466e20
	ctx.lr = 0x8265CB54;
	sub_82466E20(ctx, base);
	// 8265CB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CB68 size=100
    let mut pc: u32 = 0x8265CB68;
    'dispatch: loop {
        match pc {
            0x8265CB68 => {
    //   block [0x8265CB68..0x8265CBCC)
	// 8265CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CB7C: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CB88: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8265CB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CB9C: 386A9598  addi r3, r10, -0x6a68
	ctx.r[3].s64 = ctx.r[10].s64 + -27240;
	// 8265CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CBA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CBA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CBB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CBB8: 4BE0A269  bl 0x82466e20
	ctx.lr = 0x8265CBBC;
	sub_82466E20(ctx, base);
	// 8265CBBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CBD0 size=100
    let mut pc: u32 = 0x8265CBD0;
    'dispatch: loop {
        match pc {
            0x8265CBD0 => {
    //   block [0x8265CBD0..0x8265CC34)
	// 8265CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CBDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CBE4: 38AA95F8  addi r5, r10, -0x6a08
	ctx.r[5].s64 = ctx.r[10].s64 + -27144;
	// 8265CBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CBF0: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8265CBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CC04: 386A95C8  addi r3, r10, -0x6a38
	ctx.r[3].s64 = ctx.r[10].s64 + -27192;
	// 8265CC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CC0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CC10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CC18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CC20: 4BE0A201  bl 0x82466e20
	ctx.lr = 0x8265CC24;
	sub_82466E20(ctx, base);
	// 8265CC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CC38 size=112
    let mut pc: u32 = 0x8265CC38;
    'dispatch: loop {
        match pc {
            0x8265CC38 => {
    //   block [0x8265CC38..0x8265CCA8)
	// 8265CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CC44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CC48: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CC4C: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CC54: 390BF38C  addi r8, r11, -0xc74
	ctx.r[8].s64 = ctx.r[11].s64 + -3188;
	// 8265CC58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CC5C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8265CC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CC70: 386A95F8  addi r3, r10, -0x6a08
	ctx.r[3].s64 = ctx.r[10].s64 + -27144;
	// 8265CC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CC94: 4BE0A18D  bl 0x82466e20
	ctx.lr = 0x8265CC98;
	sub_82466E20(ctx, base);
	// 8265CC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CCA8 size=100
    let mut pc: u32 = 0x8265CCA8;
    'dispatch: loop {
        match pc {
            0x8265CCA8 => {
    //   block [0x8265CCA8..0x8265CD0C)
	// 8265CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CCB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CCBC: 38AA95F8  addi r5, r10, -0x6a08
	ctx.r[5].s64 = ctx.r[10].s64 + -27144;
	// 8265CCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CCC8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8265CCCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CCDC: 386A9628  addi r3, r10, -0x69d8
	ctx.r[3].s64 = ctx.r[10].s64 + -27096;
	// 8265CCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CCE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CCE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CCF8: 4BE0A129  bl 0x82466e20
	ctx.lr = 0x8265CCFC;
	sub_82466E20(ctx, base);
	// 8265CCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CD10 size=100
    let mut pc: u32 = 0x8265CD10;
    'dispatch: loop {
        match pc {
            0x8265CD10 => {
    //   block [0x8265CD10..0x8265CD74)
	// 8265CD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CD24: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CD30: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8265CD34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CD44: 386A9658  addi r3, r10, -0x69a8
	ctx.r[3].s64 = ctx.r[10].s64 + -27048;
	// 8265CD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CD50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CD58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CD60: 4BE0A0C1  bl 0x82466e20
	ctx.lr = 0x8265CD64;
	sub_82466E20(ctx, base);
	// 8265CD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CD78 size=100
    let mut pc: u32 = 0x8265CD78;
    'dispatch: loop {
        match pc {
            0x8265CD78 => {
    //   block [0x8265CD78..0x8265CDDC)
	// 8265CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CD8C: 38AA9598  addi r5, r10, -0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + -27240;
	// 8265CD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CD98: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8265CD9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CDAC: 386A9688  addi r3, r10, -0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + -27000;
	// 8265CDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CDB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CDB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CDC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CDC8: 4BE0A059  bl 0x82466e20
	ctx.lr = 0x8265CDCC;
	sub_82466E20(ctx, base);
	// 8265CDCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CDE0 size=100
    let mut pc: u32 = 0x8265CDE0;
    'dispatch: loop {
        match pc {
            0x8265CDE0 => {
    //   block [0x8265CDE0..0x8265CE44)
	// 8265CDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CDEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CDF4: 38AA9658  addi r5, r10, -0x69a8
	ctx.r[5].s64 = ctx.r[10].s64 + -27048;
	// 8265CDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CE00: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8265CE04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CE14: 386A96B8  addi r3, r10, -0x6948
	ctx.r[3].s64 = ctx.r[10].s64 + -26952;
	// 8265CE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CE1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CE20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CE28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CE2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CE30: 4BE09FF1  bl 0x82466e20
	ctx.lr = 0x8265CE34;
	sub_82466E20(ctx, base);
	// 8265CE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CE48 size=100
    let mut pc: u32 = 0x8265CE48;
    'dispatch: loop {
        match pc {
            0x8265CE48 => {
    //   block [0x8265CE48..0x8265CEAC)
	// 8265CE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CE54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CE5C: 38AA9598  addi r5, r10, -0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + -27240;
	// 8265CE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CE68: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8265CE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CE7C: 386A96E8  addi r3, r10, -0x6918
	ctx.r[3].s64 = ctx.r[10].s64 + -26904;
	// 8265CE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CE84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CE88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CE90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CE98: 4BE09F89  bl 0x82466e20
	ctx.lr = 0x8265CE9C;
	sub_82466E20(ctx, base);
	// 8265CE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CEB0 size=112
    let mut pc: u32 = 0x8265CEB0;
    'dispatch: loop {
        match pc {
            0x8265CEB0 => {
    //   block [0x8265CEB0..0x8265CF20)
	// 8265CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CEBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CEC0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CEC4: 38AA9778  addi r5, r10, -0x6888
	ctx.r[5].s64 = ctx.r[10].s64 + -26760;
	// 8265CEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CECC: 390BF3BC  addi r8, r11, -0xc44
	ctx.r[8].s64 = ctx.r[11].s64 + -3140;
	// 8265CED0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CED4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8265CED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CEE8: 386A9718  addi r3, r10, -0x68e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26856;
	// 8265CEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CF0C: 4BE09F15  bl 0x82466e20
	ctx.lr = 0x8265CF10;
	sub_82466E20(ctx, base);
	// 8265CF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CF20 size=112
    let mut pc: u32 = 0x8265CF20;
    'dispatch: loop {
        match pc {
            0x8265CF20 => {
    //   block [0x8265CF20..0x8265CF90)
	// 8265CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CF2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CF30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CF34: 38AA97A8  addi r5, r10, -0x6858
	ctx.r[5].s64 = ctx.r[10].s64 + -26712;
	// 8265CF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CF3C: 390BF3EC  addi r8, r11, -0xc14
	ctx.r[8].s64 = ctx.r[11].s64 + -3092;
	// 8265CF40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265CF44: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8265CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CF4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CF58: 386A9748  addi r3, r10, -0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + -26808;
	// 8265CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CF7C: 4BE09EA5  bl 0x82466e20
	ctx.lr = 0x8265CF80;
	sub_82466E20(ctx, base);
	// 8265CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CF90 size=112
    let mut pc: u32 = 0x8265CF90;
    'dispatch: loop {
        match pc {
            0x8265CF90 => {
    //   block [0x8265CF90..0x8265D000)
	// 8265CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CF9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CFA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CFA4: 38AA9868  addi r5, r10, -0x6798
	ctx.r[5].s64 = ctx.r[10].s64 + -26520;
	// 8265CFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CFAC: 390BF404  addi r8, r11, -0xbfc
	ctx.r[8].s64 = ctx.r[11].s64 + -3068;
	// 8265CFB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CFB4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8265CFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CFBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CFC8: 386A9778  addi r3, r10, -0x6888
	ctx.r[3].s64 = ctx.r[10].s64 + -26760;
	// 8265CFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CFEC: 4BE09E35  bl 0x82466e20
	ctx.lr = 0x8265CFF0;
	sub_82466E20(ctx, base);
	// 8265CFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D000 size=112
    let mut pc: u32 = 0x8265D000;
    'dispatch: loop {
        match pc {
            0x8265D000 => {
    //   block [0x8265D000..0x8265D070)
	// 8265D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D00C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D010: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D014: 38AA9778  addi r5, r10, -0x6888
	ctx.r[5].s64 = ctx.r[10].s64 + -26760;
	// 8265D018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D01C: 390BF434  addi r8, r11, -0xbcc
	ctx.r[8].s64 = ctx.r[11].s64 + -3020;
	// 8265D020: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D024: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8265D028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D02C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D038: 386A97A8  addi r3, r10, -0x6858
	ctx.r[3].s64 = ctx.r[10].s64 + -26712;
	// 8265D03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D05C: 4BE09DC5  bl 0x82466e20
	ctx.lr = 0x8265D060;
	sub_82466E20(ctx, base);
	// 8265D060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D070 size=112
    let mut pc: u32 = 0x8265D070;
    'dispatch: loop {
        match pc {
            0x8265D070 => {
    //   block [0x8265D070..0x8265D0E0)
	// 8265D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D07C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D080: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D084: 38AA97A8  addi r5, r10, -0x6858
	ctx.r[5].s64 = ctx.r[10].s64 + -26712;
	// 8265D088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D08C: 390BF44C  addi r8, r11, -0xbb4
	ctx.r[8].s64 = ctx.r[11].s64 + -2996;
	// 8265D090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D094: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8265D098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D09C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D0A8: 386A97D8  addi r3, r10, -0x6828
	ctx.r[3].s64 = ctx.r[10].s64 + -26664;
	// 8265D0AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D0CC: 4BE09D55  bl 0x82466e20
	ctx.lr = 0x8265D0D0;
	sub_82466E20(ctx, base);
	// 8265D0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


