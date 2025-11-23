pub fn sub_826D3B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3B88 size=112
    let mut pc: u32 = 0x826D3B88;
    'dispatch: loop {
        match pc {
            0x826D3B88 => {
    //   block [0x826D3B88..0x826D3BF8)
	// 826D3B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3B94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3B9C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3BA4: 390B0908  addi r8, r11, 0x908
	ctx.r[8].s64 = ctx.r[11].s64 + 2312;
	// 826D3BA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D3BAC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826D3BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3BC0: 386AB8F4  addi r3, r10, -0x470c
	ctx.r[3].s64 = ctx.r[10].s64 + -18188;
	// 826D3BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3BE4: 4BD9323D  bl 0x82466e20
	ctx.lr = 0x826D3BE8;
	sub_82466E20(ctx, base);
	// 826D3BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3BF8 size=108
    let mut pc: u32 = 0x826D3BF8;
    'dispatch: loop {
        match pc {
            0x826D3BF8 => {
    //   block [0x826D3BF8..0x826D3C64)
	// 826D3BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3C04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3C0C: 38EB0980  addi r7, r11, 0x980
	ctx.r[7].s64 = ctx.r[11].s64 + 2432;
	// 826D3C10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D3C14: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826D3C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3C1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3C20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3C28: 386AB924  addi r3, r10, -0x46dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18140;
	// 826D3C2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3C50: 4BD931D1  bl 0x82466e20
	ctx.lr = 0x826D3C54;
	sub_82466E20(ctx, base);
	// 826D3C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3C68 size=112
    let mut pc: u32 = 0x826D3C68;
    'dispatch: loop {
        match pc {
            0x826D3C68 => {
    //   block [0x826D3C68..0x826D3CD8)
	// 826D3C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3C74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3C78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3C7C: 392A4290  addi r9, r10, 0x4290
	ctx.r[9].s64 = ctx.r[10].s64 + 17040;
	// 826D3C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3C84: 390B0A2C  addi r8, r11, 0xa2c
	ctx.r[8].s64 = ctx.r[11].s64 + 2604;
	// 826D3C88: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D3C8C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826D3C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3C94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3CA0: 386AB954  addi r3, r10, -0x46ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18092;
	// 826D3CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3CA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3CC4: 4BD9315D  bl 0x82466e20
	ctx.lr = 0x826D3CC8;
	sub_82466E20(ctx, base);
	// 826D3CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3CD8 size=100
    let mut pc: u32 = 0x826D3CD8;
    'dispatch: loop {
        match pc {
            0x826D3CD8 => {
    //   block [0x826D3CD8..0x826D3D3C)
	// 826D3CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3CE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3CEC: 38AAC164  addi r5, r10, -0x3e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -16028;
	// 826D3CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3CF8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826D3CFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3D0C: 386AB984  addi r3, r10, -0x467c
	ctx.r[3].s64 = ctx.r[10].s64 + -18044;
	// 826D3D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3D28: 4BD930F9  bl 0x82466e20
	ctx.lr = 0x826D3D2C;
	sub_82466E20(ctx, base);
	// 826D3D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3D40 size=24
    let mut pc: u32 = 0x826D3D40;
    'dispatch: loop {
        match pc {
            0x826D3D40 => {
    //   block [0x826D3D40..0x826D3D58)
	// 826D3D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3D44: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3D48: 394A9470  addi r10, r10, -0x6b90
	ctx.r[10].s64 = ctx.r[10].s64 + -27536;
	// 826D3D4C: 816B0A60  lwz r11, 0xa60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2656 as u32) ) } as u64;
	// 826D3D50: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3D58 size=108
    let mut pc: u32 = 0x826D3D58;
    'dispatch: loop {
        match pc {
            0x826D3D58 => {
    //   block [0x826D3D58..0x826D3DC4)
	// 826D3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3D64: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3D6C: 38EB9470  addi r7, r11, -0x6b90
	ctx.r[7].s64 = ctx.r[11].s64 + -27536;
	// 826D3D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D3D74: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826D3D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3D7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3D88: 386AB9B4  addi r3, r10, -0x464c
	ctx.r[3].s64 = ctx.r[10].s64 + -17996;
	// 826D3D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3DB0: 4BD93071  bl 0x82466e20
	ctx.lr = 0x826D3DB4;
	sub_82466E20(ctx, base);
	// 826D3DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3DC8 size=112
    let mut pc: u32 = 0x826D3DC8;
    'dispatch: loop {
        match pc {
            0x826D3DC8 => {
    //   block [0x826D3DC8..0x826D3E38)
	// 826D3DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3DD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3DD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3DDC: 392A4310  addi r9, r10, 0x4310
	ctx.r[9].s64 = ctx.r[10].s64 + 17168;
	// 826D3DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3DE4: 390B0A68  addi r8, r11, 0xa68
	ctx.r[8].s64 = ctx.r[11].s64 + 2664;
	// 826D3DE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D3DEC: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826D3DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3DF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3E00: 386AB9E4  addi r3, r10, -0x461c
	ctx.r[3].s64 = ctx.r[10].s64 + -17948;
	// 826D3E04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3E08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D3E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3E24: 4BD92FFD  bl 0x82466e20
	ctx.lr = 0x826D3E28;
	sub_82466E20(ctx, base);
	// 826D3E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3E38 size=112
    let mut pc: u32 = 0x826D3E38;
    'dispatch: loop {
        match pc {
            0x826D3E38 => {
    //   block [0x826D3E38..0x826D3EA8)
	// 826D3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3E48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3E4C: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3E54: 390B0AB0  addi r8, r11, 0xab0
	ctx.r[8].s64 = ctx.r[11].s64 + 2736;
	// 826D3E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D3E5C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826D3E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3E70: 386ABA14  addi r3, r10, -0x45ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17900;
	// 826D3E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3E94: 4BD92F8D  bl 0x82466e20
	ctx.lr = 0x826D3E98;
	sub_82466E20(ctx, base);
	// 826D3E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3EA8 size=116
    let mut pc: u32 = 0x826D3EA8;
    'dispatch: loop {
        match pc {
            0x826D3EA8 => {
    //   block [0x826D3EA8..0x826D3F1C)
	// 826D3EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3EB4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D3EB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D3EBC: 390A0AE0  addi r8, r10, 0xae0
	ctx.r[8].s64 = ctx.r[10].s64 + 2784;
	// 826D3EC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3EC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3EC8: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D3ECC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3ED0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3EDC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826D3EE0: 396B4338  addi r11, r11, 0x4338
	ctx.r[11].s64 = ctx.r[11].s64 + 17208;
	// 826D3EE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3EE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3EEC: 386ABA44  addi r3, r10, -0x45bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17852;
	// 826D3EF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D3EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3EF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D3EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3F08: 4BD92F19  bl 0x82466e20
	ctx.lr = 0x826D3F0C;
	sub_82466E20(ctx, base);
	// 826D3F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3F20 size=100
    let mut pc: u32 = 0x826D3F20;
    'dispatch: loop {
        match pc {
            0x826D3F20 => {
    //   block [0x826D3F20..0x826D3F84)
	// 826D3F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3F34: 38AABA44  addi r5, r10, -0x45bc
	ctx.r[5].s64 = ctx.r[10].s64 + -17852;
	// 826D3F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3F40: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826D3F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3F54: 386ABA74  addi r3, r10, -0x458c
	ctx.r[3].s64 = ctx.r[10].s64 + -17804;
	// 826D3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3F5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3F60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3F68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3F70: 4BD92EB1  bl 0x82466e20
	ctx.lr = 0x826D3F74;
	sub_82466E20(ctx, base);
	// 826D3F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3F88 size=24
    let mut pc: u32 = 0x826D3F88;
    'dispatch: loop {
        match pc {
            0x826D3F88 => {
    //   block [0x826D3F88..0x826D3FA0)
	// 826D3F88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3F8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3F90: 394A94B8  addi r10, r10, -0x6b48
	ctx.r[10].s64 = ctx.r[10].s64 + -27464;
	// 826D3F94: 816B0A64  lwz r11, 0xa64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2660 as u32) ) } as u64;
	// 826D3F98: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D3F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3FA0 size=116
    let mut pc: u32 = 0x826D3FA0;
    'dispatch: loop {
        match pc {
            0x826D3FA0 => {
    //   block [0x826D3FA0..0x826D4014)
	// 826D3FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3FAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3FB0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3FB4: 392B4374  addi r9, r11, 0x4374
	ctx.r[9].s64 = ctx.r[11].s64 + 17268;
	// 826D3FB8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3FC0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D3FC4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D3FC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3FCC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826D3FD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3FD4: 396B94B8  addi r11, r11, -0x6b48
	ctx.r[11].s64 = ctx.r[11].s64 + -27464;
	// 826D3FD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D3FDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3FE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D3FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3FE8: 386ABAA4  addi r3, r10, -0x455c
	ctx.r[3].s64 = ctx.r[10].s64 + -17756;
	// 826D3FEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3FF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D3FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3FF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D3FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4000: 4BD92E21  bl 0x82466e20
	ctx.lr = 0x826D4004;
	sub_82466E20(ctx, base);
	// 826D4004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D400C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4018 size=116
    let mut pc: u32 = 0x826D4018;
    'dispatch: loop {
        match pc {
            0x826D4018 => {
    //   block [0x826D4018..0x826D408C)
	// 826D4018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D401C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4024: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4028: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D402C: 392B43D0  addi r9, r11, 0x43d0
	ctx.r[9].s64 = ctx.r[11].s64 + 17360;
	// 826D4030: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D4034: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4038: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826D403C: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826D4040: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4044: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826D4048: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D404C: 396B0B90  addi r11, r11, 0xb90
	ctx.r[11].s64 = ctx.r[11].s64 + 2960;
	// 826D4050: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D4054: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4058: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D405C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4060: 386ABAD4  addi r3, r10, -0x452c
	ctx.r[3].s64 = ctx.r[10].s64 + -17708;
	// 826D4064: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4068: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D406C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4070: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D4074: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4078: 4BD92DA9  bl 0x82466e20
	ctx.lr = 0x826D407C;
	sub_82466E20(ctx, base);
	// 826D407C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4090 size=108
    let mut pc: u32 = 0x826D4090;
    'dispatch: loop {
        match pc {
            0x826D4090 => {
    //   block [0x826D4090..0x826D40FC)
	// 826D4090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D409C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D40A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D40A4: 38EB0CE0  addi r7, r11, 0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + 3296;
	// 826D40A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D40AC: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826D40B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D40B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D40B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D40BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D40C0: 386ABB04  addi r3, r10, -0x44fc
	ctx.r[3].s64 = ctx.r[10].s64 + -17660;
	// 826D40C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D40C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D40CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D40D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D40D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D40D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D40DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D40E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D40E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D40E8: 4BD92D39  bl 0x82466e20
	ctx.lr = 0x826D40EC;
	sub_82466E20(ctx, base);
	// 826D40EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D40F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D40F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D40F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4100 size=24
    let mut pc: u32 = 0x826D4100;
    'dispatch: loop {
        match pc {
            0x826D4100 => {
    //   block [0x826D4100..0x826D4118)
	// 826D4100: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4104: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4108: 394A9560  addi r10, r10, -0x6aa0
	ctx.r[10].s64 = ctx.r[10].s64 + -27296;
	// 826D410C: 816B0B8C  lwz r11, 0xb8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2956 as u32) ) } as u64;
	// 826D4110: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D4114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4118 size=116
    let mut pc: u32 = 0x826D4118;
    'dispatch: loop {
        match pc {
            0x826D4118 => {
    //   block [0x826D4118..0x826D418C)
	// 826D4118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4128: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D412C: 392B4468  addi r9, r11, 0x4468
	ctx.r[9].s64 = ctx.r[11].s64 + 17512;
	// 826D4130: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D4134: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4138: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826D413C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 826D4140: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4144: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826D4148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D414C: 396B9560  addi r11, r11, -0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -27296;
	// 826D4150: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D4154: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4158: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D415C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4160: 386ABB34  addi r3, r10, -0x44cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17612;
	// 826D4164: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D4168: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D416C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4170: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D4174: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4178: 4BD92CA9  bl 0x82466e20
	ctx.lr = 0x826D417C;
	sub_82466E20(ctx, base);
	// 826D417C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4190 size=112
    let mut pc: u32 = 0x826D4190;
    'dispatch: loop {
        match pc {
            0x826D4190 => {
    //   block [0x826D4190..0x826D4200)
	// 826D4190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D419C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D41A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D41A4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D41A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D41AC: 390B0D44  addi r8, r11, 0xd44
	ctx.r[8].s64 = ctx.r[11].s64 + 3396;
	// 826D41B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D41B4: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826D41B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D41BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D41C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D41C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D41C8: 386ABB64  addi r3, r10, -0x449c
	ctx.r[3].s64 = ctx.r[10].s64 + -17564;
	// 826D41CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D41D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D41D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D41D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D41DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D41E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D41E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D41E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D41EC: 4BD92C35  bl 0x82466e20
	ctx.lr = 0x826D41F0;
	sub_82466E20(ctx, base);
	// 826D41F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D41F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D41F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D41FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4200 size=24
    let mut pc: u32 = 0x826D4200;
    'dispatch: loop {
        match pc {
            0x826D4200 => {
    //   block [0x826D4200..0x826D4218)
	// 826D4200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4204: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4208: 394A9788  addi r10, r10, -0x6878
	ctx.r[10].s64 = ctx.r[10].s64 + -26744;
	// 826D420C: 816B0D74  lwz r11, 0xd74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3444 as u32) ) } as u64;
	// 826D4210: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D4214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4218 size=116
    let mut pc: u32 = 0x826D4218;
    'dispatch: loop {
        match pc {
            0x826D4218 => {
    //   block [0x826D4218..0x826D428C)
	// 826D4218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4224: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4228: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D422C: 390B9788  addi r8, r11, -0x6878
	ctx.r[8].s64 = ctx.r[11].s64 + -26744;
	// 826D4230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4234: 392A450C  addi r9, r10, 0x450c
	ctx.r[9].s64 = ctx.r[10].s64 + 17676;
	// 826D4238: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D423C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D4240: 38AABAD4  addi r5, r10, -0x452c
	ctx.r[5].s64 = ctx.r[10].s64 + -17708;
	// 826D4244: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D424C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D425C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D4260: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826D4264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4268: 386BBB94  addi r3, r11, -0x446c
	ctx.r[3].s64 = ctx.r[11].s64 + -17516;
	// 826D426C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4278: 4BD92BA9  bl 0x82466e20
	ctx.lr = 0x826D427C;
	sub_82466E20(ctx, base);
	// 826D427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4290 size=112
    let mut pc: u32 = 0x826D4290;
    'dispatch: loop {
        match pc {
            0x826D4290 => {
    //   block [0x826D4290..0x826D4300)
	// 826D4290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D429C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D42A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D42A4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D42A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D42AC: 390B0D78  addi r8, r11, 0xd78
	ctx.r[8].s64 = ctx.r[11].s64 + 3448;
	// 826D42B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D42B4: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826D42B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D42BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D42C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D42C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D42C8: 386ABBC4  addi r3, r10, -0x443c
	ctx.r[3].s64 = ctx.r[10].s64 + -17468;
	// 826D42CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D42D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D42D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D42D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D42DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D42E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D42E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D42E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D42EC: 4BD92B35  bl 0x82466e20
	ctx.lr = 0x826D42F0;
	sub_82466E20(ctx, base);
	// 826D42F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D42F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D42F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D42FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4300 size=100
    let mut pc: u32 = 0x826D4300;
    'dispatch: loop {
        match pc {
            0x826D4300 => {
    //   block [0x826D4300..0x826D4364)
	// 826D4300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D430C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4314: 38AAC164  addi r5, r10, -0x3e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -16028;
	// 826D4318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D431C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4320: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826D4324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D432C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4334: 386ABBF4  addi r3, r10, -0x440c
	ctx.r[3].s64 = ctx.r[10].s64 + -17420;
	// 826D4338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D433C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D4344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D434C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4350: 4BD92AD1  bl 0x82466e20
	ctx.lr = 0x826D4354;
	sub_82466E20(ctx, base);
	// 826D4354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4368 size=112
    let mut pc: u32 = 0x826D4368;
    'dispatch: loop {
        match pc {
            0x826D4368 => {
    //   block [0x826D4368..0x826D43D8)
	// 826D4368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D436C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4374: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4378: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826D437C: 38EA0D90  addi r7, r10, 0xd90
	ctx.r[7].s64 = ctx.r[10].s64 + 3472;
	// 826D4380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4388: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826D438C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4390: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4394: 396B4520  addi r11, r11, 0x4520
	ctx.r[11].s64 = ctx.r[11].s64 + 17696;
	// 826D4398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D439C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D43A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D43A4: 386ABC24  addi r3, r10, -0x43dc
	ctx.r[3].s64 = ctx.r[10].s64 + -17372;
	// 826D43A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D43AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D43B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D43B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D43B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D43BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D43C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D43C4: 4BD92A5D  bl 0x82466e20
	ctx.lr = 0x826D43C8;
	sub_82466E20(ctx, base);
	// 826D43C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D43CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D43D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D43D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D43D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D43D8 size=112
    let mut pc: u32 = 0x826D43D8;
    'dispatch: loop {
        match pc {
            0x826D43D8 => {
    //   block [0x826D43D8..0x826D4448)
	// 826D43D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D43DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D43E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D43E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D43E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D43EC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D43F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D43F4: 390B0EC8  addi r8, r11, 0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + 3784;
	// 826D43F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D43FC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826D4400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D440C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4410: 386ABC54  addi r3, r10, -0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17324;
	// 826D4414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D441C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D442C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4434: 4BD929ED  bl 0x82466e20
	ctx.lr = 0x826D4438;
	sub_82466E20(ctx, base);
	// 826D4438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D443C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4448 size=108
    let mut pc: u32 = 0x826D4448;
    'dispatch: loop {
        match pc {
            0x826D4448 => {
    //   block [0x826D4448..0x826D44B4)
	// 826D4448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4454: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D445C: 38EB0EF8  addi r7, r11, 0xef8
	ctx.r[7].s64 = ctx.r[11].s64 + 3832;
	// 826D4460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D4464: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826D4468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D446C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4478: 386ABC84  addi r3, r10, -0x437c
	ctx.r[3].s64 = ctx.r[10].s64 + -17276;
	// 826D447C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D448C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D449C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D44A0: 4BD92981  bl 0x82466e20
	ctx.lr = 0x826D44A4;
	sub_82466E20(ctx, base);
	// 826D44A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D44A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D44AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D44B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D44B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D44B8 size=112
    let mut pc: u32 = 0x826D44B8;
    'dispatch: loop {
        match pc {
            0x826D44B8 => {
    //   block [0x826D44B8..0x826D4528)
	// 826D44B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D44BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D44C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D44C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D44C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D44CC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D44D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D44D4: 390B0F28  addi r8, r11, 0xf28
	ctx.r[8].s64 = ctx.r[11].s64 + 3880;
	// 826D44D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D44DC: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826D44E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D44E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D44E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D44EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D44F0: 386ABCB4  addi r3, r10, -0x434c
	ctx.r[3].s64 = ctx.r[10].s64 + -17228;
	// 826D44F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D44F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D44FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D450C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4514: 4BD9290D  bl 0x82466e20
	ctx.lr = 0x826D4518;
	sub_82466E20(ctx, base);
	// 826D4518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D451C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4528 size=112
    let mut pc: u32 = 0x826D4528;
    'dispatch: loop {
        match pc {
            0x826D4528 => {
    //   block [0x826D4528..0x826D4598)
	// 826D4528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4534: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4538: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D453C: 38EA0F40  addi r7, r10, 0xf40
	ctx.r[7].s64 = ctx.r[10].s64 + 3904;
	// 826D4540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4544: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4548: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826D454C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4550: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4554: 396B4574  addi r11, r11, 0x4574
	ctx.r[11].s64 = ctx.r[11].s64 + 17780;
	// 826D4558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D455C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4564: 386ABCE4  addi r3, r10, -0x431c
	ctx.r[3].s64 = ctx.r[10].s64 + -17180;
	// 826D4568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D456C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4574: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4578: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D457C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4580: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4584: 4BD9289D  bl 0x82466e20
	ctx.lr = 0x826D4588;
	sub_82466E20(ctx, base);
	// 826D4588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D458C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4598 size=108
    let mut pc: u32 = 0x826D4598;
    'dispatch: loop {
        match pc {
            0x826D4598 => {
    //   block [0x826D4598..0x826D4604)
	// 826D4598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D45A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D45A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D45A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D45AC: 38EB1018  addi r7, r11, 0x1018
	ctx.r[7].s64 = ctx.r[11].s64 + 4120;
	// 826D45B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D45B4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826D45B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D45BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D45C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D45C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D45C8: 386ABD14  addi r3, r10, -0x42ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17132;
	// 826D45CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D45D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D45D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D45D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D45DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D45E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D45E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D45E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D45EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D45F0: 4BD92831  bl 0x82466e20
	ctx.lr = 0x826D45F4;
	sub_82466E20(ctx, base);
	// 826D45F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D45F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D45FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4608 size=108
    let mut pc: u32 = 0x826D4608;
    'dispatch: loop {
        match pc {
            0x826D4608 => {
    //   block [0x826D4608..0x826D4674)
	// 826D4608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4614: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D461C: 38EB1030  addi r7, r11, 0x1030
	ctx.r[7].s64 = ctx.r[11].s64 + 4144;
	// 826D4620: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826D4624: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826D4628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D462C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4638: 386ABD44  addi r3, r10, -0x42bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17084;
	// 826D463C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D464C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D465C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4660: 4BD927C1  bl 0x82466e20
	ctx.lr = 0x826D4664;
	sub_82466E20(ctx, base);
	// 826D4664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D466C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4678 size=108
    let mut pc: u32 = 0x826D4678;
    'dispatch: loop {
        match pc {
            0x826D4678 => {
    //   block [0x826D4678..0x826D46E4)
	// 826D4678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4684: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D468C: 38EB1138  addi r7, r11, 0x1138
	ctx.r[7].s64 = ctx.r[11].s64 + 4408;
	// 826D4690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D4694: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826D4698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D469C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D46A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D46A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D46A8: 386ABD74  addi r3, r10, -0x428c
	ctx.r[3].s64 = ctx.r[10].s64 + -17036;
	// 826D46AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D46B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D46B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D46B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D46BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D46C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D46C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D46C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D46CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D46D0: 4BD92751  bl 0x82466e20
	ctx.lr = 0x826D46D4;
	sub_82466E20(ctx, base);
	// 826D46D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D46D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D46DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D46E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D46E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D46E8 size=112
    let mut pc: u32 = 0x826D46E8;
    'dispatch: loop {
        match pc {
            0x826D46E8 => {
    //   block [0x826D46E8..0x826D4758)
	// 826D46E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D46EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D46F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D46F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D46F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D46FC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4704: 390B1198  addi r8, r11, 0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + 4504;
	// 826D4708: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826D470C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826D4710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4720: 386ABDA4  addi r3, r10, -0x425c
	ctx.r[3].s64 = ctx.r[10].s64 + -16988;
	// 826D4724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D472C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D473C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4744: 4BD926DD  bl 0x82466e20
	ctx.lr = 0x826D4748;
	sub_82466E20(ctx, base);
	// 826D4748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D474C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4758 size=112
    let mut pc: u32 = 0x826D4758;
    'dispatch: loop {
        match pc {
            0x826D4758 => {
    //   block [0x826D4758..0x826D47C8)
	// 826D4758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4768: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D476C: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4774: 390B12B8  addi r8, r11, 0x12b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4792;
	// 826D4778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D477C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826D4780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4790: 386ABDD4  addi r3, r10, -0x422c
	ctx.r[3].s64 = ctx.r[10].s64 + -16940;
	// 826D4794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D479C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D47A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D47A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D47A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D47AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D47B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D47B4: 4BD9266D  bl 0x82466e20
	ctx.lr = 0x826D47B8;
	sub_82466E20(ctx, base);
	// 826D47B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D47BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D47C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D47C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D47C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D47C8 size=116
    let mut pc: u32 = 0x826D47C8;
    'dispatch: loop {
        match pc {
            0x826D47C8 => {
    //   block [0x826D47C8..0x826D483C)
	// 826D47C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D47CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D47D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D47D4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D47D8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D47DC: 390A12D0  addi r8, r10, 0x12d0
	ctx.r[8].s64 = ctx.r[10].s64 + 4816;
	// 826D47E0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D47E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D47E8: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D47EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D47F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D47F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D47F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D47FC: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826D4800: 396B45A4  addi r11, r11, 0x45a4
	ctx.r[11].s64 = ctx.r[11].s64 + 17828;
	// 826D4804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D480C: 386ABE04  addi r3, r10, -0x41fc
	ctx.r[3].s64 = ctx.r[10].s64 + -16892;
	// 826D4810: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4818: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D481C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4828: 4BD925F9  bl 0x82466e20
	ctx.lr = 0x826D482C;
	sub_82466E20(ctx, base);
	// 826D482C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4840 size=108
    let mut pc: u32 = 0x826D4840;
    'dispatch: loop {
        match pc {
            0x826D4840 => {
    //   block [0x826D4840..0x826D48AC)
	// 826D4840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D484C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D4854: 38EB1330  addi r7, r11, 0x1330
	ctx.r[7].s64 = ctx.r[11].s64 + 4912;
	// 826D4858: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D485C: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826D4860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D486C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4870: 386ABE34  addi r3, r10, -0x41cc
	ctx.r[3].s64 = ctx.r[10].s64 + -16844;
	// 826D4874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D487C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D488C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4898: 4BD92589  bl 0x82466e20
	ctx.lr = 0x826D489C;
	sub_82466E20(ctx, base);
	// 826D489C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D48A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D48A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D48A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D48B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D48B0 size=112
    let mut pc: u32 = 0x826D48B0;
    'dispatch: loop {
        match pc {
            0x826D48B0 => {
    //   block [0x826D48B0..0x826D4920)
	// 826D48B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D48B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D48B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D48BC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D48C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D48C4: 38EA13D8  addi r7, r10, 0x13d8
	ctx.r[7].s64 = ctx.r[10].s64 + 5080;
	// 826D48C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D48CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D48D0: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826D48D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D48D8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D48DC: 396B45B8  addi r11, r11, 0x45b8
	ctx.r[11].s64 = ctx.r[11].s64 + 17848;
	// 826D48E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D48E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D48E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D48EC: 386ABE64  addi r3, r10, -0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + -16796;
	// 826D48F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D48F4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D48F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D48FC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4904: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D490C: 4BD92515  bl 0x82466e20
	ctx.lr = 0x826D4910;
	sub_82466E20(ctx, base);
	// 826D4910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D491C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4920 size=112
    let mut pc: u32 = 0x826D4920;
    'dispatch: loop {
        match pc {
            0x826D4920 => {
    //   block [0x826D4920..0x826D4990)
	// 826D4920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D492C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4930: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4934: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D493C: 390B1450  addi r8, r11, 0x1450
	ctx.r[8].s64 = ctx.r[11].s64 + 5200;
	// 826D4940: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D4944: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826D4948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D494C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4958: 386ABE94  addi r3, r10, -0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + -16748;
	// 826D495C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D496C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D497C: 4BD924A5  bl 0x82466e20
	ctx.lr = 0x826D4980;
	sub_82466E20(ctx, base);
	// 826D4980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D498C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4990 size=112
    let mut pc: u32 = 0x826D4990;
    'dispatch: loop {
        match pc {
            0x826D4990 => {
    //   block [0x826D4990..0x826D4A00)
	// 826D4990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D499C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D49A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D49A4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D49A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D49AC: 390B1498  addi r8, r11, 0x1498
	ctx.r[8].s64 = ctx.r[11].s64 + 5272;
	// 826D49B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826D49B4: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826D49B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D49BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D49C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D49C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D49C8: 386ABEC4  addi r3, r10, -0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + -16700;
	// 826D49CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D49D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D49D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D49D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D49DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D49E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D49E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D49E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D49EC: 4BD92435  bl 0x82466e20
	ctx.lr = 0x826D49F0;
	sub_82466E20(ctx, base);
	// 826D49F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D49F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D49F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D49FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4A00 size=112
    let mut pc: u32 = 0x826D4A00;
    'dispatch: loop {
        match pc {
            0x826D4A00 => {
    //   block [0x826D4A00..0x826D4A70)
	// 826D4A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4A0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4A14: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4A1C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826D4A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4A24: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826D4A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4A38: 386ABEF4  addi r3, r10, -0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + -16652;
	// 826D4A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4A5C: 4BD923C5  bl 0x82466e20
	ctx.lr = 0x826D4A60;
	sub_82466E20(ctx, base);
	// 826D4A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4A70 size=112
    let mut pc: u32 = 0x826D4A70;
    'dispatch: loop {
        match pc {
            0x826D4A70 => {
    //   block [0x826D4A70..0x826D4AE0)
	// 826D4A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4A7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4A84: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D4A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4A8C: 390B15B8  addi r8, r11, 0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5560;
	// 826D4A90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D4A94: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826D4A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4A9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4AA8: 386ABF24  addi r3, r10, -0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + -16604;
	// 826D4AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4ACC: 4BD92355  bl 0x82466e20
	ctx.lr = 0x826D4AD0;
	sub_82466E20(ctx, base);
	// 826D4AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4AE0 size=112
    let mut pc: u32 = 0x826D4AE0;
    'dispatch: loop {
        match pc {
            0x826D4AE0 => {
    //   block [0x826D4AE0..0x826D4B50)
	// 826D4AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4AEC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4AF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D4AF4: 38EA15E8  addi r7, r10, 0x15e8
	ctx.r[7].s64 = ctx.r[10].s64 + 5608;
	// 826D4AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4AFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4B00: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826D4B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4B08: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4B0C: 396B45D8  addi r11, r11, 0x45d8
	ctx.r[11].s64 = ctx.r[11].s64 + 17880;
	// 826D4B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4B14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4B1C: 386ABF54  addi r3, r10, -0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + -16556;
	// 826D4B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4B24: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4B28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4B2C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4B30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4B38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4B3C: 4BD922E5  bl 0x82466e20
	ctx.lr = 0x826D4B40;
	sub_82466E20(ctx, base);
	// 826D4B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4B50 size=24
    let mut pc: u32 = 0x826D4B50;
    'dispatch: loop {
        match pc {
            0x826D4B50 => {
    //   block [0x826D4B50..0x826D4B68)
	// 826D4B50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4B54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4B58: 394A9878  addi r10, r10, -0x6788
	ctx.r[10].s64 = ctx.r[10].s64 + -26504;
	// 826D4B5C: 816B1660  lwz r11, 0x1660(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5728 as u32) ) } as u64;
	// 826D4B60: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D4B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4B68 size=116
    let mut pc: u32 = 0x826D4B68;
    'dispatch: loop {
        match pc {
            0x826D4B68 => {
    //   block [0x826D4B68..0x826D4BDC)
	// 826D4B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4B74: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4B78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D4B7C: 390B9878  addi r8, r11, -0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + -26504;
	// 826D4B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4B84: 392A4618  addi r9, r10, 0x4618
	ctx.r[9].s64 = ctx.r[10].s64 + 17944;
	// 826D4B88: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4B8C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D4B90: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4B94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4B9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4BAC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D4BB0: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826D4BB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4BB8: 386BBF84  addi r3, r11, -0x407c
	ctx.r[3].s64 = ctx.r[11].s64 + -16508;
	// 826D4BBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4BC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4BC8: 4BD92259  bl 0x82466e20
	ctx.lr = 0x826D4BCC;
	sub_82466E20(ctx, base);
	// 826D4BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4BE0 size=112
    let mut pc: u32 = 0x826D4BE0;
    'dispatch: loop {
        match pc {
            0x826D4BE0 => {
    //   block [0x826D4BE0..0x826D4C50)
	// 826D4BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4BF0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4BF4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4BFC: 390B1664  addi r8, r11, 0x1664
	ctx.r[8].s64 = ctx.r[11].s64 + 5732;
	// 826D4C00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D4C04: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826D4C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4C0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4C18: 386ABFB4  addi r3, r10, -0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + -16460;
	// 826D4C1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4C3C: 4BD921E5  bl 0x82466e20
	ctx.lr = 0x826D4C40;
	sub_82466E20(ctx, base);
	// 826D4C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4C50 size=116
    let mut pc: u32 = 0x826D4C50;
    'dispatch: loop {
        match pc {
            0x826D4C50 => {
    //   block [0x826D4C50..0x826D4CC4)
	// 826D4C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4C5C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4C60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D4C64: 390A1698  addi r8, r10, 0x1698
	ctx.r[8].s64 = ctx.r[10].s64 + 5784;
	// 826D4C68: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4C70: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4C74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4C78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4C84: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826D4C88: 396B462C  addi r11, r11, 0x462c
	ctx.r[11].s64 = ctx.r[11].s64 + 17964;
	// 826D4C8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4C94: 386ABFE4  addi r3, r10, -0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + -16412;
	// 826D4C98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4CA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4CB0: 4BD92171  bl 0x82466e20
	ctx.lr = 0x826D4CB4;
	sub_82466E20(ctx, base);
	// 826D4CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4CC8 size=112
    let mut pc: u32 = 0x826D4CC8;
    'dispatch: loop {
        match pc {
            0x826D4CC8 => {
    //   block [0x826D4CC8..0x826D4D38)
	// 826D4CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4CD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4CDC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4CE4: 390B1758  addi r8, r11, 0x1758
	ctx.r[8].s64 = ctx.r[11].s64 + 5976;
	// 826D4CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4CEC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826D4CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4D00: 386AC014  addi r3, r10, -0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + -16364;
	// 826D4D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4D24: 4BD920FD  bl 0x82466e20
	ctx.lr = 0x826D4D28;
	sub_82466E20(ctx, base);
	// 826D4D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4D38 size=112
    let mut pc: u32 = 0x826D4D38;
    'dispatch: loop {
        match pc {
            0x826D4D38 => {
    //   block [0x826D4D38..0x826D4DA8)
	// 826D4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4D44: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4D48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D4D4C: 38EA1770  addi r7, r10, 0x1770
	ctx.r[7].s64 = ctx.r[10].s64 + 6000;
	// 826D4D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4D54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4D58: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826D4D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4D60: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4D64: 396B4650  addi r11, r11, 0x4650
	ctx.r[11].s64 = ctx.r[11].s64 + 18000;
	// 826D4D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4D6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4D70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4D74: 386AC044  addi r3, r10, -0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -16316;
	// 826D4D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4D7C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4D84: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4D88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4D8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4D90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4D94: 4BD9208D  bl 0x82466e20
	ctx.lr = 0x826D4D98;
	sub_82466E20(ctx, base);
	// 826D4D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4DA8 size=116
    let mut pc: u32 = 0x826D4DA8;
    'dispatch: loop {
        match pc {
            0x826D4DA8 => {
    //   block [0x826D4DA8..0x826D4E1C)
	// 826D4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4DB4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4DB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D4DBC: 390A17A0  addi r8, r10, 0x17a0
	ctx.r[8].s64 = ctx.r[10].s64 + 6048;
	// 826D4DC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4DC8: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4DDC: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826D4DE0: 396B465C  addi r11, r11, 0x465c
	ctx.r[11].s64 = ctx.r[11].s64 + 18012;
	// 826D4DE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4DEC: 386AC074  addi r3, r10, -0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16268;
	// 826D4DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4E08: 4BD92019  bl 0x82466e20
	ctx.lr = 0x826D4E0C;
	sub_82466E20(ctx, base);
	// 826D4E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4E20 size=112
    let mut pc: u32 = 0x826D4E20;
    'dispatch: loop {
        match pc {
            0x826D4E20 => {
    //   block [0x826D4E20..0x826D4E90)
	// 826D4E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4E30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4E34: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4E3C: 390B1848  addi r8, r11, 0x1848
	ctx.r[8].s64 = ctx.r[11].s64 + 6216;
	// 826D4E40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4E44: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826D4E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4E58: 386AC0A4  addi r3, r10, -0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -16220;
	// 826D4E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4E7C: 4BD91FA5  bl 0x82466e20
	ctx.lr = 0x826D4E80;
	sub_82466E20(ctx, base);
	// 826D4E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4E90 size=112
    let mut pc: u32 = 0x826D4E90;
    'dispatch: loop {
        match pc {
            0x826D4E90 => {
    //   block [0x826D4E90..0x826D4F00)
	// 826D4E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4E9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4EA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4EA4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4EAC: 390B1860  addi r8, r11, 0x1860
	ctx.r[8].s64 = ctx.r[11].s64 + 6240;
	// 826D4EB0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826D4EB4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826D4EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4EBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4EC8: 386AC0D4  addi r3, r10, -0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -16172;
	// 826D4ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4EEC: 4BD91F35  bl 0x82466e20
	ctx.lr = 0x826D4EF0;
	sub_82466E20(ctx, base);
	// 826D4EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4F00 size=112
    let mut pc: u32 = 0x826D4F00;
    'dispatch: loop {
        match pc {
            0x826D4F00 => {
    //   block [0x826D4F00..0x826D4F70)
	// 826D4F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4F0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4F14: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4F1C: 390B1998  addi r8, r11, 0x1998
	ctx.r[8].s64 = ctx.r[11].s64 + 6552;
	// 826D4F20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4F24: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826D4F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4F38: 386AC104  addi r3, r10, -0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + -16124;
	// 826D4F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4F5C: 4BD91EC5  bl 0x82466e20
	ctx.lr = 0x826D4F60;
	sub_82466E20(ctx, base);
	// 826D4F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4F70 size=112
    let mut pc: u32 = 0x826D4F70;
    'dispatch: loop {
        match pc {
            0x826D4F70 => {
    //   block [0x826D4F70..0x826D4FE0)
	// 826D4F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4F7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4F84: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4F88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D4F8C: 390B19B0  addi r8, r11, 0x19b0
	ctx.r[8].s64 = ctx.r[11].s64 + 6576;
	// 826D4F90: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D4F94: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826D4F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4F9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4FA8: 386AC134  addi r3, r10, -0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -16076;
	// 826D4FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4FCC: 4BD91E55  bl 0x82466e20
	ctx.lr = 0x826D4FD0;
	sub_82466E20(ctx, base);
	// 826D4FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4FE0 size=116
    let mut pc: u32 = 0x826D4FE0;
    'dispatch: loop {
        match pc {
            0x826D4FE0 => {
    //   block [0x826D4FE0..0x826D5054)
	// 826D4FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4FEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4FF0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D4FF4: 390B1A40  addi r8, r11, 0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + 6720;
	// 826D4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4FFC: 392A4694  addi r9, r10, 0x4694
	ctx.r[9].s64 = ctx.r[10].s64 + 18068;
	// 826D5000: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5004: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5008: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D500C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5024: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5028: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826D502C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5030: 386BC164  addi r3, r11, -0x3e9c
	ctx.r[3].s64 = ctx.r[11].s64 + -16028;
	// 826D5034: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5038: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D503C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5040: 4BD91DE1  bl 0x82466e20
	ctx.lr = 0x826D5044;
	sub_82466E20(ctx, base);
	// 826D5044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D504C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5058 size=100
    let mut pc: u32 = 0x826D5058;
    'dispatch: loop {
        match pc {
            0x826D5058 => {
    //   block [0x826D5058..0x826D50BC)
	// 826D5058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D505C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D506C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5078: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826D507C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D508C: 386AC194  addi r3, r10, -0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15980;
	// 826D5090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D509C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D50A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D50A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D50A8: 4BD91D79  bl 0x82466e20
	ctx.lr = 0x826D50AC;
	sub_82466E20(ctx, base);
	// 826D50AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D50B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D50B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D50B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D50C0 size=112
    let mut pc: u32 = 0x826D50C0;
    'dispatch: loop {
        match pc {
            0x826D50C0 => {
    //   block [0x826D50C0..0x826D5130)
	// 826D50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D50C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D50C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D50CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D50D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D50D4: 38AAC194  addi r5, r10, -0x3e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -15980;
	// 826D50D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D50DC: 390B1A70  addi r8, r11, 0x1a70
	ctx.r[8].s64 = ctx.r[11].s64 + 6768;
	// 826D50E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D50E4: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826D50E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D50EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D50F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D50F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D50F8: 386AC1C4  addi r3, r10, -0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15932;
	// 826D50FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D510C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D511C: 4BD91D05  bl 0x82466e20
	ctx.lr = 0x826D5120;
	sub_82466E20(ctx, base);
	// 826D5120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5130 size=112
    let mut pc: u32 = 0x826D5130;
    'dispatch: loop {
        match pc {
            0x826D5130 => {
    //   block [0x826D5130..0x826D51A0)
	// 826D5130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D513C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5140: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5144: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D514C: 390B1A88  addi r8, r11, 0x1a88
	ctx.r[8].s64 = ctx.r[11].s64 + 6792;
	// 826D5150: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D5154: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826D5158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D515C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5168: 386AC1F4  addi r3, r10, -0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15884;
	// 826D516C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D517C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D518C: 4BD91C95  bl 0x82466e20
	ctx.lr = 0x826D5190;
	sub_82466E20(ctx, base);
	// 826D5190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D519C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D51A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D51A0 size=112
    let mut pc: u32 = 0x826D51A0;
    'dispatch: loop {
        match pc {
            0x826D51A0 => {
    //   block [0x826D51A0..0x826D5210)
	// 826D51A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D51A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D51A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D51AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D51B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D51B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D51B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D51BC: 390B1B30  addi r8, r11, 0x1b30
	ctx.r[8].s64 = ctx.r[11].s64 + 6960;
	// 826D51C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D51C4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826D51C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D51CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D51D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D51D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D51D8: 386AC224  addi r3, r10, -0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -15836;
	// 826D51DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D51E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D51E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D51E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D51EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D51F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D51F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D51F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D51FC: 4BD91C25  bl 0x82466e20
	ctx.lr = 0x826D5200;
	sub_82466E20(ctx, base);
	// 826D5200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D520C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5210 size=112
    let mut pc: u32 = 0x826D5210;
    'dispatch: loop {
        match pc {
            0x826D5210 => {
    //   block [0x826D5210..0x826D5280)
	// 826D5210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D521C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5220: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5224: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D522C: 390B1B78  addi r8, r11, 0x1b78
	ctx.r[8].s64 = ctx.r[11].s64 + 7032;
	// 826D5230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D5234: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826D5238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D523C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5248: 386AC254  addi r3, r10, -0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + -15788;
	// 826D524C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D525C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D526C: 4BD91BB5  bl 0x82466e20
	ctx.lr = 0x826D5270;
	sub_82466E20(ctx, base);
	// 826D5270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D527C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5280 size=116
    let mut pc: u32 = 0x826D5280;
    'dispatch: loop {
        match pc {
            0x826D5280 => {
    //   block [0x826D5280..0x826D52F4)
	// 826D5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D528C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D5290: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D5294: 390A1BA8  addi r8, r10, 0x1ba8
	ctx.r[8].s64 = ctx.r[10].s64 + 7080;
	// 826D5298: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D529C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D52A0: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D52A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D52A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D52AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D52B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D52B4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826D52B8: 396B46A8  addi r11, r11, 0x46a8
	ctx.r[11].s64 = ctx.r[11].s64 + 18088;
	// 826D52BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D52C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D52C4: 386AC284  addi r3, r10, -0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15740;
	// 826D52C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D52CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D52D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D52D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D52D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D52DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D52E0: 4BD91B41  bl 0x82466e20
	ctx.lr = 0x826D52E4;
	sub_82466E20(ctx, base);
	// 826D52E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D52E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D52EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D52F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D52F8 size=100
    let mut pc: u32 = 0x826D52F8;
    'dispatch: loop {
        match pc {
            0x826D52F8 => {
    //   block [0x826D52F8..0x826D535C)
	// 826D52F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D52FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D530C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5318: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826D531C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D532C: 386AC2B4  addi r3, r10, -0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15692;
	// 826D5330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5338: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D533C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5340: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D5344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5348: 4BD91AD9  bl 0x82466e20
	ctx.lr = 0x826D534C;
	sub_82466E20(ctx, base);
	// 826D534C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5360 size=108
    let mut pc: u32 = 0x826D5360;
    'dispatch: loop {
        match pc {
            0x826D5360 => {
    //   block [0x826D5360..0x826D53CC)
	// 826D5360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D536C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5374: 38EB1C68  addi r7, r11, 0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + 7272;
	// 826D5378: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D537C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826D5380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5390: 386AC2E4  addi r3, r10, -0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15644;
	// 826D5394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D53A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D53A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D53A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D53AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D53B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D53B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D53B8: 4BD91A69  bl 0x82466e20
	ctx.lr = 0x826D53BC;
	sub_82466E20(ctx, base);
	// 826D53BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D53C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D53C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D53C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D53D0 size=112
    let mut pc: u32 = 0x826D53D0;
    'dispatch: loop {
        match pc {
            0x826D53D0 => {
    //   block [0x826D53D0..0x826D5440)
	// 826D53D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D53D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D53D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D53DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D53E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D53E4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D53E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D53EC: 390B1C98  addi r8, r11, 0x1c98
	ctx.r[8].s64 = ctx.r[11].s64 + 7320;
	// 826D53F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D53F4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826D53F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D53FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5408: 386AC314  addi r3, r10, -0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + -15596;
	// 826D540C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D541C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D542C: 4BD919F5  bl 0x82466e20
	ctx.lr = 0x826D5430;
	sub_82466E20(ctx, base);
	// 826D5430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D543C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5440 size=108
    let mut pc: u32 = 0x826D5440;
    'dispatch: loop {
        match pc {
            0x826D5440 => {
    //   block [0x826D5440..0x826D54AC)
	// 826D5440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D544C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5454: 38EB1CC8  addi r7, r11, 0x1cc8
	ctx.r[7].s64 = ctx.r[11].s64 + 7368;
	// 826D5458: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D545C: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 826D5460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D546C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5470: 386AC344  addi r3, r10, -0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15548;
	// 826D5474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D547C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D548C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5498: 4BD91989  bl 0x82466e20
	ctx.lr = 0x826D549C;
	sub_82466E20(ctx, base);
	// 826D549C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D54A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D54A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D54A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D54B0 size=112
    let mut pc: u32 = 0x826D54B0;
    'dispatch: loop {
        match pc {
            0x826D54B0 => {
    //   block [0x826D54B0..0x826D5520)
	// 826D54B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D54B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D54B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D54BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D54C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D54C4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D54C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D54CC: 390B1CF8  addi r8, r11, 0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 7416;
	// 826D54D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D54D4: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826D54D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D54DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D54E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D54E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D54E8: 386AC374  addi r3, r10, -0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15500;
	// 826D54EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D54F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D54F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D54F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D54FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D550C: 4BD91915  bl 0x82466e20
	ctx.lr = 0x826D5510;
	sub_82466E20(ctx, base);
	// 826D5510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D551C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5520 size=108
    let mut pc: u32 = 0x826D5520;
    'dispatch: loop {
        match pc {
            0x826D5520 => {
    //   block [0x826D5520..0x826D558C)
	// 826D5520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D552C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5534: 38EB1D40  addi r7, r11, 0x1d40
	ctx.r[7].s64 = ctx.r[11].s64 + 7488;
	// 826D5538: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D553C: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 826D5540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D554C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5550: 386AC3A4  addi r3, r10, -0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15452;
	// 826D5554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D555C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D556C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5578: 4BD918A9  bl 0x82466e20
	ctx.lr = 0x826D557C;
	sub_82466E20(ctx, base);
	// 826D557C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5590 size=112
    let mut pc: u32 = 0x826D5590;
    'dispatch: loop {
        match pc {
            0x826D5590 => {
    //   block [0x826D5590..0x826D5600)
	// 826D5590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D559C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D55A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D55A4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D55A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D55AC: 390B1D70  addi r8, r11, 0x1d70
	ctx.r[8].s64 = ctx.r[11].s64 + 7536;
	// 826D55B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D55B4: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826D55B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D55BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D55C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D55C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D55C8: 386AC3D4  addi r3, r10, -0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15404;
	// 826D55CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D55D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D55D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D55D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D55DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D55E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D55E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D55E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D55EC: 4BD91835  bl 0x82466e20
	ctx.lr = 0x826D55F0;
	sub_82466E20(ctx, base);
	// 826D55F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D55F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D55F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D55FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5600 size=108
    let mut pc: u32 = 0x826D5600;
    'dispatch: loop {
        match pc {
            0x826D5600 => {
    //   block [0x826D5600..0x826D566C)
	// 826D5600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D560C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5610: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5614: 38EB1DB8  addi r7, r11, 0x1db8
	ctx.r[7].s64 = ctx.r[11].s64 + 7608;
	// 826D5618: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D561C: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 826D5620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5628: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D562C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5630: 386AC404  addi r3, r10, -0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15356;
	// 826D5634: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D563C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D564C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5658: 4BD917C9  bl 0x82466e20
	ctx.lr = 0x826D565C;
	sub_82466E20(ctx, base);
	// 826D565C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5670 size=112
    let mut pc: u32 = 0x826D5670;
    'dispatch: loop {
        match pc {
            0x826D5670 => {
    //   block [0x826D5670..0x826D56E0)
	// 826D5670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D567C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5680: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5684: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D5688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D568C: 390B1DE8  addi r8, r11, 0x1de8
	ctx.r[8].s64 = ctx.r[11].s64 + 7656;
	// 826D5690: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D5694: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826D5698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D569C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D56A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D56A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D56A8: 386AC434  addi r3, r10, -0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -15308;
	// 826D56AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D56B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D56B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D56B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D56BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D56C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D56C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D56C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D56CC: 4BD91755  bl 0x82466e20
	ctx.lr = 0x826D56D0;
	sub_82466E20(ctx, base);
	// 826D56D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D56D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D56D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D56DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D56E0 size=108
    let mut pc: u32 = 0x826D56E0;
    'dispatch: loop {
        match pc {
            0x826D56E0 => {
    //   block [0x826D56E0..0x826D574C)
	// 826D56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D56E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D56E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D56EC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D56F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D56F4: 38EB1E38  addi r7, r11, 0x1e38
	ctx.r[7].s64 = ctx.r[11].s64 + 7736;
	// 826D56F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D56FC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826D5700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5704: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D570C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5710: 386AC464  addi r3, r10, -0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15260;
	// 826D5714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D571C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5738: 4BD916E9  bl 0x82466e20
	ctx.lr = 0x826D573C;
	sub_82466E20(ctx, base);
	// 826D573C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5750 size=24
    let mut pc: u32 = 0x826D5750;
    'dispatch: loop {
        match pc {
            0x826D5750 => {
    //   block [0x826D5750..0x826D5768)
	// 826D5750: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5754: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5758: 394A9980  addi r10, r10, -0x6680
	ctx.r[10].s64 = ctx.r[10].s64 + -26240;
	// 826D575C: 816B1E34  lwz r11, 0x1e34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7732 as u32) ) } as u64;
	// 826D5760: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826D5764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5768 size=112
    let mut pc: u32 = 0x826D5768;
    'dispatch: loop {
        match pc {
            0x826D5768 => {
    //   block [0x826D5768..0x826D57D8)
	// 826D5768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5774: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D577C: 392A4790  addi r9, r10, 0x4790
	ctx.r[9].s64 = ctx.r[10].s64 + 18320;
	// 826D5780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5784: 390B9980  addi r8, r11, -0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + -26240;
	// 826D5788: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D578C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826D5790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D579C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D57A0: 386AC494  addi r3, r10, -0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15212;
	// 826D57A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D57A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D57AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D57B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D57B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D57B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D57BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D57C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D57C4: 4BD9165D  bl 0x82466e20
	ctx.lr = 0x826D57C8;
	sub_82466E20(ctx, base);
	// 826D57C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D57CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D57D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D57D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D57D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D57D8 size=108
    let mut pc: u32 = 0x826D57D8;
    'dispatch: loop {
        match pc {
            0x826D57D8 => {
    //   block [0x826D57D8..0x826D5844)
	// 826D57D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D57DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D57E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D57E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D57E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D57EC: 38EB1EA0  addi r7, r11, 0x1ea0
	ctx.r[7].s64 = ctx.r[11].s64 + 7840;
	// 826D57F0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D57F4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826D57F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D57FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5808: 386AC4C4  addi r3, r10, -0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15164;
	// 826D580C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D582C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5830: 4BD915F1  bl 0x82466e20
	ctx.lr = 0x826D5834;
	sub_82466E20(ctx, base);
	// 826D5834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D583C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5848 size=108
    let mut pc: u32 = 0x826D5848;
    'dispatch: loop {
        match pc {
            0x826D5848 => {
    //   block [0x826D5848..0x826D58B4)
	// 826D5848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D584C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5854: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D585C: 38EB1F18  addi r7, r11, 0x1f18
	ctx.r[7].s64 = ctx.r[11].s64 + 7960;
	// 826D5860: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D5864: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 826D5868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D586C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5878: 386AC4F4  addi r3, r10, -0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15116;
	// 826D587C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D588C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D589C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D58A0: 4BD91581  bl 0x82466e20
	ctx.lr = 0x826D58A4;
	sub_82466E20(ctx, base);
	// 826D58A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D58A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D58AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D58B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D58B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D58B8 size=108
    let mut pc: u32 = 0x826D58B8;
    'dispatch: loop {
        match pc {
            0x826D58B8 => {
    //   block [0x826D58B8..0x826D5924)
	// 826D58B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D58BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D58C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D58C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D58C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D58CC: 38EB1F78  addi r7, r11, 0x1f78
	ctx.r[7].s64 = ctx.r[11].s64 + 8056;
	// 826D58D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D58D4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826D58D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D58DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D58E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D58E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D58E8: 386AC524  addi r3, r10, -0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + -15068;
	// 826D58EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D58F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D58F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D58F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D58FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D590C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5910: 4BD91511  bl 0x82466e20
	ctx.lr = 0x826D5914;
	sub_82466E20(ctx, base);
	// 826D5914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5928 size=24
    let mut pc: u32 = 0x826D5928;
    'dispatch: loop {
        match pc {
            0x826D5928 => {
    //   block [0x826D5928..0x826D5940)
	// 826D5928: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D592C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5930: 394A9A88  addi r10, r10, -0x6578
	ctx.r[10].s64 = ctx.r[10].s64 + -25976;
	// 826D5934: 816B1E30  lwz r11, 0x1e30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7728 as u32) ) } as u64;
	// 826D5938: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826D593C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5940 size=116
    let mut pc: u32 = 0x826D5940;
    'dispatch: loop {
        match pc {
            0x826D5940 => {
    //   block [0x826D5940..0x826D59B4)
	// 826D5940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D594C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D5950: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5954: 392B46F0  addi r9, r11, 0x46f0
	ctx.r[9].s64 = ctx.r[11].s64 + 18160;
	// 826D5958: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D595C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5960: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 826D5964: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 826D5968: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D596C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826D5970: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5974: 396B9A88  addi r11, r11, -0x6578
	ctx.r[11].s64 = ctx.r[11].s64 + -25976;
	// 826D5978: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D597C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5980: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D5984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5988: 386AC554  addi r3, r10, -0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + -15020;
	// 826D598C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5990: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D5994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5998: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D599C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D59A0: 4BD91481  bl 0x82466e20
	ctx.lr = 0x826D59A4;
	sub_82466E20(ctx, base);
	// 826D59A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D59A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D59AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D59B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D59B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D59B8 size=36
    let mut pc: u32 = 0x826D59B8;
    'dispatch: loop {
        match pc {
            0x826D59B8 => {
    //   block [0x826D59B8..0x826D59DC)
	// 826D59B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D59BC: 814B1E9C  lwz r10, 0x1e9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7836 as u32) ) } as u64;
	// 826D59C0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D59C4: 396B9D58  addi r11, r11, -0x62a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25256;
	// 826D59C8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D59CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D59D0: 814A2020  lwz r10, 0x2020(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8224 as u32) ) } as u64;
	// 826D59D4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D59D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D59E0 size=116
    let mut pc: u32 = 0x826D59E0;
    'dispatch: loop {
        match pc {
            0x826D59E0 => {
    //   block [0x826D59E0..0x826D5A54)
	// 826D59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D59EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D59F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D59F4: 390B9D58  addi r8, r11, -0x62a8
	ctx.r[8].s64 = ctx.r[11].s64 + -25256;
	// 826D59F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D59FC: 392A4878  addi r9, r10, 0x4878
	ctx.r[9].s64 = ctx.r[10].s64 + 18552;
	// 826D5A00: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5A04: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5A08: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D5A0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5A14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5A24: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5A28: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826D5A2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5A30: 386BC584  addi r3, r11, -0x3a7c
	ctx.r[3].s64 = ctx.r[11].s64 + -14972;
	// 826D5A34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D5A38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5A40: 4BD913E1  bl 0x82466e20
	ctx.lr = 0x826D5A44;
	sub_82466E20(ctx, base);
	// 826D5A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5A58 size=24
    let mut pc: u32 = 0x826D5A58;
    'dispatch: loop {
        match pc {
            0x826D5A58 => {
    //   block [0x826D5A58..0x826D5A70)
	// 826D5A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5A5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5A60: 394A9D88  addi r10, r10, -0x6278
	ctx.r[10].s64 = ctx.r[10].s64 + -25208;
	// 826D5A64: 816B2028  lwz r11, 0x2028(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8232 as u32) ) } as u64;
	// 826D5A68: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D5A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5A70 size=116
    let mut pc: u32 = 0x826D5A70;
    'dispatch: loop {
        match pc {
            0x826D5A70 => {
    //   block [0x826D5A70..0x826D5AE4)
	// 826D5A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5A7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D5A80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5A84: 390B9D88  addi r8, r11, -0x6278
	ctx.r[8].s64 = ctx.r[11].s64 + -25208;
	// 826D5A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5A8C: 392A48D0  addi r9, r10, 0x48d0
	ctx.r[9].s64 = ctx.r[10].s64 + 18640;
	// 826D5A90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5A94: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826D5A98: 38AAC584  addi r5, r10, -0x3a7c
	ctx.r[5].s64 = ctx.r[10].s64 + -14972;
	// 826D5A9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5AA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5AB4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5AB8: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826D5ABC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5AC0: 386BC5B4  addi r3, r11, -0x3a4c
	ctx.r[3].s64 = ctx.r[11].s64 + -14924;
	// 826D5AC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D5AC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5AD0: 4BD91351  bl 0x82466e20
	ctx.lr = 0x826D5AD4;
	sub_82466E20(ctx, base);
	// 826D5AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5AE8 size=112
    let mut pc: u32 = 0x826D5AE8;
    'dispatch: loop {
        match pc {
            0x826D5AE8 => {
    //   block [0x826D5AE8..0x826D5B58)
	// 826D5AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5AF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5AFC: 38AAC584  addi r5, r10, -0x3a7c
	ctx.r[5].s64 = ctx.r[10].s64 + -14972;
	// 826D5B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5B04: 390B2030  addi r8, r11, 0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + 8240;
	// 826D5B08: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D5B0C: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826D5B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5B14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5B20: 386AC5E4  addi r3, r10, -0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -14876;
	// 826D5B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5B44: 4BD912DD  bl 0x82466e20
	ctx.lr = 0x826D5B48;
	sub_82466E20(ctx, base);
	// 826D5B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5B58 size=112
    let mut pc: u32 = 0x826D5B58;
    'dispatch: loop {
        match pc {
            0x826D5B58 => {
    //   block [0x826D5B58..0x826D5BC8)
	// 826D5B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5B64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5B6C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5B74: 390B2120  addi r8, r11, 0x2120
	ctx.r[8].s64 = ctx.r[11].s64 + 8480;
	// 826D5B78: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826D5B7C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826D5B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5B90: 386AC614  addi r3, r10, -0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14828;
	// 826D5B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5BB4: 4BD9126D  bl 0x82466e20
	ctx.lr = 0x826D5BB8;
	sub_82466E20(ctx, base);
	// 826D5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5BC8 size=108
    let mut pc: u32 = 0x826D5BC8;
    'dispatch: loop {
        match pc {
            0x826D5BC8 => {
    //   block [0x826D5BC8..0x826D5C34)
	// 826D5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5BD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5BDC: 38EB21F8  addi r7, r11, 0x21f8
	ctx.r[7].s64 = ctx.r[11].s64 + 8696;
	// 826D5BE0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D5BE4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826D5BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5BF8: 386AC644  addi r3, r10, -0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14780;
	// 826D5BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5C20: 4BD91201  bl 0x82466e20
	ctx.lr = 0x826D5C24;
	sub_82466E20(ctx, base);
	// 826D5C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5C38 size=108
    let mut pc: u32 = 0x826D5C38;
    'dispatch: loop {
        match pc {
            0x826D5C38 => {
    //   block [0x826D5C38..0x826D5CA4)
	// 826D5C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5C44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5C4C: 38EB2270  addi r7, r11, 0x2270
	ctx.r[7].s64 = ctx.r[11].s64 + 8816;
	// 826D5C50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D5C54: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826D5C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5C60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5C68: 386AC674  addi r3, r10, -0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + -14732;
	// 826D5C6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5C90: 4BD91191  bl 0x82466e20
	ctx.lr = 0x826D5C94;
	sub_82466E20(ctx, base);
	// 826D5C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5CA8 size=112
    let mut pc: u32 = 0x826D5CA8;
    'dispatch: loop {
        match pc {
            0x826D5CA8 => {
    //   block [0x826D5CA8..0x826D5D18)
	// 826D5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5CB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5CB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5CBC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5CC4: 390B22B8  addi r8, r11, 0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8888;
	// 826D5CC8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 826D5CCC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826D5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5CE0: 386AC6A4  addi r3, r10, -0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + -14684;
	// 826D5CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5D04: 4BD9111D  bl 0x82466e20
	ctx.lr = 0x826D5D08;
	sub_82466E20(ctx, base);
	// 826D5D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5D18 size=108
    let mut pc: u32 = 0x826D5D18;
    'dispatch: loop {
        match pc {
            0x826D5D18 => {
    //   block [0x826D5D18..0x826D5D84)
	// 826D5D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5D24: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5D2C: 38EB2498  addi r7, r11, 0x2498
	ctx.r[7].s64 = ctx.r[11].s64 + 9368;
	// 826D5D30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D5D34: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826D5D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5D3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5D48: 386AC6D4  addi r3, r10, -0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + -14636;
	// 826D5D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5D70: 4BD910B1  bl 0x82466e20
	ctx.lr = 0x826D5D74;
	sub_82466E20(ctx, base);
	// 826D5D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5D88 size=24
    let mut pc: u32 = 0x826D5D88;
    'dispatch: loop {
        match pc {
            0x826D5D88 => {
    //   block [0x826D5D88..0x826D5DA0)
	// 826D5D88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5D8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5D90: 394A9EA8  addi r10, r10, -0x6158
	ctx.r[10].s64 = ctx.r[10].s64 + -24920;
	// 826D5D94: 816B202C  lwz r11, 0x202c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8236 as u32) ) } as u64;
	// 826D5D98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D5D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5DA0 size=112
    let mut pc: u32 = 0x826D5DA0;
    'dispatch: loop {
        match pc {
            0x826D5DA0 => {
    //   block [0x826D5DA0..0x826D5E10)
	// 826D5DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5DAC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5DB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D5DB4: 392A4928  addi r9, r10, 0x4928
	ctx.r[9].s64 = ctx.r[10].s64 + 18728;
	// 826D5DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5DBC: 390B9EA8  addi r8, r11, -0x6158
	ctx.r[8].s64 = ctx.r[11].s64 + -24920;
	// 826D5DC0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5DC4: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826D5DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5DD8: 386AC704  addi r3, r10, -0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14588;
	// 826D5DDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5DE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5DFC: 4BD91025  bl 0x82466e20
	ctx.lr = 0x826D5E00;
	sub_82466E20(ctx, base);
	// 826D5E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5E10 size=112
    let mut pc: u32 = 0x826D5E10;
    'dispatch: loop {
        match pc {
            0x826D5E10 => {
    //   block [0x826D5E10..0x826D5E80)
	// 826D5E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5E1C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D5E20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D5E24: 38EA24B0  addi r7, r10, 0x24b0
	ctx.r[7].s64 = ctx.r[10].s64 + 9392;
	// 826D5E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5E2C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D5E30: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826D5E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5E38: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5E3C: 396B493C  addi r11, r11, 0x493c
	ctx.r[11].s64 = ctx.r[11].s64 + 18748;
	// 826D5E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5E4C: 386AC734  addi r3, r10, -0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14540;
	// 826D5E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5E54: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D5E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5E5C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D5E60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5E64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5E68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5E6C: 4BD90FB5  bl 0x82466e20
	ctx.lr = 0x826D5E70;
	sub_82466E20(ctx, base);
	// 826D5E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5E80 size=112
    let mut pc: u32 = 0x826D5E80;
    'dispatch: loop {
        match pc {
            0x826D5E80 => {
    //   block [0x826D5E80..0x826D5EF0)
	// 826D5E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5E8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5E90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5E94: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5E9C: 390B2540  addi r8, r11, 0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + 9536;
	// 826D5EA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D5EA4: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826D5EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5EB8: 386AC764  addi r3, r10, -0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + -14492;
	// 826D5EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5EDC: 4BD90F45  bl 0x82466e20
	ctx.lr = 0x826D5EE0;
	sub_82466E20(ctx, base);
	// 826D5EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5EF0 size=108
    let mut pc: u32 = 0x826D5EF0;
    'dispatch: loop {
        match pc {
            0x826D5EF0 => {
    //   block [0x826D5EF0..0x826D5F5C)
	// 826D5EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5EFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5F04: 38EB2560  addi r7, r11, 0x2560
	ctx.r[7].s64 = ctx.r[11].s64 + 9568;
	// 826D5F08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D5F0C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826D5F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5F20: 386AC794  addi r3, r10, -0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + -14444;
	// 826D5F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5F48: 4BD90ED9  bl 0x82466e20
	ctx.lr = 0x826D5F4C;
	sub_82466E20(ctx, base);
	// 826D5F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5F60 size=108
    let mut pc: u32 = 0x826D5F60;
    'dispatch: loop {
        match pc {
            0x826D5F60 => {
    //   block [0x826D5F60..0x826D5FCC)
	// 826D5F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5F6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5F74: 38EB25C0  addi r7, r11, 0x25c0
	ctx.r[7].s64 = ctx.r[11].s64 + 9664;
	// 826D5F78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D5F7C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826D5F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5F90: 386AC7C4  addi r3, r10, -0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + -14396;
	// 826D5F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5FB8: 4BD90E69  bl 0x82466e20
	ctx.lr = 0x826D5FBC;
	sub_82466E20(ctx, base);
	// 826D5FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5FD0 size=116
    let mut pc: u32 = 0x826D5FD0;
    'dispatch: loop {
        match pc {
            0x826D5FD0 => {
    //   block [0x826D5FD0..0x826D6044)
	// 826D5FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5FDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5FE0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5FE4: 390B25F0  addi r8, r11, 0x25f0
	ctx.r[8].s64 = ctx.r[11].s64 + 9712;
	// 826D5FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5FEC: 392A4970  addi r9, r10, 0x4970
	ctx.r[9].s64 = ctx.r[10].s64 + 18800;
	// 826D5FF0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5FF4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D5FF8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5FFC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6014: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6018: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826D601C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6020: 386BC7F4  addi r3, r11, -0x380c
	ctx.r[3].s64 = ctx.r[11].s64 + -14348;
	// 826D6024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6030: 4BD90DF1  bl 0x82466e20
	ctx.lr = 0x826D6034;
	sub_82466E20(ctx, base);
	// 826D6034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6048 size=76
    let mut pc: u32 = 0x826D6048;
    'dispatch: loop {
        match pc {
            0x826D6048 => {
    //   block [0x826D6048..0x826D6094)
	// 826D6048: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D604C: 814B255C  lwz r10, 0x255c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9564 as u32) ) } as u64;
	// 826D6050: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6054: 396B9ED8  addi r11, r11, -0x6128
	ctx.r[11].s64 = ctx.r[11].s64 + -24872;
	// 826D6058: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826D605C: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826D6060: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D6064: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6068: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826D606C: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826D6070: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6074: 814A2608  lwz r10, 0x2608(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9736 as u32) ) } as u64;
	// 826D6078: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826D607C: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826D6080: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826D6084: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826D6088: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826D608C: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826D6090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6098 size=108
    let mut pc: u32 = 0x826D6098;
    'dispatch: loop {
        match pc {
            0x826D6098 => {
    //   block [0x826D6098..0x826D6104)
	// 826D6098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D60A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D60A4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D60A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D60AC: 38EB9ED8  addi r7, r11, -0x6128
	ctx.r[7].s64 = ctx.r[11].s64 + -24872;
	// 826D60B0: 39000019  li r8, 0x19
	ctx.r[8].s64 = 25;
	// 826D60B4: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826D60B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D60BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D60C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D60C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D60C8: 386AC824  addi r3, r10, -0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14300;
	// 826D60CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D60D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D60D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D60D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D60DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D60E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D60E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D60E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D60EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D60F0: 4BD90D31  bl 0x82466e20
	ctx.lr = 0x826D60F4;
	sub_82466E20(ctx, base);
	// 826D60F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D60F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D60FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6108 size=76
    let mut pc: u32 = 0x826D6108;
    'dispatch: loop {
        match pc {
            0x826D6108 => {
    //   block [0x826D6108..0x826D6154)
	// 826D6108: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D610C: 814B255C  lwz r10, 0x255c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9564 as u32) ) } as u64;
	// 826D6110: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6114: 396BA130  addi r11, r11, -0x5ed0
	ctx.r[11].s64 = ctx.r[11].s64 + -24272;
	// 826D6118: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826D611C: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826D6120: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D6124: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6128: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826D612C: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826D6130: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6134: 814A2608  lwz r10, 0x2608(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9736 as u32) ) } as u64;
	// 826D6138: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826D613C: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826D6140: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826D6144: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826D6148: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826D614C: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826D6150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6158 size=116
    let mut pc: u32 = 0x826D6158;
    'dispatch: loop {
        match pc {
            0x826D6158 => {
    //   block [0x826D6158..0x826D61CC)
	// 826D6158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D615C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6164: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6168: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D616C: 390BA130  addi r8, r11, -0x5ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -24272;
	// 826D6170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6174: 392A49BC  addi r9, r10, 0x49bc
	ctx.r[9].s64 = ctx.r[10].s64 + 18876;
	// 826D6178: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D617C: 38E00026  li r7, 0x26
	ctx.r[7].s64 = 38;
	// 826D6180: 38AABBC4  addi r5, r10, -0x443c
	ctx.r[5].s64 = ctx.r[10].s64 + -17468;
	// 826D6184: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D618C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D619C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D61A0: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826D61A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D61A8: 386BC854  addi r3, r11, -0x37ac
	ctx.r[3].s64 = ctx.r[11].s64 + -14252;
	// 826D61AC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D61B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D61B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D61B8: 4BD90C69  bl 0x82466e20
	ctx.lr = 0x826D61BC;
	sub_82466E20(ctx, base);
	// 826D61BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D61C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D61C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D61C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D61D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D61D0 size=112
    let mut pc: u32 = 0x826D61D0;
    'dispatch: loop {
        match pc {
            0x826D61D0 => {
    //   block [0x826D61D0..0x826D6240)
	// 826D61D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D61D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D61D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D61DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D61E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D61E4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D61E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D61EC: 390B2610  addi r8, r11, 0x2610
	ctx.r[8].s64 = ctx.r[11].s64 + 9744;
	// 826D61F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D61F4: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 826D61F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D61FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6208: 386AC884  addi r3, r10, -0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + -14204;
	// 826D620C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D621C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D622C: 4BD90BF5  bl 0x82466e20
	ctx.lr = 0x826D6230;
	sub_82466E20(ctx, base);
	// 826D6230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6240 size=108
    let mut pc: u32 = 0x826D6240;
    'dispatch: loop {
        match pc {
            0x826D6240 => {
    //   block [0x826D6240..0x826D62AC)
	// 826D6240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D624C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6254: 38EB2658  addi r7, r11, 0x2658
	ctx.r[7].s64 = ctx.r[11].s64 + 9816;
	// 826D6258: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D625C: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 826D6260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6264: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6270: 386AC8B4  addi r3, r10, -0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + -14156;
	// 826D6274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D627C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D628C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6298: 4BD90B89  bl 0x82466e20
	ctx.lr = 0x826D629C;
	sub_82466E20(ctx, base);
	// 826D629C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D62A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D62A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D62A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D62B0 size=108
    let mut pc: u32 = 0x826D62B0;
    'dispatch: loop {
        match pc {
            0x826D62B0 => {
    //   block [0x826D62B0..0x826D631C)
	// 826D62B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D62B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D62B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D62BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D62C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D62C4: 38EB26A0  addi r7, r11, 0x26a0
	ctx.r[7].s64 = ctx.r[11].s64 + 9888;
	// 826D62C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D62CC: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 826D62D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D62D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D62D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D62DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D62E0: 386AC8E4  addi r3, r10, -0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + -14108;
	// 826D62E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D62E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D62EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D62F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D62F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D62F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6308: 4BD90B19  bl 0x82466e20
	ctx.lr = 0x826D630C;
	sub_82466E20(ctx, base);
	// 826D630C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6320 size=116
    let mut pc: u32 = 0x826D6320;
    'dispatch: loop {
        match pc {
            0x826D6320 => {
    //   block [0x826D6320..0x826D6394)
	// 826D6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D632C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D6334: 390A26E8  addi r8, r10, 0x26e8
	ctx.r[8].s64 = ctx.r[10].s64 + 9960;
	// 826D6338: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D633C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6340: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D6344: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6348: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6354: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 826D6358: 396B49E4  addi r11, r11, 0x49e4
	ctx.r[11].s64 = ctx.r[11].s64 + 18916;
	// 826D635C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6364: 386AC914  addi r3, r10, -0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14060;
	// 826D6368: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D636C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6370: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D6374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D637C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6380: 4BD90AA1  bl 0x82466e20
	ctx.lr = 0x826D6384;
	sub_82466E20(ctx, base);
	// 826D6384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6398 size=116
    let mut pc: u32 = 0x826D6398;
    'dispatch: loop {
        match pc {
            0x826D6398 => {
    //   block [0x826D6398..0x826D640C)
	// 826D6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D63A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D63A4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D63A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D63AC: 390A2790  addi r8, r10, 0x2790
	ctx.r[8].s64 = ctx.r[10].s64 + 10128;
	// 826D63B0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D63B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D63B8: 38AAC914  addi r5, r10, -0x36ec
	ctx.r[5].s64 = ctx.r[10].s64 + -14060;
	// 826D63BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D63C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D63C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D63C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D63CC: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 826D63D0: 396B4A08  addi r11, r11, 0x4a08
	ctx.r[11].s64 = ctx.r[11].s64 + 18952;
	// 826D63D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D63D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D63DC: 386AC944  addi r3, r10, -0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14012;
	// 826D63E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D63E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D63E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D63EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D63F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D63F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D63F8: 4BD90A29  bl 0x82466e20
	ctx.lr = 0x826D63FC;
	sub_82466E20(ctx, base);
	// 826D63FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6410 size=36
    let mut pc: u32 = 0x826D6410;
    'dispatch: loop {
        match pc {
            0x826D6410 => {
    //   block [0x826D6410..0x826D6434)
	// 826D6410: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6414: 814B260C  lwz r10, 0x260c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9740 as u32) ) } as u64;
	// 826D6418: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D641C: 396BA4C0  addi r11, r11, -0x5b40
	ctx.r[11].s64 = ctx.r[11].s64 + -23360;
	// 826D6420: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D6424: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6428: 814A27F0  lwz r10, 0x27f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10224 as u32) ) } as u64;
	// 826D642C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D6430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6438 size=116
    let mut pc: u32 = 0x826D6438;
    'dispatch: loop {
        match pc {
            0x826D6438 => {
    //   block [0x826D6438..0x826D64AC)
	// 826D6438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6444: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6448: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D644C: 390BA4C0  addi r8, r11, -0x5b40
	ctx.r[8].s64 = ctx.r[11].s64 + -23360;
	// 826D6450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6454: 392A4A4C  addi r9, r10, 0x4a4c
	ctx.r[9].s64 = ctx.r[10].s64 + 19020;
	// 826D6458: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D645C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D6460: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6464: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D646C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D647C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6480: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 826D6484: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6488: 386BC974  addi r3, r11, -0x368c
	ctx.r[3].s64 = ctx.r[11].s64 + -13964;
	// 826D648C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D6490: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6498: 4BD90989  bl 0x82466e20
	ctx.lr = 0x826D649C;
	sub_82466E20(ctx, base);
	// 826D649C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D64A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D64A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D64A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D64B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D64B0 size=112
    let mut pc: u32 = 0x826D64B0;
    'dispatch: loop {
        match pc {
            0x826D64B0 => {
    //   block [0x826D64B0..0x826D6520)
	// 826D64B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D64B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D64B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D64BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D64C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D64C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D64C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D64CC: 390B27F8  addi r8, r11, 0x27f8
	ctx.r[8].s64 = ctx.r[11].s64 + 10232;
	// 826D64D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D64D4: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 826D64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D64DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D64E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D64E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D64E8: 386AC9A4  addi r3, r10, -0x365c
	ctx.r[3].s64 = ctx.r[10].s64 + -13916;
	// 826D64EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D64F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D64F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D64F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D64FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D650C: 4BD90915  bl 0x82466e20
	ctx.lr = 0x826D6510;
	sub_82466E20(ctx, base);
	// 826D6510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6520 size=108
    let mut pc: u32 = 0x826D6520;
    'dispatch: loop {
        match pc {
            0x826D6520 => {
    //   block [0x826D6520..0x826D658C)
	// 826D6520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D652C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6534: 38EB28B8  addi r7, r11, 0x28b8
	ctx.r[7].s64 = ctx.r[11].s64 + 10424;
	// 826D6538: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D653C: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 826D6540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D654C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6550: 386AC9D4  addi r3, r10, -0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + -13868;
	// 826D6554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D655C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D656C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6578: 4BD908A9  bl 0x82466e20
	ctx.lr = 0x826D657C;
	sub_82466E20(ctx, base);
	// 826D657C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6590 size=112
    let mut pc: u32 = 0x826D6590;
    'dispatch: loop {
        match pc {
            0x826D6590 => {
    //   block [0x826D6590..0x826D6600)
	// 826D6590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D659C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D65A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D65A4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D65A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D65AC: 390B28E8  addi r8, r11, 0x28e8
	ctx.r[8].s64 = ctx.r[11].s64 + 10472;
	// 826D65B0: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 826D65B4: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 826D65B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D65BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D65C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D65C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D65C8: 386ACA04  addi r3, r10, -0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13820;
	// 826D65CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D65D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D65D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D65D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D65DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D65E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D65E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D65E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D65EC: 4BD90835  bl 0x82466e20
	ctx.lr = 0x826D65F0;
	sub_82466E20(ctx, base);
	// 826D65F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D65F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D65F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D65FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6600 size=112
    let mut pc: u32 = 0x826D6600;
    'dispatch: loop {
        match pc {
            0x826D6600 => {
    //   block [0x826D6600..0x826D6670)
	// 826D6600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D660C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6610: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6614: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D661C: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 826D6620: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826D6624: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 826D6628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D662C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6638: 386ACA34  addi r3, r10, -0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13772;
	// 826D663C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D664C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D665C: 4BD907C5  bl 0x82466e20
	ctx.lr = 0x826D6660;
	sub_82466E20(ctx, base);
	// 826D6660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D666C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6670 size=116
    let mut pc: u32 = 0x826D6670;
    'dispatch: loop {
        match pc {
            0x826D6670 => {
    //   block [0x826D6670..0x826D66E4)
	// 826D6670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D667C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6680: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6684: 390B2B88  addi r8, r11, 0x2b88
	ctx.r[8].s64 = ctx.r[11].s64 + 11144;
	// 826D6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D668C: 392A4A84  addi r9, r10, 0x4a84
	ctx.r[9].s64 = ctx.r[10].s64 + 19076;
	// 826D6690: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6694: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 826D6698: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D669C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D66A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D66A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D66A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D66AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D66B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D66B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D66B8: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 826D66BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D66C0: 386BCA64  addi r3, r11, -0x359c
	ctx.r[3].s64 = ctx.r[11].s64 + -13724;
	// 826D66C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D66C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D66CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D66D0: 4BD90751  bl 0x82466e20
	ctx.lr = 0x826D66D4;
	sub_82466E20(ctx, base);
	// 826D66D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D66D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D66DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D66E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D66E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D66E8 size=112
    let mut pc: u32 = 0x826D66E8;
    'dispatch: loop {
        match pc {
            0x826D66E8 => {
    //   block [0x826D66E8..0x826D6758)
	// 826D66E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D66EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D66F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D66F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D66F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D66FC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6704: 390B2E40  addi r8, r11, 0x2e40
	ctx.r[8].s64 = ctx.r[11].s64 + 11840;
	// 826D6708: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D670C: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 826D6710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D671C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6720: 386ACA94  addi r3, r10, -0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + -13676;
	// 826D6724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D672C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D673C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6744: 4BD906DD  bl 0x82466e20
	ctx.lr = 0x826D6748;
	sub_82466E20(ctx, base);
	// 826D6748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D674C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6758 size=116
    let mut pc: u32 = 0x826D6758;
    'dispatch: loop {
        match pc {
            0x826D6758 => {
    //   block [0x826D6758..0x826D67CC)
	// 826D6758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D675C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6764: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6768: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D676C: 390A2E88  addi r8, r10, 0x2e88
	ctx.r[8].s64 = ctx.r[10].s64 + 11912;
	// 826D6770: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6774: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6778: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D677C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6780: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D678C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826D6790: 396B4A98  addi r11, r11, 0x4a98
	ctx.r[11].s64 = ctx.r[11].s64 + 19096;
	// 826D6794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6798: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D679C: 386ACAC4  addi r3, r10, -0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + -13628;
	// 826D67A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D67A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D67A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D67AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D67B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D67B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D67B8: 4BD90669  bl 0x82466e20
	ctx.lr = 0x826D67BC;
	sub_82466E20(ctx, base);
	// 826D67BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D67C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D67C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D67C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D67D0 size=112
    let mut pc: u32 = 0x826D67D0;
    'dispatch: loop {
        match pc {
            0x826D67D0 => {
    //   block [0x826D67D0..0x826D6840)
	// 826D67D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D67D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D67D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D67DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D67E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D67E4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D67E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D67EC: 390B2ED0  addi r8, r11, 0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 11984;
	// 826D67F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D67F4: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 826D67F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D67FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6808: 386ACAF4  addi r3, r10, -0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + -13580;
	// 826D680C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D681C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D682C: 4BD905F5  bl 0x82466e20
	ctx.lr = 0x826D6830;
	sub_82466E20(ctx, base);
	// 826D6830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6840 size=112
    let mut pc: u32 = 0x826D6840;
    'dispatch: loop {
        match pc {
            0x826D6840 => {
    //   block [0x826D6840..0x826D68B0)
	// 826D6840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D684C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6850: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6854: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6858: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D685C: 390B2F60  addi r8, r11, 0x2f60
	ctx.r[8].s64 = ctx.r[11].s64 + 12128;
	// 826D6860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D6864: 388AE324  addi r4, r10, -0x1cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -7388;
	// 826D6868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D686C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6878: 386ACB24  addi r3, r10, -0x34dc
	ctx.r[3].s64 = ctx.r[10].s64 + -13532;
	// 826D687C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D688C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D689C: 4BD90585  bl 0x82466e20
	ctx.lr = 0x826D68A0;
	sub_82466E20(ctx, base);
	// 826D68A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D68A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D68A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D68AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D68B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D68B0 size=112
    let mut pc: u32 = 0x826D68B0;
    'dispatch: loop {
        match pc {
            0x826D68B0 => {
    //   block [0x826D68B0..0x826D6920)
	// 826D68B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D68B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D68B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D68BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D68C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D68C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D68C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D68CC: 390B2FD8  addi r8, r11, 0x2fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 12248;
	// 826D68D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D68D4: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 826D68D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D68DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D68E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D68E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D68E8: 386ACB54  addi r3, r10, -0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13484;
	// 826D68EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D68F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D68F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D68F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D68FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D690C: 4BD90515  bl 0x82466e20
	ctx.lr = 0x826D6910;
	sub_82466E20(ctx, base);
	// 826D6910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D691C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6920 size=24
    let mut pc: u32 = 0x826D6920;
    'dispatch: loop {
        match pc {
            0x826D6920 => {
    //   block [0x826D6920..0x826D6938)
	// 826D6920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6928: 394AA5C8  addi r10, r10, -0x5a38
	ctx.r[10].s64 = ctx.r[10].s64 + -23096;
	// 826D692C: 816B3050  lwz r11, 0x3050(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12368 as u32) ) } as u64;
	// 826D6930: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826D6934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6938 size=116
    let mut pc: u32 = 0x826D6938;
    'dispatch: loop {
        match pc {
            0x826D6938 => {
    //   block [0x826D6938..0x826D69AC)
	// 826D6938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6944: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6948: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D694C: 392B4AC4  addi r9, r11, 0x4ac4
	ctx.r[9].s64 = ctx.r[11].s64 + 19140;
	// 826D6950: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6954: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6958: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D695C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D6960: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6964: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826D6968: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D696C: 396BA5C8  addi r11, r11, -0x5a38
	ctx.r[11].s64 = ctx.r[11].s64 + -23096;
	// 826D6970: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D6974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6978: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D697C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6980: 386ACB84  addi r3, r10, -0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + -13436;
	// 826D6984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6988: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D698C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6990: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D6994: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D6998: 4BD90489  bl 0x82466e20
	ctx.lr = 0x826D699C;
	sub_82466E20(ctx, base);
	// 826D699C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D69A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D69A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D69A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D69B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D69B0 size=112
    let mut pc: u32 = 0x826D69B0;
    'dispatch: loop {
        match pc {
            0x826D69B0 => {
    //   block [0x826D69B0..0x826D6A20)
	// 826D69B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D69B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D69B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D69BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D69C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D69C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D69C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D69CC: 390B3058  addi r8, r11, 0x3058
	ctx.r[8].s64 = ctx.r[11].s64 + 12376;
	// 826D69D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D69D4: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 826D69D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D69DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D69E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D69E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D69E8: 386ACBB4  addi r3, r10, -0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + -13388;
	// 826D69EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D69F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D69F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D69F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D69FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6A0C: 4BD90415  bl 0x82466e20
	ctx.lr = 0x826D6A10;
	sub_82466E20(ctx, base);
	// 826D6A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6A20 size=24
    let mut pc: u32 = 0x826D6A20;
    'dispatch: loop {
        match pc {
            0x826D6A20 => {
    //   block [0x826D6A20..0x826D6A38)
	// 826D6A20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6A24: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6A28: 394AA670  addi r10, r10, -0x5990
	ctx.r[10].s64 = ctx.r[10].s64 + -22928;
	// 826D6A2C: 816B3054  lwz r11, 0x3054(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12372 as u32) ) } as u64;
	// 826D6A30: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D6A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6A38 size=116
    let mut pc: u32 = 0x826D6A38;
    'dispatch: loop {
        match pc {
            0x826D6A38 => {
    //   block [0x826D6A38..0x826D6AAC)
	// 826D6A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6A44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6A48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6A4C: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 826D6A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6A54: 392A4B2C  addi r9, r10, 0x4b2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19244;
	// 826D6A58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6A5C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D6A60: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6A64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6A6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6A7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6A80: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826D6A84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6A88: 386BCBE4  addi r3, r11, -0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + -13340;
	// 826D6A8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6A90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6A98: 4BD90389  bl 0x82466e20
	ctx.lr = 0x826D6A9C;
	sub_82466E20(ctx, base);
	// 826D6A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6AB0 size=112
    let mut pc: u32 = 0x826D6AB0;
    'dispatch: loop {
        match pc {
            0x826D6AB0 => {
    //   block [0x826D6AB0..0x826D6B20)
	// 826D6AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6ABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6AC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6AC4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6ACC: 390B30D0  addi r8, r11, 0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + 12496;
	// 826D6AD0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D6AD4: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 826D6AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6ADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6AE8: 386ACC14  addi r3, r10, -0x33ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13292;
	// 826D6AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6B0C: 4BD90315  bl 0x82466e20
	ctx.lr = 0x826D6B10;
	sub_82466E20(ctx, base);
	// 826D6B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6B20 size=112
    let mut pc: u32 = 0x826D6B20;
    'dispatch: loop {
        match pc {
            0x826D6B20 => {
    //   block [0x826D6B20..0x826D6B90)
	// 826D6B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6B2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6B30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6B34: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6B3C: 390B31C0  addi r8, r11, 0x31c0
	ctx.r[8].s64 = ctx.r[11].s64 + 12736;
	// 826D6B40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6B44: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 826D6B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6B58: 386ACC44  addi r3, r10, -0x33bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13244;
	// 826D6B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6B7C: 4BD902A5  bl 0x82466e20
	ctx.lr = 0x826D6B80;
	sub_82466E20(ctx, base);
	// 826D6B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6B90 size=24
    let mut pc: u32 = 0x826D6B90;
    'dispatch: loop {
        match pc {
            0x826D6B90 => {
    //   block [0x826D6B90..0x826D6BA8)
	// 826D6B90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6B94: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6B98: 394AA718  addi r10, r10, -0x58e8
	ctx.r[10].s64 = ctx.r[10].s64 + -22760;
	// 826D6B9C: 816B3220  lwz r11, 0x3220(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12832 as u32) ) } as u64;
	// 826D6BA0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D6BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6BA8 size=116
    let mut pc: u32 = 0x826D6BA8;
    'dispatch: loop {
        match pc {
            0x826D6BA8 => {
    //   block [0x826D6BA8..0x826D6C1C)
	// 826D6BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6BB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6BB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6BBC: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 826D6BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6BC4: 392A4B78  addi r9, r10, 0x4b78
	ctx.r[9].s64 = ctx.r[10].s64 + 19320;
	// 826D6BC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6BCC: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 826D6BD0: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6BD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6BDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6BEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6BF0: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826D6BF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6BF8: 386BCC74  addi r3, r11, -0x338c
	ctx.r[3].s64 = ctx.r[11].s64 + -13196;
	// 826D6BFC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D6C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6C08: 4BD90219  bl 0x82466e20
	ctx.lr = 0x826D6C0C;
	sub_82466E20(ctx, base);
	// 826D6C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6C20 size=116
    let mut pc: u32 = 0x826D6C20;
    'dispatch: loop {
        match pc {
            0x826D6C20 => {
    //   block [0x826D6C20..0x826D6C94)
	// 826D6C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6C2C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6C30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6C34: 392B4BB0  addi r9, r11, 0x4bb0
	ctx.r[9].s64 = ctx.r[11].s64 + 19376;
	// 826D6C38: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6C3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6C40: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826D6C44: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 826D6C48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6C4C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 826D6C50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6C54: 396B3230  addi r11, r11, 0x3230
	ctx.r[11].s64 = ctx.r[11].s64 + 12848;
	// 826D6C58: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D6C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6C60: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D6C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6C68: 386ACCA4  addi r3, r10, -0x335c
	ctx.r[3].s64 = ctx.r[10].s64 + -13148;
	// 826D6C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6C70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D6C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6C78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D6C7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D6C80: 4BD901A1  bl 0x82466e20
	ctx.lr = 0x826D6C84;
	sub_82466E20(ctx, base);
	// 826D6C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6C98 size=112
    let mut pc: u32 = 0x826D6C98;
    'dispatch: loop {
        match pc {
            0x826D6C98 => {
    //   block [0x826D6C98..0x826D6D08)
	// 826D6C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6CA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6CAC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D6CB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6CB4: 390B33F8  addi r8, r11, 0x33f8
	ctx.r[8].s64 = ctx.r[11].s64 + 13304;
	// 826D6CB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6CBC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 826D6CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6CC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6CD0: 386ACCD4  addi r3, r10, -0x332c
	ctx.r[3].s64 = ctx.r[10].s64 + -13100;
	// 826D6CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6CF4: 4BD9012D  bl 0x82466e20
	ctx.lr = 0x826D6CF8;
	sub_82466E20(ctx, base);
	// 826D6CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6D08 size=48
    let mut pc: u32 = 0x826D6D08;
    'dispatch: loop {
        match pc {
            0x826D6D08 => {
    //   block [0x826D6D08..0x826D6D38)
	// 826D6D08: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6D0C: 814B3458  lwz r10, 0x3458(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13400 as u32) ) } as u64;
	// 826D6D10: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6D14: 396BA970  addi r11, r11, -0x5690
	ctx.r[11].s64 = ctx.r[11].s64 + -22160;
	// 826D6D18: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D6D1C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6D20: 814A345C  lwz r10, 0x345c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13404 as u32) ) } as u64;
	// 826D6D24: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826D6D28: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6D2C: 814A322C  lwz r10, 0x322c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12844 as u32) ) } as u64;
	// 826D6D30: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6D38 size=116
    let mut pc: u32 = 0x826D6D38;
    'dispatch: loop {
        match pc {
            0x826D6D38 => {
    //   block [0x826D6D38..0x826D6DAC)
	// 826D6D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6D44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6D48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6D4C: 390BA970  addi r8, r11, -0x5690
	ctx.r[8].s64 = ctx.r[11].s64 + -22160;
	// 826D6D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6D54: 392A4C70  addi r9, r10, 0x4c70
	ctx.r[9].s64 = ctx.r[10].s64 + 19568;
	// 826D6D58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6D5C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826D6D60: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6D64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6D6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6D7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6D80: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826D6D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6D88: 386BCD04  addi r3, r11, -0x32fc
	ctx.r[3].s64 = ctx.r[11].s64 + -13052;
	// 826D6D8C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826D6D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6D98: 4BD90089  bl 0x82466e20
	ctx.lr = 0x826D6D9C;
	sub_82466E20(ctx, base);
	// 826D6D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6DB0 size=112
    let mut pc: u32 = 0x826D6DB0;
    'dispatch: loop {
        match pc {
            0x826D6DB0 => {
    //   block [0x826D6DB0..0x826D6E20)
	// 826D6DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6DC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6DC4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6DCC: 390B3460  addi r8, r11, 0x3460
	ctx.r[8].s64 = ctx.r[11].s64 + 13408;
	// 826D6DD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D6DD4: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 826D6DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6DE8: 386ACD34  addi r3, r10, -0x32cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13004;
	// 826D6DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6E0C: 4BD90015  bl 0x82466e20
	ctx.lr = 0x826D6E10;
	sub_82466E20(ctx, base);
	// 826D6E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6E20 size=112
    let mut pc: u32 = 0x826D6E20;
    'dispatch: loop {
        match pc {
            0x826D6E20 => {
    //   block [0x826D6E20..0x826D6E90)
	// 826D6E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6E30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6E34: 38AAC194  addi r5, r10, -0x3e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -15980;
	// 826D6E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6E3C: 390B34A8  addi r8, r11, 0x34a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13480;
	// 826D6E40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6E44: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826D6E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6E58: 386ACD64  addi r3, r10, -0x329c
	ctx.r[3].s64 = ctx.r[10].s64 + -12956;
	// 826D6E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6E7C: 4BD8FFA5  bl 0x82466e20
	ctx.lr = 0x826D6E80;
	sub_82466E20(ctx, base);
	// 826D6E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6E90 size=116
    let mut pc: u32 = 0x826D6E90;
    'dispatch: loop {
        match pc {
            0x826D6E90 => {
    //   block [0x826D6E90..0x826D6F04)
	// 826D6E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6E9C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6EA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D6EA4: 390A3508  addi r8, r10, 0x3508
	ctx.r[8].s64 = ctx.r[10].s64 + 13576;
	// 826D6EA8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6EAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6EB0: 38AAC5B4  addi r5, r10, -0x3a4c
	ctx.r[5].s64 = ctx.r[10].s64 + -14924;
	// 826D6EB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6EB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6EC4: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 826D6EC8: 396B4CAC  addi r11, r11, 0x4cac
	ctx.r[11].s64 = ctx.r[11].s64 + 19628;
	// 826D6ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6ED4: 386ACD94  addi r3, r10, -0x326c
	ctx.r[3].s64 = ctx.r[10].s64 + -12908;
	// 826D6ED8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D6EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6EE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D6EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6EF0: 4BD8FF31  bl 0x82466e20
	ctx.lr = 0x826D6EF4;
	sub_82466E20(ctx, base);
	// 826D6EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6F08 size=108
    let mut pc: u32 = 0x826D6F08;
    'dispatch: loop {
        match pc {
            0x826D6F08 => {
    //   block [0x826D6F08..0x826D6F74)
	// 826D6F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6F14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6F18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6F1C: 38EB3580  addi r7, r11, 0x3580
	ctx.r[7].s64 = ctx.r[11].s64 + 13696;
	// 826D6F20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D6F24: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 826D6F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D6F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6F38: 386ACDC4  addi r3, r10, -0x323c
	ctx.r[3].s64 = ctx.r[10].s64 + -12860;
	// 826D6F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6F60: 4BD8FEC1  bl 0x82466e20
	ctx.lr = 0x826D6F64;
	sub_82466E20(ctx, base);
	// 826D6F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6F78 size=108
    let mut pc: u32 = 0x826D6F78;
    'dispatch: loop {
        match pc {
            0x826D6F78 => {
    //   block [0x826D6F78..0x826D6FE4)
	// 826D6F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6F84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6F88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6F8C: 38EB35C8  addi r7, r11, 0x35c8
	ctx.r[7].s64 = ctx.r[11].s64 + 13768;
	// 826D6F90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D6F94: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826D6F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6F9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6FA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D6FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6FA8: 386ACDF4  addi r3, r10, -0x320c
	ctx.r[3].s64 = ctx.r[10].s64 + -12812;
	// 826D6FAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6FD0: 4BD8FE51  bl 0x82466e20
	ctx.lr = 0x826D6FD4;
	sub_82466E20(ctx, base);
	// 826D6FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6FE8 size=108
    let mut pc: u32 = 0x826D6FE8;
    'dispatch: loop {
        match pc {
            0x826D6FE8 => {
    //   block [0x826D6FE8..0x826D7054)
	// 826D6FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6FF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6FF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6FFC: 38EB3610  addi r7, r11, 0x3610
	ctx.r[7].s64 = ctx.r[11].s64 + 13840;
	// 826D7000: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D7004: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 826D7008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D700C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7018: 386ACE24  addi r3, r10, -0x31dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12764;
	// 826D701C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D702C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D703C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7040: 4BD8FDE1  bl 0x82466e20
	ctx.lr = 0x826D7044;
	sub_82466E20(ctx, base);
	// 826D7044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D704C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7058 size=108
    let mut pc: u32 = 0x826D7058;
    'dispatch: loop {
        match pc {
            0x826D7058 => {
    //   block [0x826D7058..0x826D70C4)
	// 826D7058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D705C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7064: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D706C: 38EB3658  addi r7, r11, 0x3658
	ctx.r[7].s64 = ctx.r[11].s64 + 13912;
	// 826D7070: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D7074: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826D7078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D707C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7088: 386ACE54  addi r3, r10, -0x31ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12716;
	// 826D708C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D709C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D70A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D70A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D70A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D70AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D70B0: 4BD8FD71  bl 0x82466e20
	ctx.lr = 0x826D70B4;
	sub_82466E20(ctx, base);
	// 826D70B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D70B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D70BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D70C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D70C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D70C8 size=108
    let mut pc: u32 = 0x826D70C8;
    'dispatch: loop {
        match pc {
            0x826D70C8 => {
    //   block [0x826D70C8..0x826D7134)
	// 826D70C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D70CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D70D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D70D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D70D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D70DC: 38EB3700  addi r7, r11, 0x3700
	ctx.r[7].s64 = ctx.r[11].s64 + 14080;
	// 826D70E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D70E4: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826D70E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D70EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D70F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D70F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D70F8: 386ACE84  addi r3, r10, -0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + -12668;
	// 826D70FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D710C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D711C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7120: 4BD8FD01  bl 0x82466e20
	ctx.lr = 0x826D7124;
	sub_82466E20(ctx, base);
	// 826D7124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D712C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7138 size=112
    let mut pc: u32 = 0x826D7138;
    'dispatch: loop {
        match pc {
            0x826D7138 => {
    //   block [0x826D7138..0x826D71A8)
	// 826D7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7144: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D714C: 392A4CEC  addi r9, r10, 0x4cec
	ctx.r[9].s64 = ctx.r[10].s64 + 19692;
	// 826D7150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7154: 390B3738  addi r8, r11, 0x3738
	ctx.r[8].s64 = ctx.r[11].s64 + 14136;
	// 826D7158: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D715C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826D7160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D716C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7170: 386ACEB4  addi r3, r10, -0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + -12620;
	// 826D7174: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7178: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D718C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7194: 4BD8FC8D  bl 0x82466e20
	ctx.lr = 0x826D7198;
	sub_82466E20(ctx, base);
	// 826D7198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D719C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D71A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D71A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D71A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D71A8 size=108
    let mut pc: u32 = 0x826D71A8;
    'dispatch: loop {
        match pc {
            0x826D71A8 => {
    //   block [0x826D71A8..0x826D7214)
	// 826D71A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D71AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D71B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D71B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D71B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D71BC: 38EB3780  addi r7, r11, 0x3780
	ctx.r[7].s64 = ctx.r[11].s64 + 14208;
	// 826D71C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D71C4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826D71C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D71CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D71D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D71D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D71D8: 386ACEE4  addi r3, r10, -0x311c
	ctx.r[3].s64 = ctx.r[10].s64 + -12572;
	// 826D71DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D71E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D71E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D71E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D71EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D71F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D71F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D71F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D71FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7200: 4BD8FC21  bl 0x82466e20
	ctx.lr = 0x826D7204;
	sub_82466E20(ctx, base);
	// 826D7204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7218 size=108
    let mut pc: u32 = 0x826D7218;
    'dispatch: loop {
        match pc {
            0x826D7218 => {
    //   block [0x826D7218..0x826D7284)
	// 826D7218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D721C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7224: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7228: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D722C: 38EB37F8  addi r7, r11, 0x37f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14328;
	// 826D7230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7234: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826D7238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D723C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7248: 386ACF14  addi r3, r10, -0x30ec
	ctx.r[3].s64 = ctx.r[10].s64 + -12524;
	// 826D724C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D725C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D726C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7270: 4BD8FBB1  bl 0x82466e20
	ctx.lr = 0x826D7274;
	sub_82466E20(ctx, base);
	// 826D7274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D727C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7288 size=108
    let mut pc: u32 = 0x826D7288;
    'dispatch: loop {
        match pc {
            0x826D7288 => {
    //   block [0x826D7288..0x826D72F4)
	// 826D7288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D728C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7294: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7298: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D729C: 38EB3828  addi r7, r11, 0x3828
	ctx.r[7].s64 = ctx.r[11].s64 + 14376;
	// 826D72A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D72A4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826D72A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D72AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D72B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D72B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D72B8: 386ACF44  addi r3, r10, -0x30bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12476;
	// 826D72BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D72C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D72C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D72C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D72CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D72D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D72D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D72D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D72DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D72E0: 4BD8FB41  bl 0x82466e20
	ctx.lr = 0x826D72E4;
	sub_82466E20(ctx, base);
	// 826D72E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D72E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D72EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D72F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D72F8 size=24
    let mut pc: u32 = 0x826D72F8;
    'dispatch: loop {
        match pc {
            0x826D72F8 => {
    //   block [0x826D72F8..0x826D7310)
	// 826D72F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D72FC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7300: 394AAB80  addi r10, r10, -0x5480
	ctx.r[10].s64 = ctx.r[10].s64 + -21632;
	// 826D7304: 816B3840  lwz r11, 0x3840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14400 as u32) ) } as u64;
	// 826D7308: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7310 size=112
    let mut pc: u32 = 0x826D7310;
    'dispatch: loop {
        match pc {
            0x826D7310 => {
    //   block [0x826D7310..0x826D7380)
	// 826D7310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D731C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7320: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7324: 392A4D2C  addi r9, r10, 0x4d2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19756;
	// 826D7328: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D732C: 390BAB80  addi r8, r11, -0x5480
	ctx.r[8].s64 = ctx.r[11].s64 + -21632;
	// 826D7330: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D7334: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826D7338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D733C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7348: 386ACF74  addi r3, r10, -0x308c
	ctx.r[3].s64 = ctx.r[10].s64 + -12428;
	// 826D734C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D7354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D736C: 4BD8FAB5  bl 0x82466e20
	ctx.lr = 0x826D7370;
	sub_82466E20(ctx, base);
	// 826D7370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7380 size=96
    let mut pc: u32 = 0x826D7380;
    'dispatch: loop {
        match pc {
            0x826D7380 => {
    //   block [0x826D7380..0x826D73E0)
	// 826D7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D738C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7394: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826D7398: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D73A0: 386ACFA4  addi r3, r10, -0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + -12380;
	// 826D73A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D73A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D73AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D73B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D73B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D73B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D73BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D73C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D73C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D73C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D73CC: 4BD8FA55  bl 0x82466e20
	ctx.lr = 0x826D73D0;
	sub_82466E20(ctx, base);
	// 826D73D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D73D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D73D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D73DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D73E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D73E0 size=112
    let mut pc: u32 = 0x826D73E0;
    'dispatch: loop {
        match pc {
            0x826D73E0 => {
    //   block [0x826D73E0..0x826D7450)
	// 826D73E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D73E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D73E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D73EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D73F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D73F4: 38AACFA4  addi r5, r10, -0x305c
	ctx.r[5].s64 = ctx.r[10].s64 + -12380;
	// 826D73F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D73FC: 390B3844  addi r8, r11, 0x3844
	ctx.r[8].s64 = ctx.r[11].s64 + 14404;
	// 826D7400: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D7404: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826D7408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D740C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7418: 386ACFD4  addi r3, r10, -0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + -12332;
	// 826D741C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D7420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D742C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D743C: 4BD8F9E5  bl 0x82466e20
	ctx.lr = 0x826D7440;
	sub_82466E20(ctx, base);
	// 826D7440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D744C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7450 size=24
    let mut pc: u32 = 0x826D7450;
    'dispatch: loop {
        match pc {
            0x826D7450 => {
    //   block [0x826D7450..0x826D7468)
	// 826D7450: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7454: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7458: 394AAC58  addi r10, r10, -0x53a8
	ctx.r[10].s64 = ctx.r[10].s64 + -21416;
	// 826D745C: 816B3878  lwz r11, 0x3878(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14456 as u32) ) } as u64;
	// 826D7460: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826D7464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7468 size=112
    let mut pc: u32 = 0x826D7468;
    'dispatch: loop {
        match pc {
            0x826D7468 => {
    //   block [0x826D7468..0x826D74D8)
	// 826D7468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7474: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7478: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D747C: 392A4D50  addi r9, r10, 0x4d50
	ctx.r[9].s64 = ctx.r[10].s64 + 19792;
	// 826D7480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7484: 390BAC58  addi r8, r11, -0x53a8
	ctx.r[8].s64 = ctx.r[11].s64 + -21416;
	// 826D7488: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D748C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826D7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D74A0: 386AD004  addi r3, r10, -0x2ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -12284;
	// 826D74A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D74A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D74AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D74B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D74BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D74C4: 4BD8F95D  bl 0x82466e20
	ctx.lr = 0x826D74C8;
	sub_82466E20(ctx, base);
	// 826D74C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D74CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D74D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D74D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D74D8 size=108
    let mut pc: u32 = 0x826D74D8;
    'dispatch: loop {
        match pc {
            0x826D74D8 => {
    //   block [0x826D74D8..0x826D7544)
	// 826D74D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D74DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D74E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D74E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D74E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D74EC: 38EB3880  addi r7, r11, 0x3880
	ctx.r[7].s64 = ctx.r[11].s64 + 14464;
	// 826D74F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D74F4: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826D74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D74FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7508: 386AD034  addi r3, r10, -0x2fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -12236;
	// 826D750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7530: 4BD8F8F1  bl 0x82466e20
	ctx.lr = 0x826D7534;
	sub_82466E20(ctx, base);
	// 826D7534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7548 size=24
    let mut pc: u32 = 0x826D7548;
    'dispatch: loop {
        match pc {
            0x826D7548 => {
    //   block [0x826D7548..0x826D7560)
	// 826D7548: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D754C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7550: 394AAD48  addi r10, r10, -0x52b8
	ctx.r[10].s64 = ctx.r[10].s64 + -21176;
	// 826D7554: 816B387C  lwz r11, 0x387c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14460 as u32) ) } as u64;
	// 826D7558: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7560 size=112
    let mut pc: u32 = 0x826D7560;
    'dispatch: loop {
        match pc {
            0x826D7560 => {
    //   block [0x826D7560..0x826D75D0)
	// 826D7560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D756C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7570: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7574: 392A4D80  addi r9, r10, 0x4d80
	ctx.r[9].s64 = ctx.r[10].s64 + 19840;
	// 826D7578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D757C: 390BAD48  addi r8, r11, -0x52b8
	ctx.r[8].s64 = ctx.r[11].s64 + -21176;
	// 826D7580: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D7584: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826D7588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D758C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7598: 386AD064  addi r3, r10, -0x2f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -12188;
	// 826D759C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D75A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D75A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D75A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D75AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D75B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D75B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D75B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D75BC: 4BD8F865  bl 0x82466e20
	ctx.lr = 0x826D75C0;
	sub_82466E20(ctx, base);
	// 826D75C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D75C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D75C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D75CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D75D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D75D0 size=40
    let mut pc: u32 = 0x826D75D0;
    'dispatch: loop {
        match pc {
            0x826D75D0 => {
    //   block [0x826D75D0..0x826D75F8)
	// 826D75D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D75D4: 814B38B0  lwz r10, 0x38b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14512 as u32) ) } as u64;
	// 826D75D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D75DC: 396BADA8  addi r11, r11, -0x5258
	ctx.r[11].s64 = ctx.r[11].s64 + -21080;
	// 826D75E0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826D75E4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 826D75E8: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D75EC: 814A38B4  lwz r10, 0x38b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(14516 as u32) ) } as u64;
	// 826D75F0: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826D75F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D75F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D75F8 size=112
    let mut pc: u32 = 0x826D75F8;
    'dispatch: loop {
        match pc {
            0x826D75F8 => {
    //   block [0x826D75F8..0x826D7668)
	// 826D75F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D75FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7604: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7608: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D760C: 392A4F30  addi r9, r10, 0x4f30
	ctx.r[9].s64 = ctx.r[10].s64 + 20272;
	// 826D7610: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7614: 390BADA8  addi r8, r11, -0x5258
	ctx.r[8].s64 = ctx.r[11].s64 + -21080;
	// 826D7618: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D761C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826D7620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7630: 386AD094  addi r3, r10, -0x2f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -12140;
	// 826D7634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7638: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826D763C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D764C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7654: 4BD8F7CD  bl 0x82466e20
	ctx.lr = 0x826D7658;
	sub_82466E20(ctx, base);
	// 826D7658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D765C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7668 size=108
    let mut pc: u32 = 0x826D7668;
    'dispatch: loop {
        match pc {
            0x826D7668 => {
    //   block [0x826D7668..0x826D76D4)
	// 826D7668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D766C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7674: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D767C: 38EB38C0  addi r7, r11, 0x38c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14528;
	// 826D7680: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7684: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 826D7688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D768C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7698: 386AD0C4  addi r3, r10, -0x2f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -12092;
	// 826D769C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D76A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D76A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D76A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D76AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D76B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D76B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D76B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D76BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D76C0: 4BD8F761  bl 0x82466e20
	ctx.lr = 0x826D76C4;
	sub_82466E20(ctx, base);
	// 826D76C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D76C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D76CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D76D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D76D8 size=108
    let mut pc: u32 = 0x826D76D8;
    'dispatch: loop {
        match pc {
            0x826D76D8 => {
    //   block [0x826D76D8..0x826D7744)
	// 826D76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D76E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D76E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D76EC: 38EB38F0  addi r7, r11, 0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + 14576;
	// 826D76F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D76F4: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 826D76F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D76FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7708: 386AD0F4  addi r3, r10, -0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -12044;
	// 826D770C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D771C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D772C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7730: 4BD8F6F1  bl 0x82466e20
	ctx.lr = 0x826D7734;
	sub_82466E20(ctx, base);
	// 826D7734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D773C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7748 size=108
    let mut pc: u32 = 0x826D7748;
    'dispatch: loop {
        match pc {
            0x826D7748 => {
    //   block [0x826D7748..0x826D77B4)
	// 826D7748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7754: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7758: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D775C: 38EB3908  addi r7, r11, 0x3908
	ctx.r[7].s64 = ctx.r[11].s64 + 14600;
	// 826D7760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7764: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826D7768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D776C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7778: 386AD124  addi r3, r10, -0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + -11996;
	// 826D777C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D778C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D779C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D77A0: 4BD8F681  bl 0x82466e20
	ctx.lr = 0x826D77A4;
	sub_82466E20(ctx, base);
	// 826D77A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D77A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D77AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D77B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D77B8 size=108
    let mut pc: u32 = 0x826D77B8;
    'dispatch: loop {
        match pc {
            0x826D77B8 => {
    //   block [0x826D77B8..0x826D7824)
	// 826D77B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D77BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D77C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D77C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D77C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D77CC: 38EB3938  addi r7, r11, 0x3938
	ctx.r[7].s64 = ctx.r[11].s64 + 14648;
	// 826D77D0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D77D4: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 826D77D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D77DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D77E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D77E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D77E8: 386AD154  addi r3, r10, -0x2eac
	ctx.r[3].s64 = ctx.r[10].s64 + -11948;
	// 826D77EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D77F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D77F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D77FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7810: 4BD8F611  bl 0x82466e20
	ctx.lr = 0x826D7814;
	sub_82466E20(ctx, base);
	// 826D7814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7828 size=108
    let mut pc: u32 = 0x826D7828;
    'dispatch: loop {
        match pc {
            0x826D7828 => {
    //   block [0x826D7828..0x826D7894)
	// 826D7828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7834: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7838: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D783C: 38EB39C8  addi r7, r11, 0x39c8
	ctx.r[7].s64 = ctx.r[11].s64 + 14792;
	// 826D7840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7844: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826D7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D784C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7858: 386AD184  addi r3, r10, -0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11900;
	// 826D785C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D786C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D787C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7880: 4BD8F5A1  bl 0x82466e20
	ctx.lr = 0x826D7884;
	sub_82466E20(ctx, base);
	// 826D7884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7898 size=108
    let mut pc: u32 = 0x826D7898;
    'dispatch: loop {
        match pc {
            0x826D7898 => {
    //   block [0x826D7898..0x826D7904)
	// 826D7898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D78A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D78A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D78A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D78AC: 38EB39E0  addi r7, r11, 0x39e0
	ctx.r[7].s64 = ctx.r[11].s64 + 14816;
	// 826D78B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D78B4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826D78B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D78BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D78C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D78C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D78C8: 386AD1B4  addi r3, r10, -0x2e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11852;
	// 826D78CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D78D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D78D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D78DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D78E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D78E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D78E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D78EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D78F0: 4BD8F531  bl 0x82466e20
	ctx.lr = 0x826D78F4;
	sub_82466E20(ctx, base);
	// 826D78F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7908 size=112
    let mut pc: u32 = 0x826D7908;
    'dispatch: loop {
        match pc {
            0x826D7908 => {
    //   block [0x826D7908..0x826D7978)
	// 826D7908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7914: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7918: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D791C: 392A4F98  addi r9, r10, 0x4f98
	ctx.r[9].s64 = ctx.r[10].s64 + 20376;
	// 826D7920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7924: 390B3A18  addi r8, r11, 0x3a18
	ctx.r[8].s64 = ctx.r[11].s64 + 14872;
	// 826D7928: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D792C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826D7930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D793C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7940: 386AD1E4  addi r3, r10, -0x2e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11804;
	// 826D7944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D794C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D795C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7964: 4BD8F4BD  bl 0x82466e20
	ctx.lr = 0x826D7968;
	sub_82466E20(ctx, base);
	// 826D7968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D796C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7978 size=108
    let mut pc: u32 = 0x826D7978;
    'dispatch: loop {
        match pc {
            0x826D7978 => {
    //   block [0x826D7978..0x826D79E4)
	// 826D7978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7984: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7988: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D798C: 38EB3A90  addi r7, r11, 0x3a90
	ctx.r[7].s64 = ctx.r[11].s64 + 14992;
	// 826D7990: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D7994: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826D7998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D799C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D79A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D79A8: 386AD214  addi r3, r10, -0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + -11756;
	// 826D79AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D79B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D79B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D79B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D79C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D79C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D79C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D79CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D79D0: 4BD8F451  bl 0x82466e20
	ctx.lr = 0x826D79D4;
	sub_82466E20(ctx, base);
	// 826D79D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D79D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D79DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D79E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D79E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D79E8 size=24
    let mut pc: u32 = 0x826D79E8;
    'dispatch: loop {
        match pc {
            0x826D79E8 => {
    //   block [0x826D79E8..0x826D7A00)
	// 826D79E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D79EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D79F0: 394AAE80  addi r10, r10, -0x5180
	ctx.r[10].s64 = ctx.r[10].s64 + -20864;
	// 826D79F4: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D79F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D79FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7A00 size=108
    let mut pc: u32 = 0x826D7A00;
    'dispatch: loop {
        match pc {
            0x826D7A00 => {
    //   block [0x826D7A00..0x826D7A6C)
	// 826D7A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7A0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7A14: 38EBAE80  addi r7, r11, -0x5180
	ctx.r[7].s64 = ctx.r[11].s64 + -20864;
	// 826D7A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7A1C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826D7A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7A30: 386AD244  addi r3, r10, -0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11708;
	// 826D7A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7A58: 4BD8F3C9  bl 0x82466e20
	ctx.lr = 0x826D7A5C;
	sub_82466E20(ctx, base);
	// 826D7A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7A70 size=24
    let mut pc: u32 = 0x826D7A70;
    'dispatch: loop {
        match pc {
            0x826D7A70 => {
    //   block [0x826D7A70..0x826D7A88)
	// 826D7A70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7A74: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7A78: 394AAEB0  addi r10, r10, -0x5150
	ctx.r[10].s64 = ctx.r[10].s64 + -20816;
	// 826D7A7C: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D7A80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D7A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7A88 size=108
    let mut pc: u32 = 0x826D7A88;
    'dispatch: loop {
        match pc {
            0x826D7A88 => {
    //   block [0x826D7A88..0x826D7AF4)
	// 826D7A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7A94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7A9C: 38EBAEB0  addi r7, r11, -0x5150
	ctx.r[7].s64 = ctx.r[11].s64 + -20816;
	// 826D7AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7AA4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826D7AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7AAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7AB8: 386AD274  addi r3, r10, -0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11660;
	// 826D7ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7AE0: 4BD8F341  bl 0x82466e20
	ctx.lr = 0x826D7AE4;
	sub_82466E20(ctx, base);
	// 826D7AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7AF8 size=108
    let mut pc: u32 = 0x826D7AF8;
    'dispatch: loop {
        match pc {
            0x826D7AF8 => {
    //   block [0x826D7AF8..0x826D7B64)
	// 826D7AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7B04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7B0C: 38EB3B68  addi r7, r11, 0x3b68
	ctx.r[7].s64 = ctx.r[11].s64 + 15208;
	// 826D7B10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7B14: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826D7B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7B1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7B28: 386AD2A4  addi r3, r10, -0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11612;
	// 826D7B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7B50: 4BD8F2D1  bl 0x82466e20
	ctx.lr = 0x826D7B54;
	sub_82466E20(ctx, base);
	// 826D7B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7B68 size=24
    let mut pc: u32 = 0x826D7B68;
    'dispatch: loop {
        match pc {
            0x826D7B68 => {
    //   block [0x826D7B68..0x826D7B80)
	// 826D7B68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7B6C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7B70: 394AAEE0  addi r10, r10, -0x5120
	ctx.r[10].s64 = ctx.r[10].s64 + -20768;
	// 826D7B74: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D7B78: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D7B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7B80 size=108
    let mut pc: u32 = 0x826D7B80;
    'dispatch: loop {
        match pc {
            0x826D7B80 => {
    //   block [0x826D7B80..0x826D7BEC)
	// 826D7B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7B8C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7B94: 38EBAEE0  addi r7, r11, -0x5120
	ctx.r[7].s64 = ctx.r[11].s64 + -20768;
	// 826D7B98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7B9C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826D7BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7BA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7BB0: 386AD2D4  addi r3, r10, -0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11564;
	// 826D7BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7BD8: 4BD8F249  bl 0x82466e20
	ctx.lr = 0x826D7BDC;
	sub_82466E20(ctx, base);
	// 826D7BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7BF0 size=112
    let mut pc: u32 = 0x826D7BF0;
    'dispatch: loop {
        match pc {
            0x826D7BF0 => {
    //   block [0x826D7BF0..0x826D7C60)
	// 826D7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7BFC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7C00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7C04: 392A4FDC  addi r9, r10, 0x4fdc
	ctx.r[9].s64 = ctx.r[10].s64 + 20444;
	// 826D7C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7C0C: 390B3B80  addi r8, r11, 0x3b80
	ctx.r[8].s64 = ctx.r[11].s64 + 15232;
	// 826D7C10: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D7C14: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826D7C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7C1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7C28: 386AD304  addi r3, r10, -0x2cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11516;
	// 826D7C2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7C30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D7C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7C4C: 4BD8F1D5  bl 0x82466e20
	ctx.lr = 0x826D7C50;
	sub_82466E20(ctx, base);
	// 826D7C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7C60 size=108
    let mut pc: u32 = 0x826D7C60;
    'dispatch: loop {
        match pc {
            0x826D7C60 => {
    //   block [0x826D7C60..0x826D7CCC)
	// 826D7C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7C6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7C74: 38EB3BB0  addi r7, r11, 0x3bb0
	ctx.r[7].s64 = ctx.r[11].s64 + 15280;
	// 826D7C78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7C7C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826D7C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7C90: 386AD334  addi r3, r10, -0x2ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -11468;
	// 826D7C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7CB8: 4BD8F169  bl 0x82466e20
	ctx.lr = 0x826D7CBC;
	sub_82466E20(ctx, base);
	// 826D7CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7CD0 size=108
    let mut pc: u32 = 0x826D7CD0;
    'dispatch: loop {
        match pc {
            0x826D7CD0 => {
    //   block [0x826D7CD0..0x826D7D3C)
	// 826D7CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7CDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7CE4: 38EB3BE0  addi r7, r11, 0x3be0
	ctx.r[7].s64 = ctx.r[11].s64 + 15328;
	// 826D7CE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7CEC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826D7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7D00: 386AD364  addi r3, r10, -0x2c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11420;
	// 826D7D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7D28: 4BD8F0F9  bl 0x82466e20
	ctx.lr = 0x826D7D2C;
	sub_82466E20(ctx, base);
	// 826D7D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7D40 size=108
    let mut pc: u32 = 0x826D7D40;
    'dispatch: loop {
        match pc {
            0x826D7D40 => {
    //   block [0x826D7D40..0x826D7DAC)
	// 826D7D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7D4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7D54: 38EB3BF8  addi r7, r11, 0x3bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15352;
	// 826D7D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7D5C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826D7D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7D70: 386AD394  addi r3, r10, -0x2c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11372;
	// 826D7D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7D98: 4BD8F089  bl 0x82466e20
	ctx.lr = 0x826D7D9C;
	sub_82466E20(ctx, base);
	// 826D7D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7DB0 size=112
    let mut pc: u32 = 0x826D7DB0;
    'dispatch: loop {
        match pc {
            0x826D7DB0 => {
    //   block [0x826D7DB0..0x826D7E20)
	// 826D7DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7DC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7DC4: 38AAD3F4  addi r5, r10, -0x2c0c
	ctx.r[5].s64 = ctx.r[10].s64 + -11276;
	// 826D7DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7DCC: 390B3C28  addi r8, r11, 0x3c28
	ctx.r[8].s64 = ctx.r[11].s64 + 15400;
	// 826D7DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D7DD4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826D7DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7DE8: 386AD3C4  addi r3, r10, -0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11324;
	// 826D7DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D7DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7E0C: 4BD8F015  bl 0x82466e20
	ctx.lr = 0x826D7E10;
	sub_82466E20(ctx, base);
	// 826D7E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7E20 size=108
    let mut pc: u32 = 0x826D7E20;
    'dispatch: loop {
        match pc {
            0x826D7E20 => {
    //   block [0x826D7E20..0x826D7E8C)
	// 826D7E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7E2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7E34: 38EB3C40  addi r7, r11, 0x3c40
	ctx.r[7].s64 = ctx.r[11].s64 + 15424;
	// 826D7E38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7E3C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826D7E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7E50: 386AD3F4  addi r3, r10, -0x2c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11276;
	// 826D7E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7E78: 4BD8EFA9  bl 0x82466e20
	ctx.lr = 0x826D7E7C;
	sub_82466E20(ctx, base);
	// 826D7E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7E90 size=108
    let mut pc: u32 = 0x826D7E90;
    'dispatch: loop {
        match pc {
            0x826D7E90 => {
    //   block [0x826D7E90..0x826D7EFC)
	// 826D7E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7E9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7EA4: 38EB3C70  addi r7, r11, 0x3c70
	ctx.r[7].s64 = ctx.r[11].s64 + 15472;
	// 826D7EA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7EAC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826D7EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7EC0: 386AD424  addi r3, r10, -0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -11228;
	// 826D7EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7EE8: 4BD8EF39  bl 0x82466e20
	ctx.lr = 0x826D7EEC;
	sub_82466E20(ctx, base);
	// 826D7EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7F00 size=108
    let mut pc: u32 = 0x826D7F00;
    'dispatch: loop {
        match pc {
            0x826D7F00 => {
    //   block [0x826D7F00..0x826D7F6C)
	// 826D7F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7F0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7F14: 38EB3C88  addi r7, r11, 0x3c88
	ctx.r[7].s64 = ctx.r[11].s64 + 15496;
	// 826D7F18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7F1C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826D7F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7F30: 386AD454  addi r3, r10, -0x2bac
	ctx.r[3].s64 = ctx.r[10].s64 + -11180;
	// 826D7F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7F58: 4BD8EEC9  bl 0x82466e20
	ctx.lr = 0x826D7F5C;
	sub_82466E20(ctx, base);
	// 826D7F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7F70 size=108
    let mut pc: u32 = 0x826D7F70;
    'dispatch: loop {
        match pc {
            0x826D7F70 => {
    //   block [0x826D7F70..0x826D7FDC)
	// 826D7F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7F7C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7F84: 38EB3CB8  addi r7, r11, 0x3cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15544;
	// 826D7F88: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D7F8C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826D7F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7FA0: 386AD484  addi r3, r10, -0x2b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11132;
	// 826D7FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7FC8: 4BD8EE59  bl 0x82466e20
	ctx.lr = 0x826D7FCC;
	sub_82466E20(ctx, base);
	// 826D7FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7FE0 size=108
    let mut pc: u32 = 0x826D7FE0;
    'dispatch: loop {
        match pc {
            0x826D7FE0 => {
    //   block [0x826D7FE0..0x826D804C)
	// 826D7FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7FEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7FF4: 38EB3D60  addi r7, r11, 0x3d60
	ctx.r[7].s64 = ctx.r[11].s64 + 15712;
	// 826D7FF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7FFC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826D8000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D800C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8010: 386AD4B4  addi r3, r10, -0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11084;
	// 826D8014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D801C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D802C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8038: 4BD8EDE9  bl 0x82466e20
	ctx.lr = 0x826D803C;
	sub_82466E20(ctx, base);
	// 826D803C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8050 size=108
    let mut pc: u32 = 0x826D8050;
    'dispatch: loop {
        match pc {
            0x826D8050 => {
    //   block [0x826D8050..0x826D80BC)
	// 826D8050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D805C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8064: 38EB3D90  addi r7, r11, 0x3d90
	ctx.r[7].s64 = ctx.r[11].s64 + 15760;
	// 826D8068: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D806C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826D8070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D807C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8080: 386AD4E4  addi r3, r10, -0x2b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11036;
	// 826D8084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D808C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D809C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D80A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D80A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D80A8: 4BD8ED79  bl 0x82466e20
	ctx.lr = 0x826D80AC;
	sub_82466E20(ctx, base);
	// 826D80AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D80B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D80B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D80B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D80C0 size=108
    let mut pc: u32 = 0x826D80C0;
    'dispatch: loop {
        match pc {
            0x826D80C0 => {
    //   block [0x826D80C0..0x826D812C)
	// 826D80C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D80C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D80C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D80CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D80D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D80D4: 38EB3DA8  addi r7, r11, 0x3da8
	ctx.r[7].s64 = ctx.r[11].s64 + 15784;
	// 826D80D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D80DC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826D80E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D80E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D80E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D80EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D80F0: 386AD514  addi r3, r10, -0x2aec
	ctx.r[3].s64 = ctx.r[10].s64 + -10988;
	// 826D80F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D80F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D80FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8118: 4BD8ED09  bl 0x82466e20
	ctx.lr = 0x826D811C;
	sub_82466E20(ctx, base);
	// 826D811C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8130 size=112
    let mut pc: u32 = 0x826D8130;
    'dispatch: loop {
        match pc {
            0x826D8130 => {
    //   block [0x826D8130..0x826D81A0)
	// 826D8130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D813C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8140: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8144: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D814C: 390B3DD8  addi r8, r11, 0x3dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 15832;
	// 826D8150: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D8154: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826D8158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D815C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8168: 386AD544  addi r3, r10, -0x2abc
	ctx.r[3].s64 = ctx.r[10].s64 + -10940;
	// 826D816C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D817C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D818C: 4BD8EC95  bl 0x82466e20
	ctx.lr = 0x826D8190;
	sub_82466E20(ctx, base);
	// 826D8190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D819C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D81A0 size=24
    let mut pc: u32 = 0x826D81A0;
    'dispatch: loop {
        match pc {
            0x826D81A0 => {
    //   block [0x826D81A0..0x826D81B8)
	// 826D81A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D81A4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D81A8: 394AAF10  addi r10, r10, -0x50f0
	ctx.r[10].s64 = ctx.r[10].s64 + -20720;
	// 826D81AC: 816B3E80  lwz r11, 0x3e80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16000 as u32) ) } as u64;
	// 826D81B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D81B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D81B8 size=112
    let mut pc: u32 = 0x826D81B8;
    'dispatch: loop {
        match pc {
            0x826D81B8 => {
    //   block [0x826D81B8..0x826D8228)
	// 826D81B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D81BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D81C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D81C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D81C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D81CC: 392A5008  addi r9, r10, 0x5008
	ctx.r[9].s64 = ctx.r[10].s64 + 20488;
	// 826D81D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D81D4: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 826D81D8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D81DC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826D81E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D81E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D81E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D81EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D81F0: 386AD574  addi r3, r10, -0x2a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -10892;
	// 826D81F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D81F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D81FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D820C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8214: 4BD8EC0D  bl 0x82466e20
	ctx.lr = 0x826D8218;
	sub_82466E20(ctx, base);
	// 826D8218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D821C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8228 size=108
    let mut pc: u32 = 0x826D8228;
    'dispatch: loop {
        match pc {
            0x826D8228 => {
    //   block [0x826D8228..0x826D8294)
	// 826D8228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8234: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D823C: 38EB3E88  addi r7, r11, 0x3e88
	ctx.r[7].s64 = ctx.r[11].s64 + 16008;
	// 826D8240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D8244: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826D8248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D824C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8258: 386AD5A4  addi r3, r10, -0x2a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -10844;
	// 826D825C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D826C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D827C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8280: 4BD8EBA1  bl 0x82466e20
	ctx.lr = 0x826D8284;
	sub_82466E20(ctx, base);
	// 826D8284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D828C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8298 size=116
    let mut pc: u32 = 0x826D8298;
    'dispatch: loop {
        match pc {
            0x826D8298 => {
    //   block [0x826D8298..0x826D830C)
	// 826D8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D82A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D82A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D82A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D82AC: 390B3EB8  addi r8, r11, 0x3eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 16056;
	// 826D82B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D82B4: 392A504C  addi r9, r10, 0x504c
	ctx.r[9].s64 = ctx.r[10].s64 + 20556;
	// 826D82B8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D82BC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D82C0: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D82C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D82C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D82CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D82D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D82D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D82D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D82DC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D82E0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826D82E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D82E8: 386BD5D4  addi r3, r11, -0x2a2c
	ctx.r[3].s64 = ctx.r[11].s64 + -10796;
	// 826D82EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D82F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D82F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D82F8: 4BD8EB29  bl 0x82466e20
	ctx.lr = 0x826D82FC;
	sub_82466E20(ctx, base);
	// 826D82FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D8310 size=24
    let mut pc: u32 = 0x826D8310;
    'dispatch: loop {
        match pc {
            0x826D8310 => {
    //   block [0x826D8310..0x826D8328)
	// 826D8310: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8314: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D8318: 394AAF88  addi r10, r10, -0x5078
	ctx.r[10].s64 = ctx.r[10].s64 + -20600;
	// 826D831C: 816B3F78  lwz r11, 0x3f78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16248 as u32) ) } as u64;
	// 826D8320: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D8324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8328 size=112
    let mut pc: u32 = 0x826D8328;
    'dispatch: loop {
        match pc {
            0x826D8328 => {
    //   block [0x826D8328..0x826D8398)
	// 826D8328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D832C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8334: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D8338: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D833C: 392A5088  addi r9, r10, 0x5088
	ctx.r[9].s64 = ctx.r[10].s64 + 20616;
	// 826D8340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8344: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 826D8348: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D834C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826D8350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8360: 386AD604  addi r3, r10, -0x29fc
	ctx.r[3].s64 = ctx.r[10].s64 + -10748;
	// 826D8364: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D8368: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D836C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D837C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8384: 4BD8EA9D  bl 0x82466e20
	ctx.lr = 0x826D8388;
	sub_82466E20(ctx, base);
	// 826D8388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D838C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8398 size=108
    let mut pc: u32 = 0x826D8398;
    'dispatch: loop {
        match pc {
            0x826D8398 => {
    //   block [0x826D8398..0x826D8404)
	// 826D8398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D839C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D83A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D83A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D83A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D83AC: 38EB3F7C  addi r7, r11, 0x3f7c
	ctx.r[7].s64 = ctx.r[11].s64 + 16252;
	// 826D83B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D83B4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826D83B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D83BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D83C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D83C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D83C8: 386AD634  addi r3, r10, -0x29cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10700;
	// 826D83CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D83D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D83D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D83D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D83DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D83E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D83E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D83E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D83EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D83F0: 4BD8EA31  bl 0x82466e20
	ctx.lr = 0x826D83F4;
	sub_82466E20(ctx, base);
	// 826D83F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D83F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D83FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8408 size=108
    let mut pc: u32 = 0x826D8408;
    'dispatch: loop {
        match pc {
            0x826D8408 => {
    //   block [0x826D8408..0x826D8474)
	// 826D8408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D840C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8414: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D841C: 38EB3F94  addi r7, r11, 0x3f94
	ctx.r[7].s64 = ctx.r[11].s64 + 16276;
	// 826D8420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D8424: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826D8428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D842C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8438: 386AD664  addi r3, r10, -0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + -10652;
	// 826D843C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D844C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D845C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8460: 4BD8E9C1  bl 0x82466e20
	ctx.lr = 0x826D8464;
	sub_82466E20(ctx, base);
	// 826D8464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D846C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D8478 size=24
    let mut pc: u32 = 0x826D8478;
    'dispatch: loop {
        match pc {
            0x826D8478 => {
    //   block [0x826D8478..0x826D8490)
	// 826D8478: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D847C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D8480: 394AAFD0  addi r10, r10, -0x5030
	ctx.r[10].s64 = ctx.r[10].s64 + -20528;
	// 826D8484: 816B3FC4  lwz r11, 0x3fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16324 as u32) ) } as u64;
	// 826D8488: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D848C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8490 size=112
    let mut pc: u32 = 0x826D8490;
    'dispatch: loop {
        match pc {
            0x826D8490 => {
    //   block [0x826D8490..0x826D8500)
	// 826D8490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D849C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D84A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D84A4: 392A50C4  addi r9, r10, 0x50c4
	ctx.r[9].s64 = ctx.r[10].s64 + 20676;
	// 826D84A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D84AC: 390BAFD0  addi r8, r11, -0x5030
	ctx.r[8].s64 = ctx.r[11].s64 + -20528;
	// 826D84B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D84B4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826D84B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D84BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D84C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D84C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D84C8: 386AD694  addi r3, r10, -0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + -10604;
	// 826D84CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D84D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D84D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D84D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D84DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D84E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D84E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D84E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D84EC: 4BD8E935  bl 0x82466e20
	ctx.lr = 0x826D84F0;
	sub_82466E20(ctx, base);
	// 826D84F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D84F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D84F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D84FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8500 size=112
    let mut pc: u32 = 0x826D8500;
    'dispatch: loop {
        match pc {
            0x826D8500 => {
    //   block [0x826D8500..0x826D8570)
	// 826D8500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D850C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8510: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8514: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D851C: 390B3FC8  addi r8, r11, 0x3fc8
	ctx.r[8].s64 = ctx.r[11].s64 + 16328;
	// 826D8520: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D8524: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826D8528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D852C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8538: 386AD6C4  addi r3, r10, -0x293c
	ctx.r[3].s64 = ctx.r[10].s64 + -10556;
	// 826D853C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D854C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D855C: 4BD8E8C5  bl 0x82466e20
	ctx.lr = 0x826D8560;
	sub_82466E20(ctx, base);
	// 826D8560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8570 size=108
    let mut pc: u32 = 0x826D8570;
    'dispatch: loop {
        match pc {
            0x826D8570 => {
    //   block [0x826D8570..0x826D85DC)
	// 826D8570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D857C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8584: 38EB3FF8  addi r7, r11, 0x3ff8
	ctx.r[7].s64 = ctx.r[11].s64 + 16376;
	// 826D8588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D858C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826D8590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8594: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D859C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D85A0: 386AD6F4  addi r3, r10, -0x290c
	ctx.r[3].s64 = ctx.r[10].s64 + -10508;
	// 826D85A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D85A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D85AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D85B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D85B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D85B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D85BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D85C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D85C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D85C8: 4BD8E859  bl 0x82466e20
	ctx.lr = 0x826D85CC;
	sub_82466E20(ctx, base);
	// 826D85CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D85D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D85D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D85D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D85E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D85E0 size=108
    let mut pc: u32 = 0x826D85E0;
    'dispatch: loop {
        match pc {
            0x826D85E0 => {
    //   block [0x826D85E0..0x826D864C)
	// 826D85E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D85E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D85E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D85EC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D85F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D85F4: 38EB4028  addi r7, r11, 0x4028
	ctx.r[7].s64 = ctx.r[11].s64 + 16424;
	// 826D85F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D85FC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826D8600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8604: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D860C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8610: 386AD724  addi r3, r10, -0x28dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10460;
	// 826D8614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D861C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D862C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8638: 4BD8E7E9  bl 0x82466e20
	ctx.lr = 0x826D863C;
	sub_82466E20(ctx, base);
	// 826D863C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8650 size=108
    let mut pc: u32 = 0x826D8650;
    'dispatch: loop {
        match pc {
            0x826D8650 => {
    //   block [0x826D8650..0x826D86BC)
	// 826D8650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D865C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8664: 38EB4088  addi r7, r11, 0x4088
	ctx.r[7].s64 = ctx.r[11].s64 + 16520;
	// 826D8668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D866C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826D8670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8674: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D867C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8680: 386AD754  addi r3, r10, -0x28ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10412;
	// 826D8684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D868C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D869C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D86A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D86A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D86A8: 4BD8E779  bl 0x82466e20
	ctx.lr = 0x826D86AC;
	sub_82466E20(ctx, base);
	// 826D86AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D86B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D86B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D86B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D86C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D86C0 size=108
    let mut pc: u32 = 0x826D86C0;
    'dispatch: loop {
        match pc {
            0x826D86C0 => {
    //   block [0x826D86C0..0x826D872C)
	// 826D86C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D86C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D86C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D86CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D86D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D86D4: 38EB40B8  addi r7, r11, 0x40b8
	ctx.r[7].s64 = ctx.r[11].s64 + 16568;
	// 826D86D8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826D86DC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826D86E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D86E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D86E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D86EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D86F0: 386AD784  addi r3, r10, -0x287c
	ctx.r[3].s64 = ctx.r[10].s64 + -10364;
	// 826D86F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D86F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D86FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D870C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8718: 4BD8E709  bl 0x82466e20
	ctx.lr = 0x826D871C;
	sub_82466E20(ctx, base);
	// 826D871C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8730 size=108
    let mut pc: u32 = 0x826D8730;
    'dispatch: loop {
        match pc {
            0x826D8730 => {
    //   block [0x826D8730..0x826D879C)
	// 826D8730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D873C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8744: 38EB41D8  addi r7, r11, 0x41d8
	ctx.r[7].s64 = ctx.r[11].s64 + 16856;
	// 826D8748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D874C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826D8750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8754: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D875C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8760: 386AD7B4  addi r3, r10, -0x284c
	ctx.r[3].s64 = ctx.r[10].s64 + -10316;
	// 826D8764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D876C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D877C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8788: 4BD8E699  bl 0x82466e20
	ctx.lr = 0x826D878C;
	sub_82466E20(ctx, base);
	// 826D878C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D87A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D87A0 size=108
    let mut pc: u32 = 0x826D87A0;
    'dispatch: loop {
        match pc {
            0x826D87A0 => {
    //   block [0x826D87A0..0x826D880C)
	// 826D87A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D87A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D87A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D87AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D87B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D87B4: 38EB41F0  addi r7, r11, 0x41f0
	ctx.r[7].s64 = ctx.r[11].s64 + 16880;
	// 826D87B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D87BC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826D87C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D87C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D87C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D87CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D87D0: 386AD7E4  addi r3, r10, -0x281c
	ctx.r[3].s64 = ctx.r[10].s64 + -10268;
	// 826D87D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D87D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D87DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D87E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D87E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D87E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D87EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D87F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D87F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D87F8: 4BD8E629  bl 0x82466e20
	ctx.lr = 0x826D87FC;
	sub_82466E20(ctx, base);
	// 826D87FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8810 size=108
    let mut pc: u32 = 0x826D8810;
    'dispatch: loop {
        match pc {
            0x826D8810 => {
    //   block [0x826D8810..0x826D887C)
	// 826D8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D881C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8824: 38EB4208  addi r7, r11, 0x4208
	ctx.r[7].s64 = ctx.r[11].s64 + 16904;
	// 826D8828: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D882C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826D8830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8834: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D883C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8840: 386AD814  addi r3, r10, -0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10220;
	// 826D8844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D885C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8868: 4BD8E5B9  bl 0x82466e20
	ctx.lr = 0x826D886C;
	sub_82466E20(ctx, base);
	// 826D886C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8880 size=108
    let mut pc: u32 = 0x826D8880;
    'dispatch: loop {
        match pc {
            0x826D8880 => {
    //   block [0x826D8880..0x826D88EC)
	// 826D8880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D888C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8894: 38EB4220  addi r7, r11, 0x4220
	ctx.r[7].s64 = ctx.r[11].s64 + 16928;
	// 826D8898: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D889C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826D88A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D88A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D88A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D88AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D88B0: 386AD844  addi r3, r10, -0x27bc
	ctx.r[3].s64 = ctx.r[10].s64 + -10172;
	// 826D88B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D88B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D88BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D88C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D88C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D88C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D88CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D88D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D88D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D88D8: 4BD8E549  bl 0x82466e20
	ctx.lr = 0x826D88DC;
	sub_82466E20(ctx, base);
	// 826D88DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D88E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D88E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D88E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D88F0 size=108
    let mut pc: u32 = 0x826D88F0;
    'dispatch: loop {
        match pc {
            0x826D88F0 => {
    //   block [0x826D88F0..0x826D895C)
	// 826D88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D88F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D88F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D88FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8904: 38EB4238  addi r7, r11, 0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + 16952;
	// 826D8908: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D890C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826D8910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8914: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D891C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8920: 386AD874  addi r3, r10, -0x278c
	ctx.r[3].s64 = ctx.r[10].s64 + -10124;
	// 826D8924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D892C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8948: 4BD8E4D9  bl 0x82466e20
	ctx.lr = 0x826D894C;
	sub_82466E20(ctx, base);
	// 826D894C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8960 size=108
    let mut pc: u32 = 0x826D8960;
    'dispatch: loop {
        match pc {
            0x826D8960 => {
    //   block [0x826D8960..0x826D89CC)
	// 826D8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D896C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8974: 38EB4250  addi r7, r11, 0x4250
	ctx.r[7].s64 = ctx.r[11].s64 + 16976;
	// 826D8978: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D897C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826D8980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8984: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D898C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8990: 386AD8A4  addi r3, r10, -0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + -10076;
	// 826D8994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D899C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D89A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D89A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D89A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D89AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D89B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D89B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D89B8: 4BD8E469  bl 0x82466e20
	ctx.lr = 0x826D89BC;
	sub_82466E20(ctx, base);
	// 826D89BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D89C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D89C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D89C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D89D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D89D0 size=108
    let mut pc: u32 = 0x826D89D0;
    'dispatch: loop {
        match pc {
            0x826D89D0 => {
    //   block [0x826D89D0..0x826D8A3C)
	// 826D89D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D89D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D89D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D89DC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D89E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D89E4: 38EB4268  addi r7, r11, 0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + 17000;
	// 826D89E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D89EC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826D89F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D89F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D89F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D89FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8A00: 386AD8D4  addi r3, r10, -0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + -10028;
	// 826D8A04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8A28: 4BD8E3F9  bl 0x82466e20
	ctx.lr = 0x826D8A2C;
	sub_82466E20(ctx, base);
	// 826D8A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8A40 size=108
    let mut pc: u32 = 0x826D8A40;
    'dispatch: loop {
        match pc {
            0x826D8A40 => {
    //   block [0x826D8A40..0x826D8AAC)
	// 826D8A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8A4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8A54: 38EB42F8  addi r7, r11, 0x42f8
	ctx.r[7].s64 = ctx.r[11].s64 + 17144;
	// 826D8A58: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8A5C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826D8A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8A64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8A68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8A70: 386AD904  addi r3, r10, -0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9980;
	// 826D8A74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8A98: 4BD8E389  bl 0x82466e20
	ctx.lr = 0x826D8A9C;
	sub_82466E20(ctx, base);
	// 826D8A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8AB0 size=108
    let mut pc: u32 = 0x826D8AB0;
    'dispatch: loop {
        match pc {
            0x826D8AB0 => {
    //   block [0x826D8AB0..0x826D8B1C)
	// 826D8AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8ABC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8AC4: 38EB43B8  addi r7, r11, 0x43b8
	ctx.r[7].s64 = ctx.r[11].s64 + 17336;
	// 826D8AC8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D8ACC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826D8AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8AD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8AE0: 386AD934  addi r3, r10, -0x26cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9932;
	// 826D8AE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8B04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8B08: 4BD8E319  bl 0x82466e20
	ctx.lr = 0x826D8B0C;
	sub_82466E20(ctx, base);
	// 826D8B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8B20 size=108
    let mut pc: u32 = 0x826D8B20;
    'dispatch: loop {
        match pc {
            0x826D8B20 => {
    //   block [0x826D8B20..0x826D8B8C)
	// 826D8B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8B2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8B34: 38EB4490  addi r7, r11, 0x4490
	ctx.r[7].s64 = ctx.r[11].s64 + 17552;
	// 826D8B38: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8B3C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826D8B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8B50: 386AD964  addi r3, r10, -0x269c
	ctx.r[3].s64 = ctx.r[10].s64 + -9884;
	// 826D8B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8B78: 4BD8E2A9  bl 0x82466e20
	ctx.lr = 0x826D8B7C;
	sub_82466E20(ctx, base);
	// 826D8B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8B90 size=108
    let mut pc: u32 = 0x826D8B90;
    'dispatch: loop {
        match pc {
            0x826D8B90 => {
    //   block [0x826D8B90..0x826D8BFC)
	// 826D8B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8B9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8BA4: 38EB4550  addi r7, r11, 0x4550
	ctx.r[7].s64 = ctx.r[11].s64 + 17744;
	// 826D8BA8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D8BAC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826D8BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8BC0: 386AD994  addi r3, r10, -0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + -9836;
	// 826D8BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8BE8: 4BD8E239  bl 0x82466e20
	ctx.lr = 0x826D8BEC;
	sub_82466E20(ctx, base);
	// 826D8BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8C00 size=112
    let mut pc: u32 = 0x826D8C00;
    'dispatch: loop {
        match pc {
            0x826D8C00 => {
    //   block [0x826D8C00..0x826D8C70)
	// 826D8C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8C0C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D8C10: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826D8C14: 38EA45F8  addi r7, r10, 0x45f8
	ctx.r[7].s64 = ctx.r[10].s64 + 17912;
	// 826D8C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8C1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D8C20: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826D8C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8C28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8C2C: 396B50D8  addi r11, r11, 0x50d8
	ctx.r[11].s64 = ctx.r[11].s64 + 20696;
	// 826D8C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8C34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8C3C: 386AD9C4  addi r3, r10, -0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + -9788;
	// 826D8C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8C44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D8C48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8C4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D8C50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8C54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8C58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8C5C: 4BD8E1C5  bl 0x82466e20
	ctx.lr = 0x826D8C60;
	sub_82466E20(ctx, base);
	// 826D8C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8C70 size=112
    let mut pc: u32 = 0x826D8C70;
    'dispatch: loop {
        match pc {
            0x826D8C70 => {
    //   block [0x826D8C70..0x826D8CE0)
	// 826D8C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8C7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8C80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8C84: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8C8C: 390B4730  addi r8, r11, 0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + 18224;
	// 826D8C90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D8C94: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 826D8C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8C9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8CA8: 386AD9F4  addi r3, r10, -0x260c
	ctx.r[3].s64 = ctx.r[10].s64 + -9740;
	// 826D8CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8CCC: 4BD8E155  bl 0x82466e20
	ctx.lr = 0x826D8CD0;
	sub_82466E20(ctx, base);
	// 826D8CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8CE0 size=108
    let mut pc: u32 = 0x826D8CE0;
    'dispatch: loop {
        match pc {
            0x826D8CE0 => {
    //   block [0x826D8CE0..0x826D8D4C)
	// 826D8CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8CEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8CF4: 38EB4760  addi r7, r11, 0x4760
	ctx.r[7].s64 = ctx.r[11].s64 + 18272;
	// 826D8CF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D8CFC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826D8D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8D04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8D10: 386ADA24  addi r3, r10, -0x25dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9692;
	// 826D8D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8D38: 4BD8E0E9  bl 0x82466e20
	ctx.lr = 0x826D8D3C;
	sub_82466E20(ctx, base);
	// 826D8D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8D50 size=108
    let mut pc: u32 = 0x826D8D50;
    'dispatch: loop {
        match pc {
            0x826D8D50 => {
    //   block [0x826D8D50..0x826D8DBC)
	// 826D8D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8D5C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8D64: 38EB47C0  addi r7, r11, 0x47c0
	ctx.r[7].s64 = ctx.r[11].s64 + 18368;
	// 826D8D68: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826D8D6C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826D8D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8D74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8D80: 386ADA54  addi r3, r10, -0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9644;
	// 826D8D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8DA8: 4BD8E079  bl 0x82466e20
	ctx.lr = 0x826D8DAC;
	sub_82466E20(ctx, base);
	// 826D8DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8DC0 size=108
    let mut pc: u32 = 0x826D8DC0;
    'dispatch: loop {
        match pc {
            0x826D8DC0 => {
    //   block [0x826D8DC0..0x826D8E2C)
	// 826D8DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8DCC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8DD4: 38EB48C8  addi r7, r11, 0x48c8
	ctx.r[7].s64 = ctx.r[11].s64 + 18632;
	// 826D8DD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D8DDC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826D8DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8DE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8DF0: 386ADA84  addi r3, r10, -0x257c
	ctx.r[3].s64 = ctx.r[10].s64 + -9596;
	// 826D8DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8E18: 4BD8E009  bl 0x82466e20
	ctx.lr = 0x826D8E1C;
	sub_82466E20(ctx, base);
	// 826D8E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8E30 size=108
    let mut pc: u32 = 0x826D8E30;
    'dispatch: loop {
        match pc {
            0x826D8E30 => {
    //   block [0x826D8E30..0x826D8E9C)
	// 826D8E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8E3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8E44: 38EB49A0  addi r7, r11, 0x49a0
	ctx.r[7].s64 = ctx.r[11].s64 + 18848;
	// 826D8E48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D8E4C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826D8E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8E54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8E60: 386ADAB4  addi r3, r10, -0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + -9548;
	// 826D8E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8E88: 4BD8DF99  bl 0x82466e20
	ctx.lr = 0x826D8E8C;
	sub_82466E20(ctx, base);
	// 826D8E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8EA0 size=108
    let mut pc: u32 = 0x826D8EA0;
    'dispatch: loop {
        match pc {
            0x826D8EA0 => {
    //   block [0x826D8EA0..0x826D8F0C)
	// 826D8EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8EAC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8EB4: 38EB49E8  addi r7, r11, 0x49e8
	ctx.r[7].s64 = ctx.r[11].s64 + 18920;
	// 826D8EB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D8EBC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826D8EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8EC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8ED0: 386ADAE4  addi r3, r10, -0x251c
	ctx.r[3].s64 = ctx.r[10].s64 + -9500;
	// 826D8ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8EF8: 4BD8DF29  bl 0x82466e20
	ctx.lr = 0x826D8EFC;
	sub_82466E20(ctx, base);
	// 826D8EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8F10 size=108
    let mut pc: u32 = 0x826D8F10;
    'dispatch: loop {
        match pc {
            0x826D8F10 => {
    //   block [0x826D8F10..0x826D8F7C)
	// 826D8F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8F1C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8F20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D8F24: 38EB4A00  addi r7, r11, 0x4a00
	ctx.r[7].s64 = ctx.r[11].s64 + 18944;
	// 826D8F28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D8F2C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 826D8F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8F34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8F40: 386ADB14  addi r3, r10, -0x24ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9452;
	// 826D8F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8F68: 4BD8DEB9  bl 0x82466e20
	ctx.lr = 0x826D8F6C;
	sub_82466E20(ctx, base);
	// 826D8F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8F80 size=108
    let mut pc: u32 = 0x826D8F80;
    'dispatch: loop {
        match pc {
            0x826D8F80 => {
    //   block [0x826D8F80..0x826D8FEC)
	// 826D8F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8F8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D8F94: 38EB4A68  addi r7, r11, 0x4a68
	ctx.r[7].s64 = ctx.r[11].s64 + 19048;
	// 826D8F98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8F9C: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 826D8FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8FB0: 386ADB44  addi r3, r10, -0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9404;
	// 826D8FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8FD8: 4BD8DE49  bl 0x82466e20
	ctx.lr = 0x826D8FDC;
	sub_82466E20(ctx, base);
	// 826D8FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8FF0 size=116
    let mut pc: u32 = 0x826D8FF0;
    'dispatch: loop {
        match pc {
            0x826D8FF0 => {
    //   block [0x826D8FF0..0x826D9064)
	// 826D8FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8FFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9000: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D9004: 390B4B28  addi r8, r11, 0x4b28
	ctx.r[8].s64 = ctx.r[11].s64 + 19240;
	// 826D9008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D900C: 392A5154  addi r9, r10, 0x5154
	ctx.r[9].s64 = ctx.r[10].s64 + 20820;
	// 826D9010: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9014: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826D9018: 38AADB14  addi r5, r10, -0x24ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9452;
	// 826D901C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9024: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D902C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9034: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D9038: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 826D903C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D9040: 386BDB74  addi r3, r11, -0x248c
	ctx.r[3].s64 = ctx.r[11].s64 + -9356;
	// 826D9044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D9048: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D904C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9050: 4BD8DDD1  bl 0x82466e20
	ctx.lr = 0x826D9054;
	sub_82466E20(ctx, base);
	// 826D9054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D905C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9068 size=112
    let mut pc: u32 = 0x826D9068;
    'dispatch: loop {
        match pc {
            0x826D9068 => {
    //   block [0x826D9068..0x826D90D8)
	// 826D9068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9078: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D907C: 38AAFEB4  addi r5, r10, -0x14c
	ctx.r[5].s64 = ctx.r[10].s64 + -332;
	// 826D9080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9084: 390B4BB8  addi r8, r11, 0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 19384;
	// 826D9088: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D908C: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 826D9090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D909C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D90A0: 386ADBA4  addi r3, r10, -0x245c
	ctx.r[3].s64 = ctx.r[10].s64 + -9308;
	// 826D90A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D90A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D90AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D90B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D90B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D90B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D90BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D90C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D90C4: 4BD8DD5D  bl 0x82466e20
	ctx.lr = 0x826D90C8;
	sub_82466E20(ctx, base);
	// 826D90C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D90CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D90D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D90D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D90D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D90D8 size=96
    let mut pc: u32 = 0x826D90D8;
    'dispatch: loop {
        match pc {
            0x826D90D8 => {
    //   block [0x826D90D8..0x826D9138)
	// 826D90D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D90DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D90E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D90E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D90E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D90EC: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 826D90F0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D90F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D90F8: 386ADBD4  addi r3, r10, -0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + -9260;
	// 826D90FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9104: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D910C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D911C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9124: 4BD8DCFD  bl 0x82466e20
	ctx.lr = 0x826D9128;
	sub_82466E20(ctx, base);
	// 826D9128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D912C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9138 size=24
    let mut pc: u32 = 0x826D9138;
    'dispatch: loop {
        match pc {
            0x826D9138 => {
    //   block [0x826D9138..0x826D9150)
	// 826D9138: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D913C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9140: 394AB048  addi r10, r10, -0x4fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -20408;
	// 826D9144: 816B4A64  lwz r11, 0x4a64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19044 as u32) ) } as u64;
	// 826D9148: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D914C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9150 size=116
    let mut pc: u32 = 0x826D9150;
    'dispatch: loop {
        match pc {
            0x826D9150 => {
    //   block [0x826D9150..0x826D91C4)
	// 826D9150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D915C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D9160: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D9164: 390BB048  addi r8, r11, -0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -20408;
	// 826D9168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D916C: 392A51A0  addi r9, r10, 0x51a0
	ctx.r[9].s64 = ctx.r[10].s64 + 20896;
	// 826D9170: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9174: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D9178: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D917C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D918C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D9190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9194: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D9198: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 826D919C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D91A0: 386BDC04  addi r3, r11, -0x23fc
	ctx.r[3].s64 = ctx.r[11].s64 + -9212;
	// 826D91A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D91A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D91AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D91B0: 4BD8DC71  bl 0x82466e20
	ctx.lr = 0x826D91B4;
	sub_82466E20(ctx, base);
	// 826D91B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D91B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D91BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D91C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D91C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D91C8 size=104
    let mut pc: u32 = 0x826D91C8;
    'dispatch: loop {
        match pc {
            0x826D91C8 => {
    //   block [0x826D91C8..0x826D9230)
	// 826D91C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D91CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D91D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D91D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D91D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D91DC: 392A51CC  addi r9, r10, 0x51cc
	ctx.r[9].s64 = ctx.r[10].s64 + 20940;
	// 826D91E0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D91E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D91E8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D91EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D91F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D91F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D91F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D91FC: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 826D9200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D920C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9214: 386ADC34  addi r3, r10, -0x23cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9164;
	// 826D9218: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D921C: 4BD8DC05  bl 0x82466e20
	ctx.lr = 0x826D9220;
	sub_82466E20(ctx, base);
	// 826D9220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D922C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9230 size=96
    let mut pc: u32 = 0x826D9230;
    'dispatch: loop {
        match pc {
            0x826D9230 => {
    //   block [0x826D9230..0x826D9290)
	// 826D9230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D923C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9244: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 826D9248: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D924C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9250: 386ADC64  addi r3, r10, -0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + -9116;
	// 826D9254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D925C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D926C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9270: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9278: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D927C: 4BD8DBA5  bl 0x82466e20
	ctx.lr = 0x826D9280;
	sub_82466E20(ctx, base);
	// 826D9280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D928C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9290 size=96
    let mut pc: u32 = 0x826D9290;
    'dispatch: loop {
        match pc {
            0x826D9290 => {
    //   block [0x826D9290..0x826D92F0)
	// 826D9290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D929C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D92A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D92A4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 826D92A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D92AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D92B0: 386ADC94  addi r3, r10, -0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + -9068;
	// 826D92B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D92B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D92BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D92C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D92C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D92C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D92CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D92D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D92D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D92D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D92DC: 4BD8DB45  bl 0x82466e20
	ctx.lr = 0x826D92E0;
	sub_82466E20(ctx, base);
	// 826D92E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D92E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D92E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D92EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D92F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D92F0 size=100
    let mut pc: u32 = 0x826D92F0;
    'dispatch: loop {
        match pc {
            0x826D92F0 => {
    //   block [0x826D92F0..0x826D9354)
	// 826D92F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D92F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D92F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D92FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9304: 38AADC34  addi r5, r10, -0x23cc
	ctx.r[5].s64 = ctx.r[10].s64 + -9164;
	// 826D9308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D930C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9310: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 826D9314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D931C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9324: 386ADCC4  addi r3, r10, -0x233c
	ctx.r[3].s64 = ctx.r[10].s64 + -9020;
	// 826D9328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D932C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9330: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D933C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9340: 4BD8DAE1  bl 0x82466e20
	ctx.lr = 0x826D9344;
	sub_82466E20(ctx, base);
	// 826D9344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9358 size=112
    let mut pc: u32 = 0x826D9358;
    'dispatch: loop {
        match pc {
            0x826D9358 => {
    //   block [0x826D9358..0x826D93C8)
	// 826D9358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D935C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9368: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D936C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D9370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9374: 390B4C20  addi r8, r11, 0x4c20
	ctx.r[8].s64 = ctx.r[11].s64 + 19488;
	// 826D9378: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D937C: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 826D9380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9390: 386ADCF4  addi r3, r10, -0x230c
	ctx.r[3].s64 = ctx.r[10].s64 + -8972;
	// 826D9394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D939C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D93A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D93A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D93A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D93AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D93B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D93B4: 4BD8DA6D  bl 0x82466e20
	ctx.lr = 0x826D93B8;
	sub_82466E20(ctx, base);
	// 826D93B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D93BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D93C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D93C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D93C8 size=112
    let mut pc: u32 = 0x826D93C8;
    'dispatch: loop {
        match pc {
            0x826D93C8 => {
    //   block [0x826D93C8..0x826D9438)
	// 826D93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D93D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D93D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D93D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D93DC: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D93E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D93E4: 390B4C68  addi r8, r11, 0x4c68
	ctx.r[8].s64 = ctx.r[11].s64 + 19560;
	// 826D93E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D93EC: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 826D93F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D93F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D93F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D93FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9400: 386ADD24  addi r3, r10, -0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8924;
	// 826D9404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D940C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D941C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9424: 4BD8D9FD  bl 0x82466e20
	ctx.lr = 0x826D9428;
	sub_82466E20(ctx, base);
	// 826D9428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9438 size=100
    let mut pc: u32 = 0x826D9438;
    'dispatch: loop {
        match pc {
            0x826D9438 => {
    //   block [0x826D9438..0x826D949C)
	// 826D9438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D944C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D9450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9458: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 826D945C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D946C: 386ADD54  addi r3, r10, -0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8876;
	// 826D9470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D947C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9488: 4BD8D999  bl 0x82466e20
	ctx.lr = 0x826D948C;
	sub_82466E20(ctx, base);
	// 826D948C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D94A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D94A0 size=112
    let mut pc: u32 = 0x826D94A0;
    'dispatch: loop {
        match pc {
            0x826D94A0 => {
    //   block [0x826D94A0..0x826D9510)
	// 826D94A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D94A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D94A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D94AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D94B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D94B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D94B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D94BC: 390B4C80  addi r8, r11, 0x4c80
	ctx.r[8].s64 = ctx.r[11].s64 + 19584;
	// 826D94C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D94C4: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 826D94C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D94CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D94D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D94D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D94D8: 386ADD84  addi r3, r10, -0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + -8828;
	// 826D94DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D94E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D94E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D94E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D94EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D94F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D94F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D94F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D94FC: 4BD8D925  bl 0x82466e20
	ctx.lr = 0x826D9500;
	sub_82466E20(ctx, base);
	// 826D9500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D950C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9510 size=96
    let mut pc: u32 = 0x826D9510;
    'dispatch: loop {
        match pc {
            0x826D9510 => {
    //   block [0x826D9510..0x826D9570)
	// 826D9510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D951C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9524: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 826D9528: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9530: 386ADDB4  addi r3, r10, -0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + -8780;
	// 826D9534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D953C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D954C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D955C: 4BD8D8C5  bl 0x82466e20
	ctx.lr = 0x826D9560;
	sub_82466E20(ctx, base);
	// 826D9560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D956C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9570 size=112
    let mut pc: u32 = 0x826D9570;
    'dispatch: loop {
        match pc {
            0x826D9570 => {
    //   block [0x826D9570..0x826D95E0)
	// 826D9570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D957C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9580: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9584: 38AADDB4  addi r5, r10, -0x224c
	ctx.r[5].s64 = ctx.r[10].s64 + -8780;
	// 826D9588: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D958C: 390B4CB0  addi r8, r11, 0x4cb0
	ctx.r[8].s64 = ctx.r[11].s64 + 19632;
	// 826D9590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9594: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 826D9598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D959C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D95A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D95A8: 386ADDE4  addi r3, r10, -0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + -8732;
	// 826D95AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D95B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D95B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D95B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D95BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D95C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D95C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D95C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D95CC: 4BD8D855  bl 0x82466e20
	ctx.lr = 0x826D95D0;
	sub_82466E20(ctx, base);
	// 826D95D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D95D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D95D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D95DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D95E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D95E0 size=112
    let mut pc: u32 = 0x826D95E0;
    'dispatch: loop {
        match pc {
            0x826D95E0 => {
    //   block [0x826D95E0..0x826D9650)
	// 826D95E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D95E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D95E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D95EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D95F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D95F4: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826D95F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D95FC: 390B4CC8  addi r8, r11, 0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 19656;
	// 826D9600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9604: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 826D9608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D960C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9618: 386ADE14  addi r3, r10, -0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8684;
	// 826D961C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D962C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D963C: 4BD8D7E5  bl 0x82466e20
	ctx.lr = 0x826D9640;
	sub_82466E20(ctx, base);
	// 826D9640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9650 size=112
    let mut pc: u32 = 0x826D9650;
    'dispatch: loop {
        match pc {
            0x826D9650 => {
    //   block [0x826D9650..0x826D96C0)
	// 826D9650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D965C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9660: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9664: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D966C: 390B4CE0  addi r8, r11, 0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 19680;
	// 826D9670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D9674: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 826D9678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D967C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9688: 386ADE44  addi r3, r10, -0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8636;
	// 826D968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D96A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D96A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D96A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D96AC: 4BD8D775  bl 0x82466e20
	ctx.lr = 0x826D96B0;
	sub_82466E20(ctx, base);
	// 826D96B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D96B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D96B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D96BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D96C0 size=36
    let mut pc: u32 = 0x826D96C0;
    'dispatch: loop {
        match pc {
            0x826D96C0 => {
    //   block [0x826D96C0..0x826D96E4)
	// 826D96C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D96C4: 814B4D2C  lwz r10, 0x4d2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19756 as u32) ) } as u64;
	// 826D96C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D96CC: 396BB078  addi r11, r11, -0x4f88
	ctx.r[11].s64 = ctx.r[11].s64 + -20360;
	// 826D96D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D96D4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D96D8: 814A4C1C  lwz r10, 0x4c1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19484 as u32) ) } as u64;
	// 826D96DC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D96E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D96E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D96E8 size=108
    let mut pc: u32 = 0x826D96E8;
    'dispatch: loop {
        match pc {
            0x826D96E8 => {
    //   block [0x826D96E8..0x826D9754)
	// 826D96E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D96EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D96F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D96F4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D96F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D96FC: 38EBB078  addi r7, r11, -0x4f88
	ctx.r[7].s64 = ctx.r[11].s64 + -20360;
	// 826D9700: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D9704: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 826D9708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D970C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9718: 386ADE74  addi r3, r10, -0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + -8588;
	// 826D971C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D972C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D973C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9740: 4BD8D6E1  bl 0x82466e20
	ctx.lr = 0x826D9744;
	sub_82466E20(ctx, base);
	// 826D9744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D974C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9758 size=24
    let mut pc: u32 = 0x826D9758;
    'dispatch: loop {
        match pc {
            0x826D9758 => {
    //   block [0x826D9758..0x826D9770)
	// 826D9758: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D975C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9760: 394AB120  addi r10, r10, -0x4ee0
	ctx.r[10].s64 = ctx.r[10].s64 + -20192;
	// 826D9764: 816B4C1C  lwz r11, 0x4c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19484 as u32) ) } as u64;
	// 826D9768: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826D976C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9770 size=116
    let mut pc: u32 = 0x826D9770;
    'dispatch: loop {
        match pc {
            0x826D9770 => {
    //   block [0x826D9770..0x826D97E4)
	// 826D9770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D977C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9780: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D9784: 390AB120  addi r8, r10, -0x4ee0
	ctx.r[8].s64 = ctx.r[10].s64 + -20192;
	// 826D9788: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D978C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9790: 38AADE74  addi r5, r10, -0x218c
	ctx.r[5].s64 = ctx.r[10].s64 + -8588;
	// 826D9794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9798: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D979C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D97A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D97A4: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 826D97A8: 396B526C  addi r11, r11, 0x526c
	ctx.r[11].s64 = ctx.r[11].s64 + 21100;
	// 826D97AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D97B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D97B4: 386ADEA4  addi r3, r10, -0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + -8540;
	// 826D97B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D97BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D97C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D97C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D97C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D97CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D97D0: 4BD8D651  bl 0x82466e20
	ctx.lr = 0x826D97D4;
	sub_82466E20(ctx, base);
	// 826D97D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D97D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D97DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D97E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D97E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D97E8 size=112
    let mut pc: u32 = 0x826D97E8;
    'dispatch: loop {
        match pc {
            0x826D97E8 => {
    //   block [0x826D97E8..0x826D9858)
	// 826D97E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D97EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D97F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D97F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D97F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D97FC: 38AADE74  addi r5, r10, -0x218c
	ctx.r[5].s64 = ctx.r[10].s64 + -8588;
	// 826D9800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9804: 390B4D30  addi r8, r11, 0x4d30
	ctx.r[8].s64 = ctx.r[11].s64 + 19760;
	// 826D9808: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D980C: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 826D9810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D981C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9820: 386ADED4  addi r3, r10, -0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + -8492;
	// 826D9824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D982C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D983C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9844: 4BD8D5DD  bl 0x82466e20
	ctx.lr = 0x826D9848;
	sub_82466E20(ctx, base);
	// 826D9848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D984C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9858 size=24
    let mut pc: u32 = 0x826D9858;
    'dispatch: loop {
        match pc {
            0x826D9858 => {
    //   block [0x826D9858..0x826D9870)
	// 826D9858: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D985C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9860: 394AB210  addi r10, r10, -0x4df0
	ctx.r[10].s64 = ctx.r[10].s64 + -19952;
	// 826D9864: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9868: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826D986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9870 size=116
    let mut pc: u32 = 0x826D9870;
    'dispatch: loop {
        match pc {
            0x826D9870 => {
    //   block [0x826D9870..0x826D98E4)
	// 826D9870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D987C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9880: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9884: 392B5230  addi r9, r11, 0x5230
	ctx.r[9].s64 = ctx.r[11].s64 + 21040;
	// 826D9888: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D988C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9890: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826D9894: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826D9898: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D989C: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 826D98A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D98A4: 396BB210  addi r11, r11, -0x4df0
	ctx.r[11].s64 = ctx.r[11].s64 + -19952;
	// 826D98A8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D98AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D98B0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D98B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D98B8: 386ADF04  addi r3, r10, -0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8444;
	// 826D98BC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826D98C0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D98C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D98C8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D98CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D98D0: 4BD8D551  bl 0x82466e20
	ctx.lr = 0x826D98D4;
	sub_82466E20(ctx, base);
	// 826D98D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D98D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D98DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D98E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D98E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D98E8 size=100
    let mut pc: u32 = 0x826D98E8;
    'dispatch: loop {
        match pc {
            0x826D98E8 => {
    //   block [0x826D98E8..0x826D994C)
	// 826D98E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D98EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D98F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D98F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D98F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D98FC: 38AAE024  addi r5, r10, -0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -8156;
	// 826D9900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9908: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 826D990C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D991C: 386ADF34  addi r3, r10, -0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8396;
	// 826D9920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D992C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9938: 4BD8D4E9  bl 0x82466e20
	ctx.lr = 0x826D993C;
	sub_82466E20(ctx, base);
	// 826D993C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9950 size=108
    let mut pc: u32 = 0x826D9950;
    'dispatch: loop {
        match pc {
            0x826D9950 => {
    //   block [0x826D9950..0x826D99BC)
	// 826D9950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D995C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9964: 38EB4DA8  addi r7, r11, 0x4da8
	ctx.r[7].s64 = ctx.r[11].s64 + 19880;
	// 826D9968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D996C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 826D9970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9980: 386ADF64  addi r3, r10, -0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + -8348;
	// 826D9984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D999C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D99A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D99A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D99A8: 4BD8D479  bl 0x82466e20
	ctx.lr = 0x826D99AC;
	sub_82466E20(ctx, base);
	// 826D99AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D99B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D99B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D99B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D99C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D99C0 size=112
    let mut pc: u32 = 0x826D99C0;
    'dispatch: loop {
        match pc {
            0x826D99C0 => {
    //   block [0x826D99C0..0x826D9A30)
	// 826D99C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D99C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D99C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D99CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D99D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D99D4: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D99D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D99DC: 390B4E08  addi r8, r11, 0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + 19976;
	// 826D99E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D99E4: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 826D99E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D99EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D99F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D99F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D99F8: 386ADF94  addi r3, r10, -0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + -8300;
	// 826D99FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9A1C: 4BD8D405  bl 0x82466e20
	ctx.lr = 0x826D9A20;
	sub_82466E20(ctx, base);
	// 826D9A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9A30 size=108
    let mut pc: u32 = 0x826D9A30;
    'dispatch: loop {
        match pc {
            0x826D9A30 => {
    //   block [0x826D9A30..0x826D9A9C)
	// 826D9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9A3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9A40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9A44: 38EB4E68  addi r7, r11, 0x4e68
	ctx.r[7].s64 = ctx.r[11].s64 + 20072;
	// 826D9A48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9A4C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 826D9A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9A54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9A60: 386ADFC4  addi r3, r10, -0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + -8252;
	// 826D9A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9A88: 4BD8D399  bl 0x82466e20
	ctx.lr = 0x826D9A8C;
	sub_82466E20(ctx, base);
	// 826D9A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9AA0 size=28
    let mut pc: u32 = 0x826D9AA0;
    'dispatch: loop {
        match pc {
            0x826D9AA0 => {
    //   block [0x826D9AA0..0x826D9ABC)
	// 826D9AA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9AA4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9AA8: 394AB300  addi r10, r10, -0x4d00
	ctx.r[10].s64 = ctx.r[10].s64 + -19712;
	// 826D9AAC: 816B4E80  lwz r11, 0x4e80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20096 as u32) ) } as u64;
	// 826D9AB0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9AB4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826D9AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9AC0 size=112
    let mut pc: u32 = 0x826D9AC0;
    'dispatch: loop {
        match pc {
            0x826D9AC0 => {
    //   block [0x826D9AC0..0x826D9B30)
	// 826D9AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9ACC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9AD0: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826D9AD4: 38EAB300  addi r7, r10, -0x4d00
	ctx.r[7].s64 = ctx.r[10].s64 + -19712;
	// 826D9AD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9ADC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9AE0: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 826D9AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9AE8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9AEC: 396B5330  addi r11, r11, 0x5330
	ctx.r[11].s64 = ctx.r[11].s64 + 21296;
	// 826D9AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9AF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9AFC: 386ADFF4  addi r3, r10, -0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + -8204;
	// 826D9B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9B04: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D9B08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9B0C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D9B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9B14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9B18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9B1C: 4BD8D305  bl 0x82466e20
	ctx.lr = 0x826D9B20;
	sub_82466E20(ctx, base);
	// 826D9B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9B30 size=24
    let mut pc: u32 = 0x826D9B30;
    'dispatch: loop {
        match pc {
            0x826D9B30 => {
    //   block [0x826D9B30..0x826D9B48)
	// 826D9B30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9B34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9B38: 394AB468  addi r10, r10, -0x4b98
	ctx.r[10].s64 = ctx.r[10].s64 + -19352;
	// 826D9B3C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9B40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9B48 size=116
    let mut pc: u32 = 0x826D9B48;
    'dispatch: loop {
        match pc {
            0x826D9B48 => {
    //   block [0x826D9B48..0x826D9BBC)
	// 826D9B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9B54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9B58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9B5C: 392B5304  addi r9, r11, 0x5304
	ctx.r[9].s64 = ctx.r[11].s64 + 21252;
	// 826D9B60: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9B64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9B68: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 826D9B6C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D9B70: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D9B74: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 826D9B78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9B7C: 396BB468  addi r11, r11, -0x4b98
	ctx.r[11].s64 = ctx.r[11].s64 + -19352;
	// 826D9B80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D9B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9B88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D9B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9B90: 386AE024  addi r3, r10, -0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -8156;
	// 826D9B94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D9B98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D9B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9BA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D9BA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9BA8: 4BD8D279  bl 0x82466e20
	ctx.lr = 0x826D9BAC;
	sub_82466E20(ctx, base);
	// 826D9BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9BC0 size=108
    let mut pc: u32 = 0x826D9BC0;
    'dispatch: loop {
        match pc {
            0x826D9BC0 => {
    //   block [0x826D9BC0..0x826D9C2C)
	// 826D9BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9BCC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9BD4: 38EB4E88  addi r7, r11, 0x4e88
	ctx.r[7].s64 = ctx.r[11].s64 + 20104;
	// 826D9BD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D9BDC: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 826D9BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9BF0: 386AE054  addi r3, r10, -0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + -8108;
	// 826D9BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9C18: 4BD8D209  bl 0x82466e20
	ctx.lr = 0x826D9C1C;
	sub_82466E20(ctx, base);
	// 826D9C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9C30 size=24
    let mut pc: u32 = 0x826D9C30;
    'dispatch: loop {
        match pc {
            0x826D9C30 => {
    //   block [0x826D9C30..0x826D9C48)
	// 826D9C30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9C34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9C38: 394AB510  addi r10, r10, -0x4af0
	ctx.r[10].s64 = ctx.r[10].s64 + -19184;
	// 826D9C3C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9C40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9C48 size=116
    let mut pc: u32 = 0x826D9C48;
    'dispatch: loop {
        match pc {
            0x826D9C48 => {
    //   block [0x826D9C48..0x826D9CBC)
	// 826D9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9C54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9C58: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D9C5C: 390AB510  addi r8, r10, -0x4af0
	ctx.r[8].s64 = ctx.r[10].s64 + -19184;
	// 826D9C60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9C64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9C68: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9C6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9C70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D9C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9C7C: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 826D9C80: 396B5390  addi r11, r11, 0x5390
	ctx.r[11].s64 = ctx.r[11].s64 + 21392;
	// 826D9C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9C88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9C8C: 386AE084  addi r3, r10, -0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -8060;
	// 826D9C90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D9C94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9C98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D9C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9CA8: 4BD8D179  bl 0x82466e20
	ctx.lr = 0x826D9CAC;
	sub_82466E20(ctx, base);
	// 826D9CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9CC0 size=112
    let mut pc: u32 = 0x826D9CC0;
    'dispatch: loop {
        match pc {
            0x826D9CC0 => {
    //   block [0x826D9CC0..0x826D9D30)
	// 826D9CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9CD0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9CD4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9CDC: 390B4EE8  addi r8, r11, 0x4ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 20200;
	// 826D9CE0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D9CE4: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 826D9CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9CEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9CF8: 386AE0B4  addi r3, r10, -0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -8012;
	// 826D9CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9D1C: 4BD8D105  bl 0x82466e20
	ctx.lr = 0x826D9D20;
	sub_82466E20(ctx, base);
	// 826D9D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9D30 size=112
    let mut pc: u32 = 0x826D9D30;
    'dispatch: loop {
        match pc {
            0x826D9D30 => {
    //   block [0x826D9D30..0x826D9DA0)
	// 826D9D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9D3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9D44: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9D48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9D4C: 390B4F78  addi r8, r11, 0x4f78
	ctx.r[8].s64 = ctx.r[11].s64 + 20344;
	// 826D9D50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D9D54: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 826D9D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9D60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9D68: 386AE0E4  addi r3, r10, -0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7964;
	// 826D9D6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9D8C: 4BD8D095  bl 0x82466e20
	ctx.lr = 0x826D9D90;
	sub_82466E20(ctx, base);
	// 826D9D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9DA0 size=112
    let mut pc: u32 = 0x826D9DA0;
    'dispatch: loop {
        match pc {
            0x826D9DA0 => {
    //   block [0x826D9DA0..0x826D9E10)
	// 826D9DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9DB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9DB4: 38AADF04  addi r5, r10, -0x20fc
	ctx.r[5].s64 = ctx.r[10].s64 + -8444;
	// 826D9DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9DBC: 390B4FD8  addi r8, r11, 0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 20440;
	// 826D9DC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D9DC4: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 826D9DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9DD8: 386AE114  addi r3, r10, -0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + -7916;
	// 826D9DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9DFC: 4BD8D025  bl 0x82466e20
	ctx.lr = 0x826D9E00;
	sub_82466E20(ctx, base);
	// 826D9E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9E10 size=112
    let mut pc: u32 = 0x826D9E10;
    'dispatch: loop {
        match pc {
            0x826D9E10 => {
    //   block [0x826D9E10..0x826D9E80)
	// 826D9E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9E24: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9E28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9E2C: 390B5008  addi r8, r11, 0x5008
	ctx.r[8].s64 = ctx.r[11].s64 + 20488;
	// 826D9E30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D9E34: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 826D9E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9E48: 386AE144  addi r3, r10, -0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -7868;
	// 826D9E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9E6C: 4BD8CFB5  bl 0x82466e20
	ctx.lr = 0x826D9E70;
	sub_82466E20(ctx, base);
	// 826D9E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


