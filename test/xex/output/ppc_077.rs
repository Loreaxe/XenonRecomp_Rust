pub fn sub_825F8B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8B28 size=112
    let mut pc: u32 = 0x825F8B28;
    'dispatch: loop {
        match pc {
            0x825F8B28 => {
    //   block [0x825F8B28..0x825F8B98)
	// 825F8B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8B34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8B38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8B3C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8B44: 390B03A8  addi r8, r11, 0x3a8
	ctx.r[8].s64 = ctx.r[11].s64 + 936;
	// 825F8B48: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F8B4C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 825F8B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8B60: 386AF30C  addi r3, r10, -0xcf4
	ctx.r[3].s64 = ctx.r[10].s64 + -3316;
	// 825F8B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8B84: 4BE6E29D  bl 0x82466e20
	ctx.lr = 0x825F8B88;
	sub_82466E20(ctx, base);
	// 825F8B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8B98 size=108
    let mut pc: u32 = 0x825F8B98;
    'dispatch: loop {
        match pc {
            0x825F8B98 => {
    //   block [0x825F8B98..0x825F8C04)
	// 825F8B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8BA4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8BAC: 38EB0450  addi r7, r11, 0x450
	ctx.r[7].s64 = ctx.r[11].s64 + 1104;
	// 825F8BB0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F8BB4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 825F8BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8BBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8BC8: 386AF33C  addi r3, r10, -0xcc4
	ctx.r[3].s64 = ctx.r[10].s64 + -3268;
	// 825F8BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8BF0: 4BE6E231  bl 0x82466e20
	ctx.lr = 0x825F8BF4;
	sub_82466E20(ctx, base);
	// 825F8BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8C08 size=112
    let mut pc: u32 = 0x825F8C08;
    'dispatch: loop {
        match pc {
            0x825F8C08 => {
    //   block [0x825F8C08..0x825F8C78)
	// 825F8C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8C14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8C1C: 392A7EF8  addi r9, r10, 0x7ef8
	ctx.r[9].s64 = ctx.r[10].s64 + 32504;
	// 825F8C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C24: 390B04FC  addi r8, r11, 0x4fc
	ctx.r[8].s64 = ctx.r[11].s64 + 1276;
	// 825F8C28: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F8C2C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 825F8C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8C40: 386AF36C  addi r3, r10, -0xc94
	ctx.r[3].s64 = ctx.r[10].s64 + -3220;
	// 825F8C44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8C48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8C64: 4BE6E1BD  bl 0x82466e20
	ctx.lr = 0x825F8C68;
	sub_82466E20(ctx, base);
	// 825F8C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8C78 size=100
    let mut pc: u32 = 0x825F8C78;
    'dispatch: loop {
        match pc {
            0x825F8C78 => {
    //   block [0x825F8C78..0x825F8CDC)
	// 825F8C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8C8C: 38AAFB7C  addi r5, r10, -0x484
	ctx.r[5].s64 = ctx.r[10].s64 + -1156;
	// 825F8C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8C98: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 825F8C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8CAC: 386AF39C  addi r3, r10, -0xc64
	ctx.r[3].s64 = ctx.r[10].s64 + -3172;
	// 825F8CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8CB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8CB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F8CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8CC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8CC8: 4BE6E159  bl 0x82466e20
	ctx.lr = 0x825F8CCC;
	sub_82466E20(ctx, base);
	// 825F8CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8CE0 size=24
    let mut pc: u32 = 0x825F8CE0;
    'dispatch: loop {
        match pc {
            0x825F8CE0 => {
    //   block [0x825F8CE0..0x825F8CF8)
	// 825F8CE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8CE4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8CE8: 394A8E20  addi r10, r10, -0x71e0
	ctx.r[10].s64 = ctx.r[10].s64 + -29152;
	// 825F8CEC: 816B0530  lwz r11, 0x530(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1328 as u32) ) } as u64;
	// 825F8CF0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825F8CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8CF8 size=108
    let mut pc: u32 = 0x825F8CF8;
    'dispatch: loop {
        match pc {
            0x825F8CF8 => {
    //   block [0x825F8CF8..0x825F8D64)
	// 825F8CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8D04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8D08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8D0C: 38EB8E20  addi r7, r11, -0x71e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29152;
	// 825F8D10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F8D14: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 825F8D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8D1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8D28: 386AF3CC  addi r3, r10, -0xc34
	ctx.r[3].s64 = ctx.r[10].s64 + -3124;
	// 825F8D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8D50: 4BE6E0D1  bl 0x82466e20
	ctx.lr = 0x825F8D54;
	sub_82466E20(ctx, base);
	// 825F8D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8D68 size=112
    let mut pc: u32 = 0x825F8D68;
    'dispatch: loop {
        match pc {
            0x825F8D68 => {
    //   block [0x825F8D68..0x825F8DD8)
	// 825F8D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8D78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8D7C: 392A7F78  addi r9, r10, 0x7f78
	ctx.r[9].s64 = ctx.r[10].s64 + 32632;
	// 825F8D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8D84: 390B0538  addi r8, r11, 0x538
	ctx.r[8].s64 = ctx.r[11].s64 + 1336;
	// 825F8D88: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F8D8C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 825F8D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8DA0: 386AF3FC  addi r3, r10, -0xc04
	ctx.r[3].s64 = ctx.r[10].s64 + -3076;
	// 825F8DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8DA8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F8DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8DC4: 4BE6E05D  bl 0x82466e20
	ctx.lr = 0x825F8DC8;
	sub_82466E20(ctx, base);
	// 825F8DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8DD8 size=112
    let mut pc: u32 = 0x825F8DD8;
    'dispatch: loop {
        match pc {
            0x825F8DD8 => {
    //   block [0x825F8DD8..0x825F8E48)
	// 825F8DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8DE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8DEC: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8DF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8DF4: 390B0580  addi r8, r11, 0x580
	ctx.r[8].s64 = ctx.r[11].s64 + 1408;
	// 825F8DF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F8DFC: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 825F8E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8E04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8E10: 386AF42C  addi r3, r10, -0xbd4
	ctx.r[3].s64 = ctx.r[10].s64 + -3028;
	// 825F8E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8E34: 4BE6DFED  bl 0x82466e20
	ctx.lr = 0x825F8E38;
	sub_82466E20(ctx, base);
	// 825F8E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8E48 size=116
    let mut pc: u32 = 0x825F8E48;
    'dispatch: loop {
        match pc {
            0x825F8E48 => {
    //   block [0x825F8E48..0x825F8EBC)
	// 825F8E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8E54: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F8E58: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F8E5C: 390A05B0  addi r8, r10, 0x5b0
	ctx.r[8].s64 = ctx.r[10].s64 + 1456;
	// 825F8E60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8E68: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8E6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8E70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8E7C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 825F8E80: 396B7FA0  addi r11, r11, 0x7fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 32672;
	// 825F8E84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8E8C: 386AF45C  addi r3, r10, -0xba4
	ctx.r[3].s64 = ctx.r[10].s64 + -2980;
	// 825F8E90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8E94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8E98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8EA8: 4BE6DF79  bl 0x82466e20
	ctx.lr = 0x825F8EAC;
	sub_82466E20(ctx, base);
	// 825F8EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8EC0 size=100
    let mut pc: u32 = 0x825F8EC0;
    'dispatch: loop {
        match pc {
            0x825F8EC0 => {
    //   block [0x825F8EC0..0x825F8F24)
	// 825F8EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8ED4: 38AAF45C  addi r5, r10, -0xba4
	ctx.r[5].s64 = ctx.r[10].s64 + -2980;
	// 825F8ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8EE0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 825F8EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8EF4: 386AF48C  addi r3, r10, -0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + -2932;
	// 825F8EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8EFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8F00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F8F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8F08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8F10: 4BE6DF11  bl 0x82466e20
	ctx.lr = 0x825F8F14;
	sub_82466E20(ctx, base);
	// 825F8F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8F28 size=24
    let mut pc: u32 = 0x825F8F28;
    'dispatch: loop {
        match pc {
            0x825F8F28 => {
    //   block [0x825F8F28..0x825F8F40)
	// 825F8F28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8F2C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8F30: 394A8E68  addi r10, r10, -0x7198
	ctx.r[10].s64 = ctx.r[10].s64 + -29080;
	// 825F8F34: 816B0534  lwz r11, 0x534(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1332 as u32) ) } as u64;
	// 825F8F38: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F8F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8F40 size=116
    let mut pc: u32 = 0x825F8F40;
    'dispatch: loop {
        match pc {
            0x825F8F40 => {
    //   block [0x825F8F40..0x825F8FB4)
	// 825F8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8F4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8F50: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8F54: 392B7FDC  addi r9, r11, 0x7fdc
	ctx.r[9].s64 = ctx.r[11].s64 + 32732;
	// 825F8F58: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8F60: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F8F64: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F8F68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8F6C: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 825F8F70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8F74: 396B8E68  addi r11, r11, -0x7198
	ctx.r[11].s64 = ctx.r[11].s64 + -29080;
	// 825F8F78: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F8F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8F80: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8F88: 386AF4BC  addi r3, r10, -0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + -2884;
	// 825F8F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8F90: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F8F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8F98: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F8F9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8FA0: 4BE6DE81  bl 0x82466e20
	ctx.lr = 0x825F8FA4;
	sub_82466E20(ctx, base);
	// 825F8FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8FB8 size=116
    let mut pc: u32 = 0x825F8FB8;
    'dispatch: loop {
        match pc {
            0x825F8FB8 => {
    //   block [0x825F8FB8..0x825F902C)
	// 825F8FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8FC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F8FC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8FCC: 392B8038  addi r9, r11, -0x7fc8
	ctx.r[9].s64 = ctx.r[11].s64 + -32712;
	// 825F8FD0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8FD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8FD8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 825F8FDC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F8FE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8FE4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 825F8FE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8FEC: 396B0660  addi r11, r11, 0x660
	ctx.r[11].s64 = ctx.r[11].s64 + 1632;
	// 825F8FF0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F8FF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8FF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9000: 386AF4EC  addi r3, r10, -0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + -2836;
	// 825F9004: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9008: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F900C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9010: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F9014: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F9018: 4BE6DE09  bl 0x82466e20
	ctx.lr = 0x825F901C;
	sub_82466E20(ctx, base);
	// 825F901C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9030 size=108
    let mut pc: u32 = 0x825F9030;
    'dispatch: loop {
        match pc {
            0x825F9030 => {
    //   block [0x825F9030..0x825F909C)
	// 825F9030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F903C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9044: 38EB07B0  addi r7, r11, 0x7b0
	ctx.r[7].s64 = ctx.r[11].s64 + 1968;
	// 825F9048: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F904C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 825F9050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9054: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F905C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9060: 386AF51C  addi r3, r10, -0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + -2788;
	// 825F9064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F906C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F907C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9088: 4BE6DD99  bl 0x82466e20
	ctx.lr = 0x825F908C;
	sub_82466E20(ctx, base);
	// 825F908C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F90A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F90A0 size=24
    let mut pc: u32 = 0x825F90A0;
    'dispatch: loop {
        match pc {
            0x825F90A0 => {
    //   block [0x825F90A0..0x825F90B8)
	// 825F90A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F90A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F90A8: 394A8F10  addi r10, r10, -0x70f0
	ctx.r[10].s64 = ctx.r[10].s64 + -28912;
	// 825F90AC: 816B065C  lwz r11, 0x65c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1628 as u32) ) } as u64;
	// 825F90B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F90B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F90B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F90B8 size=116
    let mut pc: u32 = 0x825F90B8;
    'dispatch: loop {
        match pc {
            0x825F90B8 => {
    //   block [0x825F90B8..0x825F912C)
	// 825F90B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F90BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F90C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F90C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F90C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F90CC: 390B8F10  addi r8, r11, -0x70f0
	ctx.r[8].s64 = ctx.r[11].s64 + -28912;
	// 825F90D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F90D4: 392A80D0  addi r9, r10, -0x7f30
	ctx.r[9].s64 = ctx.r[10].s64 + -32560;
	// 825F90D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F90DC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 825F90E0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F90E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F90E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F90EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F90F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F90F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F90F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F90FC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9100: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 825F9104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9108: 386BF54C  addi r3, r11, -0xab4
	ctx.r[3].s64 = ctx.r[11].s64 + -2740;
	// 825F910C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F9110: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9118: 4BE6DD09  bl 0x82466e20
	ctx.lr = 0x825F911C;
	sub_82466E20(ctx, base);
	// 825F911C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9130 size=112
    let mut pc: u32 = 0x825F9130;
    'dispatch: loop {
        match pc {
            0x825F9130 => {
    //   block [0x825F9130..0x825F91A0)
	// 825F9130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F913C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9140: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9144: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F9148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F914C: 390B0814  addi r8, r11, 0x814
	ctx.r[8].s64 = ctx.r[11].s64 + 2068;
	// 825F9150: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9154: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 825F9158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F915C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9168: 386AF57C  addi r3, r10, -0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + -2692;
	// 825F916C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F917C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F918C: 4BE6DC95  bl 0x82466e20
	ctx.lr = 0x825F9190;
	sub_82466E20(ctx, base);
	// 825F9190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F919C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F91A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F91A0 size=24
    let mut pc: u32 = 0x825F91A0;
    'dispatch: loop {
        match pc {
            0x825F91A0 => {
    //   block [0x825F91A0..0x825F91B8)
	// 825F91A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F91A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F91A8: 394A90A8  addi r10, r10, -0x6f58
	ctx.r[10].s64 = ctx.r[10].s64 + -28504;
	// 825F91AC: 816B0844  lwz r11, 0x844(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2116 as u32) ) } as u64;
	// 825F91B0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825F91B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F91B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F91B8 size=116
    let mut pc: u32 = 0x825F91B8;
    'dispatch: loop {
        match pc {
            0x825F91B8 => {
    //   block [0x825F91B8..0x825F922C)
	// 825F91B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F91BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F91C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F91C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F91C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F91CC: 392B8108  addi r9, r11, -0x7ef8
	ctx.r[9].s64 = ctx.r[11].s64 + -32504;
	// 825F91D0: 38AAF4EC  addi r5, r10, -0xb14
	ctx.r[5].s64 = ctx.r[10].s64 + -2836;
	// 825F91D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F91D8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 825F91DC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F91E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F91E4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 825F91E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F91EC: 396B90A8  addi r11, r11, -0x6f58
	ctx.r[11].s64 = ctx.r[11].s64 + -28504;
	// 825F91F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F91F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F91F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F91FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9200: 386AF5AC  addi r3, r10, -0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + -2644;
	// 825F9204: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9208: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F920C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9210: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F9214: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F9218: 4BE6DC09  bl 0x82466e20
	ctx.lr = 0x825F921C;
	sub_82466E20(ctx, base);
	// 825F921C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9230 size=112
    let mut pc: u32 = 0x825F9230;
    'dispatch: loop {
        match pc {
            0x825F9230 => {
    //   block [0x825F9230..0x825F92A0)
	// 825F9230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F923C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9240: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9244: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F9248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F924C: 390B0848  addi r8, r11, 0x848
	ctx.r[8].s64 = ctx.r[11].s64 + 2120;
	// 825F9250: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9254: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 825F9258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F925C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9268: 386AF5DC  addi r3, r10, -0xa24
	ctx.r[3].s64 = ctx.r[10].s64 + -2596;
	// 825F926C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F927C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F928C: 4BE6DB95  bl 0x82466e20
	ctx.lr = 0x825F9290;
	sub_82466E20(ctx, base);
	// 825F9290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F929C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F92A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F92A0 size=100
    let mut pc: u32 = 0x825F92A0;
    'dispatch: loop {
        match pc {
            0x825F92A0 => {
    //   block [0x825F92A0..0x825F9304)
	// 825F92A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F92A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F92A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F92AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F92B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F92B4: 38AAFB7C  addi r5, r10, -0x484
	ctx.r[5].s64 = ctx.r[10].s64 + -1156;
	// 825F92B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F92BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F92C0: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 825F92C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F92C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F92CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F92D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F92D4: 386AF60C  addi r3, r10, -0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + -2548;
	// 825F92D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F92DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F92E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F92E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F92E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F92EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F92F0: 4BE6DB31  bl 0x82466e20
	ctx.lr = 0x825F92F4;
	sub_82466E20(ctx, base);
	// 825F92F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F92F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F92FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9308 size=112
    let mut pc: u32 = 0x825F9308;
    'dispatch: loop {
        match pc {
            0x825F9308 => {
    //   block [0x825F9308..0x825F9378)
	// 825F9308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F930C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9314: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9318: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825F931C: 38EA0860  addi r7, r10, 0x860
	ctx.r[7].s64 = ctx.r[10].s64 + 2144;
	// 825F9320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9324: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9328: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 825F932C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9330: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9334: 396B8170  addi r11, r11, -0x7e90
	ctx.r[11].s64 = ctx.r[11].s64 + -32400;
	// 825F9338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F933C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9344: 386AF63C  addi r3, r10, -0x9c4
	ctx.r[3].s64 = ctx.r[10].s64 + -2500;
	// 825F9348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F934C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9350: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9354: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9358: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F935C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9364: 4BE6DABD  bl 0x82466e20
	ctx.lr = 0x825F9368;
	sub_82466E20(ctx, base);
	// 825F9368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F936C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9378 size=112
    let mut pc: u32 = 0x825F9378;
    'dispatch: loop {
        match pc {
            0x825F9378 => {
    //   block [0x825F9378..0x825F93E8)
	// 825F9378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F937C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9388: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F938C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9394: 390B0998  addi r8, r11, 0x998
	ctx.r[8].s64 = ctx.r[11].s64 + 2456;
	// 825F9398: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F939C: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 825F93A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F93A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F93A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F93AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F93B0: 386AF66C  addi r3, r10, -0x994
	ctx.r[3].s64 = ctx.r[10].s64 + -2452;
	// 825F93B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F93B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F93BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F93C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F93C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F93C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F93CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F93D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F93D4: 4BE6DA4D  bl 0x82466e20
	ctx.lr = 0x825F93D8;
	sub_82466E20(ctx, base);
	// 825F93D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F93DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F93E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F93E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F93E8 size=112
    let mut pc: u32 = 0x825F93E8;
    'dispatch: loop {
        match pc {
            0x825F93E8 => {
    //   block [0x825F93E8..0x825F9458)
	// 825F93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F93F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F93F4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F93F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F93FC: 38EA09C8  addi r7, r10, 0x9c8
	ctx.r[7].s64 = ctx.r[10].s64 + 2504;
	// 825F9400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9404: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9408: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 825F940C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9410: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9414: 396B81C4  addi r11, r11, -0x7e3c
	ctx.r[11].s64 = ctx.r[11].s64 + -32316;
	// 825F9418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F941C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9420: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9424: 386AF69C  addi r3, r10, -0x964
	ctx.r[3].s64 = ctx.r[10].s64 + -2404;
	// 825F9428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F942C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9434: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9438: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F943C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9440: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9444: 4BE6D9DD  bl 0x82466e20
	ctx.lr = 0x825F9448;
	sub_82466E20(ctx, base);
	// 825F9448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F944C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9458 size=112
    let mut pc: u32 = 0x825F9458;
    'dispatch: loop {
        match pc {
            0x825F9458 => {
    //   block [0x825F9458..0x825F94C8)
	// 825F9458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F945C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9468: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F946C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9474: 390B09F8  addi r8, r11, 0x9f8
	ctx.r[8].s64 = ctx.r[11].s64 + 2552;
	// 825F9478: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F947C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 825F9480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F948C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9490: 386AF6CC  addi r3, r10, -0x934
	ctx.r[3].s64 = ctx.r[10].s64 + -2356;
	// 825F9494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F949C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F94A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F94A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F94A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F94AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F94B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F94B4: 4BE6D96D  bl 0x82466e20
	ctx.lr = 0x825F94B8;
	sub_82466E20(ctx, base);
	// 825F94B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F94BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F94C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F94C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F94C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F94C8 size=108
    let mut pc: u32 = 0x825F94C8;
    'dispatch: loop {
        match pc {
            0x825F94C8 => {
    //   block [0x825F94C8..0x825F9534)
	// 825F94C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F94CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F94D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F94D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F94D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F94DC: 38EB0A10  addi r7, r11, 0xa10
	ctx.r[7].s64 = ctx.r[11].s64 + 2576;
	// 825F94E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F94E4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 825F94E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F94EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F94F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F94F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F94F8: 386AF6FC  addi r3, r10, -0x904
	ctx.r[3].s64 = ctx.r[10].s64 + -2308;
	// 825F94FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F950C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F951C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9520: 4BE6D901  bl 0x82466e20
	ctx.lr = 0x825F9524;
	sub_82466E20(ctx, base);
	// 825F9524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F952C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9538 size=112
    let mut pc: u32 = 0x825F9538;
    'dispatch: loop {
        match pc {
            0x825F9538 => {
    //   block [0x825F9538..0x825F95A8)
	// 825F9538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F953C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9548: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F954C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9554: 390B0A28  addi r8, r11, 0xa28
	ctx.r[8].s64 = ctx.r[11].s64 + 2600;
	// 825F9558: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F955C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 825F9560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F956C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9570: 386AF72C  addi r3, r10, -0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + -2260;
	// 825F9574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F957C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F958C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9594: 4BE6D88D  bl 0x82466e20
	ctx.lr = 0x825F9598;
	sub_82466E20(ctx, base);
	// 825F9598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F959C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F95A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F95A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F95A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F95A8 size=112
    let mut pc: u32 = 0x825F95A8;
    'dispatch: loop {
        match pc {
            0x825F95A8 => {
    //   block [0x825F95A8..0x825F9618)
	// 825F95A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F95AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F95B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F95B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F95B8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F95BC: 38EA0A40  addi r7, r10, 0xa40
	ctx.r[7].s64 = ctx.r[10].s64 + 2624;
	// 825F95C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F95C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F95C8: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 825F95CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F95D0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F95D4: 396B81D0  addi r11, r11, -0x7e30
	ctx.r[11].s64 = ctx.r[11].s64 + -32304;
	// 825F95D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F95DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F95E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F95E4: 386AF75C  addi r3, r10, -0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + -2212;
	// 825F95E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F95EC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F95F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F95F4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F95F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F95FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9600: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9604: 4BE6D81D  bl 0x82466e20
	ctx.lr = 0x825F9608;
	sub_82466E20(ctx, base);
	// 825F9608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9618 size=112
    let mut pc: u32 = 0x825F9618;
    'dispatch: loop {
        match pc {
            0x825F9618 => {
    //   block [0x825F9618..0x825F9688)
	// 825F9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9624: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9628: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 825F962C: 38EA0B18  addi r7, r10, 0xb18
	ctx.r[7].s64 = ctx.r[10].s64 + 2840;
	// 825F9630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9634: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9638: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 825F963C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9644: 396B8210  addi r11, r11, -0x7df0
	ctx.r[11].s64 = ctx.r[11].s64 + -32240;
	// 825F9648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F964C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9654: 386AF78C  addi r3, r10, -0x874
	ctx.r[3].s64 = ctx.r[10].s64 + -2164;
	// 825F9658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F965C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F966C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9674: 4BE6D7AD  bl 0x82466e20
	ctx.lr = 0x825F9678;
	sub_82466E20(ctx, base);
	// 825F9678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9688 size=108
    let mut pc: u32 = 0x825F9688;
    'dispatch: loop {
        match pc {
            0x825F9688 => {
    //   block [0x825F9688..0x825F96F4)
	// 825F9688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9694: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F969C: 38EB0C68  addi r7, r11, 0xc68
	ctx.r[7].s64 = ctx.r[11].s64 + 3176;
	// 825F96A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F96A4: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 825F96A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F96AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F96B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F96B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F96B8: 386AF7BC  addi r3, r10, -0x844
	ctx.r[3].s64 = ctx.r[10].s64 + -2116;
	// 825F96BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F96C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F96C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F96C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F96CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F96D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F96D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F96D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F96DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F96E0: 4BE6D741  bl 0x82466e20
	ctx.lr = 0x825F96E4;
	sub_82466E20(ctx, base);
	// 825F96E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F96E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F96EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F96F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F96F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F96F8 size=116
    let mut pc: u32 = 0x825F96F8;
    'dispatch: loop {
        match pc {
            0x825F96F8 => {
    //   block [0x825F96F8..0x825F976C)
	// 825F96F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F96FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9704: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9708: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825F970C: 390A0CC8  addi r8, r10, 0xcc8
	ctx.r[8].s64 = ctx.r[10].s64 + 3272;
	// 825F9710: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9714: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9718: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F971C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9720: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F972C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 825F9730: 396B82B0  addi r11, r11, -0x7d50
	ctx.r[11].s64 = ctx.r[11].s64 + -32080;
	// 825F9734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F973C: 386AF7EC  addi r3, r10, -0x814
	ctx.r[3].s64 = ctx.r[10].s64 + -2068;
	// 825F9740: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9748: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9758: 4BE6D6C9  bl 0x82466e20
	ctx.lr = 0x825F975C;
	sub_82466E20(ctx, base);
	// 825F975C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9770 size=116
    let mut pc: u32 = 0x825F9770;
    'dispatch: loop {
        match pc {
            0x825F9770 => {
    //   block [0x825F9770..0x825F97E4)
	// 825F9770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F977C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9780: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825F9784: 390A0DE8  addi r8, r10, 0xde8
	ctx.r[8].s64 = ctx.r[10].s64 + 3560;
	// 825F9788: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F978C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9790: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9794: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9798: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F979C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F97A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F97A4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 825F97A8: 396B82EC  addi r11, r11, -0x7d14
	ctx.r[11].s64 = ctx.r[11].s64 + -32020;
	// 825F97AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F97B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F97B4: 386AF81C  addi r3, r10, -0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + -2020;
	// 825F97B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F97BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F97C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F97C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F97C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F97CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F97D0: 4BE6D651  bl 0x82466e20
	ctx.lr = 0x825F97D4;
	sub_82466E20(ctx, base);
	// 825F97D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F97D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F97DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F97E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F97E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F97E8 size=108
    let mut pc: u32 = 0x825F97E8;
    'dispatch: loop {
        match pc {
            0x825F97E8 => {
    //   block [0x825F97E8..0x825F9854)
	// 825F97E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F97EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F97F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F97F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F97F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F97FC: 38EB0E48  addi r7, r11, 0xe48
	ctx.r[7].s64 = ctx.r[11].s64 + 3656;
	// 825F9800: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F9804: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 825F9808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F980C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9818: 386AF84C  addi r3, r10, -0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + -1972;
	// 825F981C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F982C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F983C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9840: 4BE6D5E1  bl 0x82466e20
	ctx.lr = 0x825F9844;
	sub_82466E20(ctx, base);
	// 825F9844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F984C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9858 size=112
    let mut pc: u32 = 0x825F9858;
    'dispatch: loop {
        match pc {
            0x825F9858 => {
    //   block [0x825F9858..0x825F98C8)
	// 825F9858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F985C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9864: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9868: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F986C: 38EA0EF0  addi r7, r10, 0xef0
	ctx.r[7].s64 = ctx.r[10].s64 + 3824;
	// 825F9870: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9878: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 825F987C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9880: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9884: 396B8300  addi r11, r11, -0x7d00
	ctx.r[11].s64 = ctx.r[11].s64 + -32000;
	// 825F9888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F988C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9890: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9894: 386AF87C  addi r3, r10, -0x784
	ctx.r[3].s64 = ctx.r[10].s64 + -1924;
	// 825F9898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F989C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F98A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F98A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F98A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F98AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F98B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F98B4: 4BE6D56D  bl 0x82466e20
	ctx.lr = 0x825F98B8;
	sub_82466E20(ctx, base);
	// 825F98B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F98BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F98C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F98C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F98C8 size=112
    let mut pc: u32 = 0x825F98C8;
    'dispatch: loop {
        match pc {
            0x825F98C8 => {
    //   block [0x825F98C8..0x825F9938)
	// 825F98C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F98CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F98D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F98D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F98D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F98DC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F98E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F98E4: 390B0F68  addi r8, r11, 0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + 3944;
	// 825F98E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F98EC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 825F98F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F98F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F98F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F98FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9900: 386AF8AC  addi r3, r10, -0x754
	ctx.r[3].s64 = ctx.r[10].s64 + -1876;
	// 825F9904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F990C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9924: 4BE6D4FD  bl 0x82466e20
	ctx.lr = 0x825F9928;
	sub_82466E20(ctx, base);
	// 825F9928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F992C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9938 size=112
    let mut pc: u32 = 0x825F9938;
    'dispatch: loop {
        match pc {
            0x825F9938 => {
    //   block [0x825F9938..0x825F99A8)
	// 825F9938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9944: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9948: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F994C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9954: 390B0FB0  addi r8, r11, 0xfb0
	ctx.r[8].s64 = ctx.r[11].s64 + 4016;
	// 825F9958: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825F995C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 825F9960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9964: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F996C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9970: 386AF8DC  addi r3, r10, -0x724
	ctx.r[3].s64 = ctx.r[10].s64 + -1828;
	// 825F9974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9994: 4BE6D48D  bl 0x82466e20
	ctx.lr = 0x825F9998;
	sub_82466E20(ctx, base);
	// 825F9998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F999C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F99A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F99A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F99A8 size=100
    let mut pc: u32 = 0x825F99A8;
    'dispatch: loop {
        match pc {
            0x825F99A8 => {
    //   block [0x825F99A8..0x825F9A0C)
	// 825F99A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F99AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F99B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F99B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F99B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F99BC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F99C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F99C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F99C8: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 825F99CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F99D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F99D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F99D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F99DC: 386AF90C  addi r3, r10, -0x6f4
	ctx.r[3].s64 = ctx.r[10].s64 + -1780;
	// 825F99E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F99E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F99E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F99EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F99F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F99F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F99F8: 4BE6D429  bl 0x82466e20
	ctx.lr = 0x825F99FC;
	sub_82466E20(ctx, base);
	// 825F99FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9A10 size=112
    let mut pc: u32 = 0x825F9A10;
    'dispatch: loop {
        match pc {
            0x825F9A10 => {
    //   block [0x825F9A10..0x825F9A80)
	// 825F9A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9A1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9A24: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9A2C: 390B10B8  addi r8, r11, 0x10b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4280;
	// 825F9A30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9A34: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 825F9A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9A3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9A48: 386AF93C  addi r3, r10, -0x6c4
	ctx.r[3].s64 = ctx.r[10].s64 + -1732;
	// 825F9A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9A6C: 4BE6D3B5  bl 0x82466e20
	ctx.lr = 0x825F9A70;
	sub_82466E20(ctx, base);
	// 825F9A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9A80 size=112
    let mut pc: u32 = 0x825F9A80;
    'dispatch: loop {
        match pc {
            0x825F9A80 => {
    //   block [0x825F9A80..0x825F9AF0)
	// 825F9A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9A8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9A94: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F9A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9A9C: 390B10D0  addi r8, r11, 0x10d0
	ctx.r[8].s64 = ctx.r[11].s64 + 4304;
	// 825F9AA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9AA4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 825F9AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9AAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9AB8: 386AF96C  addi r3, r10, -0x694
	ctx.r[3].s64 = ctx.r[10].s64 + -1684;
	// 825F9ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9ADC: 4BE6D345  bl 0x82466e20
	ctx.lr = 0x825F9AE0;
	sub_82466E20(ctx, base);
	// 825F9AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9AF0 size=112
    let mut pc: u32 = 0x825F9AF0;
    'dispatch: loop {
        match pc {
            0x825F9AF0 => {
    //   block [0x825F9AF0..0x825F9B60)
	// 825F9AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9AFC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9B00: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F9B04: 38EA1100  addi r7, r10, 0x1100
	ctx.r[7].s64 = ctx.r[10].s64 + 4352;
	// 825F9B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9B0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9B10: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 825F9B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9B18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9B1C: 396B8320  addi r11, r11, -0x7ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -31968;
	// 825F9B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9B2C: 386AF99C  addi r3, r10, -0x664
	ctx.r[3].s64 = ctx.r[10].s64 + -1636;
	// 825F9B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9B34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9B38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9B3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9B40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9B44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9B48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9B4C: 4BE6D2D5  bl 0x82466e20
	ctx.lr = 0x825F9B50;
	sub_82466E20(ctx, base);
	// 825F9B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9B60 size=112
    let mut pc: u32 = 0x825F9B60;
    'dispatch: loop {
        match pc {
            0x825F9B60 => {
    //   block [0x825F9B60..0x825F9BD0)
	// 825F9B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9B74: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9B7C: 390B1178  addi r8, r11, 0x1178
	ctx.r[8].s64 = ctx.r[11].s64 + 4472;
	// 825F9B80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9B84: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 825F9B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9B98: 386AF9CC  addi r3, r10, -0x634
	ctx.r[3].s64 = ctx.r[10].s64 + -1588;
	// 825F9B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9BBC: 4BE6D265  bl 0x82466e20
	ctx.lr = 0x825F9BC0;
	sub_82466E20(ctx, base);
	// 825F9BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F9BD0 size=24
    let mut pc: u32 = 0x825F9BD0;
    'dispatch: loop {
        match pc {
            0x825F9BD0 => {
    //   block [0x825F9BD0..0x825F9BE8)
	// 825F9BD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9BD4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F9BD8: 394A91F8  addi r10, r10, -0x6e08
	ctx.r[10].s64 = ctx.r[10].s64 + -28168;
	// 825F9BDC: 816B11A8  lwz r11, 0x11a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4520 as u32) ) } as u64;
	// 825F9BE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F9BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9BE8 size=116
    let mut pc: u32 = 0x825F9BE8;
    'dispatch: loop {
        match pc {
            0x825F9BE8 => {
    //   block [0x825F9BE8..0x825F9C5C)
	// 825F9BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9BF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F9BF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9BFC: 390B91F8  addi r8, r11, -0x6e08
	ctx.r[8].s64 = ctx.r[11].s64 + -28168;
	// 825F9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9C04: 392A8360  addi r9, r10, -0x7ca0
	ctx.r[9].s64 = ctx.r[10].s64 + -31904;
	// 825F9C08: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9C0C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825F9C10: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9C14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9C2C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9C30: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 825F9C34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9C38: 386BF9FC  addi r3, r11, -0x604
	ctx.r[3].s64 = ctx.r[11].s64 + -1540;
	// 825F9C3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9C40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9C48: 4BE6D1D9  bl 0x82466e20
	ctx.lr = 0x825F9C4C;
	sub_82466E20(ctx, base);
	// 825F9C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9C60 size=116
    let mut pc: u32 = 0x825F9C60;
    'dispatch: loop {
        match pc {
            0x825F9C60 => {
    //   block [0x825F9C60..0x825F9CD4)
	// 825F9C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9C6C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9C70: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825F9C74: 390A11B0  addi r8, r10, 0x11b0
	ctx.r[8].s64 = ctx.r[10].s64 + 4528;
	// 825F9C78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9C80: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9C88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9C94: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 825F9C98: 396B8374  addi r11, r11, -0x7c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -31884;
	// 825F9C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9CA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9CA4: 386AFA2C  addi r3, r10, -0x5d4
	ctx.r[3].s64 = ctx.r[10].s64 + -1492;
	// 825F9CA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9CB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9CC0: 4BE6D161  bl 0x82466e20
	ctx.lr = 0x825F9CC4;
	sub_82466E20(ctx, base);
	// 825F9CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9CD8 size=112
    let mut pc: u32 = 0x825F9CD8;
    'dispatch: loop {
        match pc {
            0x825F9CD8 => {
    //   block [0x825F9CD8..0x825F9D48)
	// 825F9CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9CE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9CE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9CEC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9CF4: 390B1270  addi r8, r11, 0x1270
	ctx.r[8].s64 = ctx.r[11].s64 + 4720;
	// 825F9CF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9CFC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 825F9D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9D04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9D10: 386AFA5C  addi r3, r10, -0x5a4
	ctx.r[3].s64 = ctx.r[10].s64 + -1444;
	// 825F9D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9D34: 4BE6D0ED  bl 0x82466e20
	ctx.lr = 0x825F9D38;
	sub_82466E20(ctx, base);
	// 825F9D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9D48 size=112
    let mut pc: u32 = 0x825F9D48;
    'dispatch: loop {
        match pc {
            0x825F9D48 => {
    //   block [0x825F9D48..0x825F9DB8)
	// 825F9D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9D54: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F9D5C: 38EA1288  addi r7, r10, 0x1288
	ctx.r[7].s64 = ctx.r[10].s64 + 4744;
	// 825F9D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9D64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9D68: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 825F9D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9D70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9D74: 396B8398  addi r11, r11, -0x7c68
	ctx.r[11].s64 = ctx.r[11].s64 + -31848;
	// 825F9D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9D7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9D84: 386AFA8C  addi r3, r10, -0x574
	ctx.r[3].s64 = ctx.r[10].s64 + -1396;
	// 825F9D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9D8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9D94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9D9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9DA4: 4BE6D07D  bl 0x82466e20
	ctx.lr = 0x825F9DA8;
	sub_82466E20(ctx, base);
	// 825F9DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9DB8 size=112
    let mut pc: u32 = 0x825F9DB8;
    'dispatch: loop {
        match pc {
            0x825F9DB8 => {
    //   block [0x825F9DB8..0x825F9E28)
	// 825F9DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9DC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9DC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9DCC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9DD4: 390B12B8  addi r8, r11, 0x12b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4792;
	// 825F9DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9DDC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 825F9DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9DF0: 386AFABC  addi r3, r10, -0x544
	ctx.r[3].s64 = ctx.r[10].s64 + -1348;
	// 825F9DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9E14: 4BE6D00D  bl 0x82466e20
	ctx.lr = 0x825F9E18;
	sub_82466E20(ctx, base);
	// 825F9E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9E28 size=116
    let mut pc: u32 = 0x825F9E28;
    'dispatch: loop {
        match pc {
            0x825F9E28 => {
    //   block [0x825F9E28..0x825F9E9C)
	// 825F9E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9E34: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9E38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F9E3C: 390A12D0  addi r8, r10, 0x12d0
	ctx.r[8].s64 = ctx.r[10].s64 + 4816;
	// 825F9E40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9E44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9E48: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9E4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9E50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9E5C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 825F9E60: 396B83A4  addi r11, r11, -0x7c5c
	ctx.r[11].s64 = ctx.r[11].s64 + -31836;
	// 825F9E64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9E68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9E6C: 386AFAEC  addi r3, r10, -0x514
	ctx.r[3].s64 = ctx.r[10].s64 + -1300;
	// 825F9E70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9E78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9E88: 4BE6CF99  bl 0x82466e20
	ctx.lr = 0x825F9E8C;
	sub_82466E20(ctx, base);
	// 825F9E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9EA0 size=112
    let mut pc: u32 = 0x825F9EA0;
    'dispatch: loop {
        match pc {
            0x825F9EA0 => {
    //   block [0x825F9EA0..0x825F9F10)
	// 825F9EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9EB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9EB4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9EBC: 390B1378  addi r8, r11, 0x1378
	ctx.r[8].s64 = ctx.r[11].s64 + 4984;
	// 825F9EC0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 825F9EC4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 825F9EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9ED8: 386AFB1C  addi r3, r10, -0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + -1252;
	// 825F9EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9EFC: 4BE6CF25  bl 0x82466e20
	ctx.lr = 0x825F9F00;
	sub_82466E20(ctx, base);
	// 825F9F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9F10 size=112
    let mut pc: u32 = 0x825F9F10;
    'dispatch: loop {
        match pc {
            0x825F9F10 => {
    //   block [0x825F9F10..0x825F9F80)
	// 825F9F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9F20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9F24: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9F2C: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 825F9F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9F34: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 825F9F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9F48: 386AFB4C  addi r3, r10, -0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + -1204;
	// 825F9F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9F6C: 4BE6CEB5  bl 0x82466e20
	ctx.lr = 0x825F9F70;
	sub_82466E20(ctx, base);
	// 825F9F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9F80 size=116
    let mut pc: u32 = 0x825F9F80;
    'dispatch: loop {
        match pc {
            0x825F9F80 => {
    //   block [0x825F9F80..0x825F9FF4)
	// 825F9F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9F8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9F94: 390B14C8  addi r8, r11, 0x14c8
	ctx.r[8].s64 = ctx.r[11].s64 + 5320;
	// 825F9F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9F9C: 392A83DC  addi r9, r10, -0x7c24
	ctx.r[9].s64 = ctx.r[10].s64 + -31780;
	// 825F9FA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9FA4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F9FA8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F9FAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9FB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9FC4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9FC8: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 825F9FCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9FD0: 386BFB7C  addi r3, r11, -0x484
	ctx.r[3].s64 = ctx.r[11].s64 + -1156;
	// 825F9FD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9FD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9FE0: 4BE6CE41  bl 0x82466e20
	ctx.lr = 0x825F9FE4;
	sub_82466E20(ctx, base);
	// 825F9FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9FF8 size=100
    let mut pc: u32 = 0x825F9FF8;
    'dispatch: loop {
        match pc {
            0x825F9FF8 => {
    //   block [0x825F9FF8..0x825FA05C)
	// 825F9FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA00C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA018: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 825FA01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA02C: 386AFBAC  addi r3, r10, -0x454
	ctx.r[3].s64 = ctx.r[10].s64 + -1108;
	// 825FA030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FA03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA048: 4BE6CDD9  bl 0x82466e20
	ctx.lr = 0x825FA04C;
	sub_82466E20(ctx, base);
	// 825FA04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA060 size=112
    let mut pc: u32 = 0x825FA060;
    'dispatch: loop {
        match pc {
            0x825FA060 => {
    //   block [0x825FA060..0x825FA0D0)
	// 825FA060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA070: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA074: 38AAFBAC  addi r5, r10, -0x454
	ctx.r[5].s64 = ctx.r[10].s64 + -1108;
	// 825FA078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA07C: 390B14F8  addi r8, r11, 0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + 5368;
	// 825FA080: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FA084: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 825FA088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA098: 386AFBDC  addi r3, r10, -0x424
	ctx.r[3].s64 = ctx.r[10].s64 + -1060;
	// 825FA09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA0BC: 4BE6CD65  bl 0x82466e20
	ctx.lr = 0x825FA0C0;
	sub_82466E20(ctx, base);
	// 825FA0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA0D0 size=112
    let mut pc: u32 = 0x825FA0D0;
    'dispatch: loop {
        match pc {
            0x825FA0D0 => {
    //   block [0x825FA0D0..0x825FA140)
	// 825FA0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA0DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA0E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA0E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA0EC: 390B1510  addi r8, r11, 0x1510
	ctx.r[8].s64 = ctx.r[11].s64 + 5392;
	// 825FA0F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA0F4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 825FA0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA108: 386AFC0C  addi r3, r10, -0x3f4
	ctx.r[3].s64 = ctx.r[10].s64 + -1012;
	// 825FA10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA12C: 4BE6CCF5  bl 0x82466e20
	ctx.lr = 0x825FA130;
	sub_82466E20(ctx, base);
	// 825FA130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA140 size=112
    let mut pc: u32 = 0x825FA140;
    'dispatch: loop {
        match pc {
            0x825FA140 => {
    //   block [0x825FA140..0x825FA1B0)
	// 825FA140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA150: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA154: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA15C: 390B1558  addi r8, r11, 0x1558
	ctx.r[8].s64 = ctx.r[11].s64 + 5464;
	// 825FA160: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FA164: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 825FA168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA178: 386AFC3C  addi r3, r10, -0x3c4
	ctx.r[3].s64 = ctx.r[10].s64 + -964;
	// 825FA17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA19C: 4BE6CC85  bl 0x82466e20
	ctx.lr = 0x825FA1A0;
	sub_82466E20(ctx, base);
	// 825FA1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA1B0 size=116
    let mut pc: u32 = 0x825FA1B0;
    'dispatch: loop {
        match pc {
            0x825FA1B0 => {
    //   block [0x825FA1B0..0x825FA224)
	// 825FA1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA1BC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FA1C0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825FA1C4: 390A1600  addi r8, r10, 0x1600
	ctx.r[8].s64 = ctx.r[10].s64 + 5632;
	// 825FA1C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA1CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FA1D0: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FA1D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA1D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA1E4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 825FA1E8: 396B83F0  addi r11, r11, -0x7c10
	ctx.r[11].s64 = ctx.r[11].s64 + -31760;
	// 825FA1EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA1F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA1F4: 386AFC6C  addi r3, r10, -0x394
	ctx.r[3].s64 = ctx.r[10].s64 + -916;
	// 825FA1F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FA1FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA200: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FA204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA210: 4BE6CC11  bl 0x82466e20
	ctx.lr = 0x825FA214;
	sub_82466E20(ctx, base);
	// 825FA214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA228 size=112
    let mut pc: u32 = 0x825FA228;
    'dispatch: loop {
        match pc {
            0x825FA228 => {
    //   block [0x825FA228..0x825FA298)
	// 825FA228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA238: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA23C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA244: 390B16C0  addi r8, r11, 0x16c0
	ctx.r[8].s64 = ctx.r[11].s64 + 5824;
	// 825FA248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FA24C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 825FA250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA260: 386AFC9C  addi r3, r10, -0x364
	ctx.r[3].s64 = ctx.r[10].s64 + -868;
	// 825FA264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA284: 4BE6CB9D  bl 0x82466e20
	ctx.lr = 0x825FA288;
	sub_82466E20(ctx, base);
	// 825FA288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA298 size=100
    let mut pc: u32 = 0x825FA298;
    'dispatch: loop {
        match pc {
            0x825FA298 => {
    //   block [0x825FA298..0x825FA2FC)
	// 825FA298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA2AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA2B8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 825FA2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA2CC: 386AFCCC  addi r3, r10, -0x334
	ctx.r[3].s64 = ctx.r[10].s64 + -820;
	// 825FA2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA2D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA2D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FA2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA2E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA2E8: 4BE6CB39  bl 0x82466e20
	ctx.lr = 0x825FA2EC;
	sub_82466E20(ctx, base);
	// 825FA2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA300 size=108
    let mut pc: u32 = 0x825FA300;
    'dispatch: loop {
        match pc {
            0x825FA300 => {
    //   block [0x825FA300..0x825FA36C)
	// 825FA300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA30C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA314: 38EB16F0  addi r7, r11, 0x16f0
	ctx.r[7].s64 = ctx.r[11].s64 + 5872;
	// 825FA318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA31C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 825FA320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA330: 386AFCFC  addi r3, r10, -0x304
	ctx.r[3].s64 = ctx.r[10].s64 + -772;
	// 825FA334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA358: 4BE6CAC9  bl 0x82466e20
	ctx.lr = 0x825FA35C;
	sub_82466E20(ctx, base);
	// 825FA35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA370 size=112
    let mut pc: u32 = 0x825FA370;
    'dispatch: loop {
        match pc {
            0x825FA370 => {
    //   block [0x825FA370..0x825FA3E0)
	// 825FA370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA37C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA380: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA384: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA38C: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 825FA390: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FA394: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 825FA398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA39C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA3A8: 386AFD2C  addi r3, r10, -0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + -724;
	// 825FA3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA3CC: 4BE6CA55  bl 0x82466e20
	ctx.lr = 0x825FA3D0;
	sub_82466E20(ctx, base);
	// 825FA3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA3E0 size=108
    let mut pc: u32 = 0x825FA3E0;
    'dispatch: loop {
        match pc {
            0x825FA3E0 => {
    //   block [0x825FA3E0..0x825FA44C)
	// 825FA3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA3EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA3F4: 38EB1750  addi r7, r11, 0x1750
	ctx.r[7].s64 = ctx.r[11].s64 + 5968;
	// 825FA3F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA3FC: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 825FA400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA410: 386AFD5C  addi r3, r10, -0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + -676;
	// 825FA414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA438: 4BE6C9E9  bl 0x82466e20
	ctx.lr = 0x825FA43C;
	sub_82466E20(ctx, base);
	// 825FA43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA450 size=112
    let mut pc: u32 = 0x825FA450;
    'dispatch: loop {
        match pc {
            0x825FA450 => {
    //   block [0x825FA450..0x825FA4C0)
	// 825FA450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA460: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA464: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA46C: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 825FA470: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA474: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 825FA478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA47C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA488: 386AFD8C  addi r3, r10, -0x274
	ctx.r[3].s64 = ctx.r[10].s64 + -628;
	// 825FA48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA4AC: 4BE6C975  bl 0x82466e20
	ctx.lr = 0x825FA4B0;
	sub_82466E20(ctx, base);
	// 825FA4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA4C0 size=108
    let mut pc: u32 = 0x825FA4C0;
    'dispatch: loop {
        match pc {
            0x825FA4C0 => {
    //   block [0x825FA4C0..0x825FA52C)
	// 825FA4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA4CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA4D4: 38EB17C8  addi r7, r11, 0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6088;
	// 825FA4D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA4DC: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 825FA4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA4F0: 386AFDBC  addi r3, r10, -0x244
	ctx.r[3].s64 = ctx.r[10].s64 + -580;
	// 825FA4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA518: 4BE6C909  bl 0x82466e20
	ctx.lr = 0x825FA51C;
	sub_82466E20(ctx, base);
	// 825FA51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA530 size=112
    let mut pc: u32 = 0x825FA530;
    'dispatch: loop {
        match pc {
            0x825FA530 => {
    //   block [0x825FA530..0x825FA5A0)
	// 825FA530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA540: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA544: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA54C: 390B17F8  addi r8, r11, 0x17f8
	ctx.r[8].s64 = ctx.r[11].s64 + 6136;
	// 825FA550: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA554: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 825FA558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA568: 386AFDEC  addi r3, r10, -0x214
	ctx.r[3].s64 = ctx.r[10].s64 + -532;
	// 825FA56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA58C: 4BE6C895  bl 0x82466e20
	ctx.lr = 0x825FA590;
	sub_82466E20(ctx, base);
	// 825FA590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA5A0 size=108
    let mut pc: u32 = 0x825FA5A0;
    'dispatch: loop {
        match pc {
            0x825FA5A0 => {
    //   block [0x825FA5A0..0x825FA60C)
	// 825FA5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA5AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA5B4: 38EB1840  addi r7, r11, 0x1840
	ctx.r[7].s64 = ctx.r[11].s64 + 6208;
	// 825FA5B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA5BC: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 825FA5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA5C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA5C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA5D0: 386AFE1C  addi r3, r10, -0x1e4
	ctx.r[3].s64 = ctx.r[10].s64 + -484;
	// 825FA5D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA5DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA5F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA5F8: 4BE6C829  bl 0x82466e20
	ctx.lr = 0x825FA5FC;
	sub_82466E20(ctx, base);
	// 825FA5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA610 size=112
    let mut pc: u32 = 0x825FA610;
    'dispatch: loop {
        match pc {
            0x825FA610 => {
    //   block [0x825FA610..0x825FA680)
	// 825FA610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA61C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA624: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA62C: 390B1870  addi r8, r11, 0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + 6256;
	// 825FA630: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA634: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 825FA638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA63C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA648: 386AFE4C  addi r3, r10, -0x1b4
	ctx.r[3].s64 = ctx.r[10].s64 + -436;
	// 825FA64C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA66C: 4BE6C7B5  bl 0x82466e20
	ctx.lr = 0x825FA670;
	sub_82466E20(ctx, base);
	// 825FA670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA680 size=108
    let mut pc: u32 = 0x825FA680;
    'dispatch: loop {
        match pc {
            0x825FA680 => {
    //   block [0x825FA680..0x825FA6EC)
	// 825FA680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA68C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA694: 38EB18C0  addi r7, r11, 0x18c0
	ctx.r[7].s64 = ctx.r[11].s64 + 6336;
	// 825FA698: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FA69C: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 825FA6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA6B0: 386AFE7C  addi r3, r10, -0x184
	ctx.r[3].s64 = ctx.r[10].s64 + -388;
	// 825FA6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA6D8: 4BE6C749  bl 0x82466e20
	ctx.lr = 0x825FA6DC;
	sub_82466E20(ctx, base);
	// 825FA6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA6F0 size=24
    let mut pc: u32 = 0x825FA6F0;
    'dispatch: loop {
        match pc {
            0x825FA6F0 => {
    //   block [0x825FA6F0..0x825FA708)
	// 825FA6F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA6F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FA6F8: 394A9300  addi r10, r10, -0x6d00
	ctx.r[10].s64 = ctx.r[10].s64 + -27904;
	// 825FA6FC: 816B18BC  lwz r11, 0x18bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6332 as u32) ) } as u64;
	// 825FA700: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 825FA704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA708 size=112
    let mut pc: u32 = 0x825FA708;
    'dispatch: loop {
        match pc {
            0x825FA708 => {
    //   block [0x825FA708..0x825FA778)
	// 825FA708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA714: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA718: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA71C: 392A84D8  addi r9, r10, -0x7b28
	ctx.r[9].s64 = ctx.r[10].s64 + -31528;
	// 825FA720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA724: 390B9300  addi r8, r11, -0x6d00
	ctx.r[8].s64 = ctx.r[11].s64 + -27904;
	// 825FA728: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825FA72C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 825FA730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA740: 386AFEAC  addi r3, r10, -0x154
	ctx.r[3].s64 = ctx.r[10].s64 + -340;
	// 825FA744: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA748: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FA74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA764: 4BE6C6BD  bl 0x82466e20
	ctx.lr = 0x825FA768;
	sub_82466E20(ctx, base);
	// 825FA768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA778 size=108
    let mut pc: u32 = 0x825FA778;
    'dispatch: loop {
        match pc {
            0x825FA778 => {
    //   block [0x825FA778..0x825FA7E4)
	// 825FA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA784: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA78C: 38EB1928  addi r7, r11, 0x1928
	ctx.r[7].s64 = ctx.r[11].s64 + 6440;
	// 825FA790: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FA794: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 825FA798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA7A8: 386AFEDC  addi r3, r10, -0x124
	ctx.r[3].s64 = ctx.r[10].s64 + -292;
	// 825FA7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA7D0: 4BE6C651  bl 0x82466e20
	ctx.lr = 0x825FA7D4;
	sub_82466E20(ctx, base);
	// 825FA7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA7E8 size=108
    let mut pc: u32 = 0x825FA7E8;
    'dispatch: loop {
        match pc {
            0x825FA7E8 => {
    //   block [0x825FA7E8..0x825FA854)
	// 825FA7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA7F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA7FC: 38EB19A0  addi r7, r11, 0x19a0
	ctx.r[7].s64 = ctx.r[11].s64 + 6560;
	// 825FA800: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FA804: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 825FA808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA818: 386AFF0C  addi r3, r10, -0xf4
	ctx.r[3].s64 = ctx.r[10].s64 + -244;
	// 825FA81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA840: 4BE6C5E1  bl 0x82466e20
	ctx.lr = 0x825FA844;
	sub_82466E20(ctx, base);
	// 825FA844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA858 size=108
    let mut pc: u32 = 0x825FA858;
    'dispatch: loop {
        match pc {
            0x825FA858 => {
    //   block [0x825FA858..0x825FA8C4)
	// 825FA858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA864: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA86C: 38EB1A00  addi r7, r11, 0x1a00
	ctx.r[7].s64 = ctx.r[11].s64 + 6656;
	// 825FA870: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FA874: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 825FA878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA888: 386AFF3C  addi r3, r10, -0xc4
	ctx.r[3].s64 = ctx.r[10].s64 + -196;
	// 825FA88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA8B0: 4BE6C571  bl 0x82466e20
	ctx.lr = 0x825FA8B4;
	sub_82466E20(ctx, base);
	// 825FA8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA8C8 size=24
    let mut pc: u32 = 0x825FA8C8;
    'dispatch: loop {
        match pc {
            0x825FA8C8 => {
    //   block [0x825FA8C8..0x825FA8E0)
	// 825FA8C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA8CC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FA8D0: 394A9408  addi r10, r10, -0x6bf8
	ctx.r[10].s64 = ctx.r[10].s64 + -27640;
	// 825FA8D4: 816B18B8  lwz r11, 0x18b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6328 as u32) ) } as u64;
	// 825FA8D8: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 825FA8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA8E0 size=116
    let mut pc: u32 = 0x825FA8E0;
    'dispatch: loop {
        match pc {
            0x825FA8E0 => {
    //   block [0x825FA8E0..0x825FA954)
	// 825FA8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA8EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FA8F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA8F4: 392B8438  addi r9, r11, -0x7bc8
	ctx.r[9].s64 = ctx.r[11].s64 + -31688;
	// 825FA8F8: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FA8FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA900: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 825FA904: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 825FA908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA90C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 825FA910: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA914: 396B9408  addi r11, r11, -0x6bf8
	ctx.r[11].s64 = ctx.r[11].s64 + -27640;
	// 825FA918: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FA91C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA920: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FA924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA928: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 825FA92C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FA930: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FA934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA938: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FA93C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA940: 4BE6C4E1  bl 0x82466e20
	ctx.lr = 0x825FA944;
	sub_82466E20(ctx, base);
	// 825FA944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA958 size=36
    let mut pc: u32 = 0x825FA958;
    'dispatch: loop {
        match pc {
            0x825FA958 => {
    //   block [0x825FA958..0x825FA97C)
	// 825FA958: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA95C: 814B1924  lwz r10, 0x1924(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6436 as u32) ) } as u64;
	// 825FA960: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA964: 396B96D8  addi r11, r11, -0x6928
	ctx.r[11].s64 = ctx.r[11].s64 + -26920;
	// 825FA968: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FA96C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FA970: 814A1AA8  lwz r10, 0x1aa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6824 as u32) ) } as u64;
	// 825FA974: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FA978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA980 size=116
    let mut pc: u32 = 0x825FA980;
    'dispatch: loop {
        match pc {
            0x825FA980 => {
    //   block [0x825FA980..0x825FA9F4)
	// 825FA980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA98C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA994: 390B96D8  addi r8, r11, -0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + -26920;
	// 825FA998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA99C: 392A85C0  addi r9, r10, -0x7a40
	ctx.r[9].s64 = ctx.r[10].s64 + -31296;
	// 825FA9A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA9A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FA9A8: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FA9AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA9B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA9C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FA9C8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 825FA9CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA9D0: 386BFF9C  addi r3, r11, -0x64
	ctx.r[3].s64 = ctx.r[11].s64 + -100;
	// 825FA9D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FA9D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA9E0: 4BE6C441  bl 0x82466e20
	ctx.lr = 0x825FA9E4;
	sub_82466E20(ctx, base);
	// 825FA9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA9F8 size=24
    let mut pc: u32 = 0x825FA9F8;
    'dispatch: loop {
        match pc {
            0x825FA9F8 => {
    //   block [0x825FA9F8..0x825FAA10)
	// 825FA9F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA9FC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FAA00: 394A9708  addi r10, r10, -0x68f8
	ctx.r[10].s64 = ctx.r[10].s64 + -26872;
	// 825FAA04: 816B1AB0  lwz r11, 0x1ab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6832 as u32) ) } as u64;
	// 825FAA08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FAA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAA10 size=116
    let mut pc: u32 = 0x825FAA10;
    'dispatch: loop {
        match pc {
            0x825FAA10 => {
    //   block [0x825FAA10..0x825FAA84)
	// 825FAA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAA1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAA24: 390B9708  addi r8, r11, -0x68f8
	ctx.r[8].s64 = ctx.r[11].s64 + -26872;
	// 825FAA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAA2C: 392A8618  addi r9, r10, -0x79e8
	ctx.r[9].s64 = ctx.r[10].s64 + -31208;
	// 825FAA30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAA34: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825FAA38: 38AAFF9C  addi r5, r10, -0x64
	ctx.r[5].s64 = ctx.r[10].s64 + -100;
	// 825FAA3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAA44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAA54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FAA58: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 825FAA5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAA60: 386BFFCC  addi r3, r11, -0x34
	ctx.r[3].s64 = ctx.r[11].s64 + -52;
	// 825FAA64: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FAA68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAA70: 4BE6C3B1  bl 0x82466e20
	ctx.lr = 0x825FAA74;
	sub_82466E20(ctx, base);
	// 825FAA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAA88 size=112
    let mut pc: u32 = 0x825FAA88;
    'dispatch: loop {
        match pc {
            0x825FAA88 => {
    //   block [0x825FAA88..0x825FAAF8)
	// 825FAA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAA94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAA9C: 38AAFF9C  addi r5, r10, -0x64
	ctx.r[5].s64 = ctx.r[10].s64 + -100;
	// 825FAAA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAAA4: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 825FAAA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825FAAAC: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 825FAAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAAC0: 386AFFFC  addi r3, r10, -4
	ctx.r[3].s64 = ctx.r[10].s64 + -4;
	// 825FAAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAAE4: 4BE6C33D  bl 0x82466e20
	ctx.lr = 0x825FAAE8;
	sub_82466E20(ctx, base);
	// 825FAAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAAF8 size=112
    let mut pc: u32 = 0x825FAAF8;
    'dispatch: loop {
        match pc {
            0x825FAAF8 => {
    //   block [0x825FAAF8..0x825FAB68)
	// 825FAAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAB04: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FAB08: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FAB0C: 38EA1BA8  addi r7, r10, 0x1ba8
	ctx.r[7].s64 = ctx.r[10].s64 + 7080;
	// 825FAB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAB14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FAB18: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 825FAB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAB20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAB24: 396B8640  addi r11, r11, -0x79c0
	ctx.r[11].s64 = ctx.r[11].s64 + -31168;
	// 825FAB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAB2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAB34: 386A002C  addi r3, r10, 0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + 44;
	// 825FAB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAB3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FAB40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAB44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FAB48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAB4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAB50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAB54: 4BE6C2CD  bl 0x82466e20
	ctx.lr = 0x825FAB58;
	sub_82466E20(ctx, base);
	// 825FAB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAB68 size=112
    let mut pc: u32 = 0x825FAB68;
    'dispatch: loop {
        match pc {
            0x825FAB68 => {
    //   block [0x825FAB68..0x825FABD8)
	// 825FAB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAB74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAB7C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAB84: 390B1C38  addi r8, r11, 0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + 7224;
	// 825FAB88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FAB8C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 825FAB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAB94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FABA0: 386A005C  addi r3, r10, 0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + 92;
	// 825FABA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FABAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FABBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FABC4: 4BE6C25D  bl 0x82466e20
	ctx.lr = 0x825FABC8;
	sub_82466E20(ctx, base);
	// 825FABC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FABCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FABD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FABD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FABD8 size=24
    let mut pc: u32 = 0x825FABD8;
    'dispatch: loop {
        match pc {
            0x825FABD8 => {
    //   block [0x825FABD8..0x825FABF0)
	// 825FABD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FABDC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FABE0: 394A9828  addi r10, r10, -0x67d8
	ctx.r[10].s64 = ctx.r[10].s64 + -26584;
	// 825FABE4: 816B1AB4  lwz r11, 0x1ab4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6836 as u32) ) } as u64;
	// 825FABE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FABEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FABF0 size=112
    let mut pc: u32 = 0x825FABF0;
    'dispatch: loop {
        match pc {
            0x825FABF0 => {
    //   block [0x825FABF0..0x825FAC60)
	// 825FABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FABF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FABFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAC00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAC04: 392A868C  addi r9, r10, -0x7974
	ctx.r[9].s64 = ctx.r[10].s64 + -31092;
	// 825FAC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAC0C: 390B9828  addi r8, r11, -0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + -26584;
	// 825FAC10: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FAC14: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 825FAC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAC1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAC28: 386A008C  addi r3, r10, 0x8c
	ctx.r[3].s64 = ctx.r[10].s64 + 140;
	// 825FAC2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAC30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FAC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAC44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAC4C: 4BE6C1D5  bl 0x82466e20
	ctx.lr = 0x825FAC50;
	sub_82466E20(ctx, base);
	// 825FAC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAC60 size=108
    let mut pc: u32 = 0x825FAC60;
    'dispatch: loop {
        match pc {
            0x825FAC60 => {
    //   block [0x825FAC60..0x825FACCC)
	// 825FAC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAC6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAC74: 38EB1C58  addi r7, r11, 0x1c58
	ctx.r[7].s64 = ctx.r[11].s64 + 7256;
	// 825FAC78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FAC7C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 825FAC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAC84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAC90: 386A00BC  addi r3, r10, 0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + 188;
	// 825FAC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FACA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FACA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FACA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FACAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FACB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FACB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FACB8: 4BE6C169  bl 0x82466e20
	ctx.lr = 0x825FACBC;
	sub_82466E20(ctx, base);
	// 825FACBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FACC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FACC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FACC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FACD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FACD0 size=108
    let mut pc: u32 = 0x825FACD0;
    'dispatch: loop {
        match pc {
            0x825FACD0 => {
    //   block [0x825FACD0..0x825FAD3C)
	// 825FACD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FACD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FACD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FACDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FACE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FACE4: 38EB1CB8  addi r7, r11, 0x1cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 7352;
	// 825FACE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FACEC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 825FACF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FACF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FACF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FACFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAD00: 386A00EC  addi r3, r10, 0xec
	ctx.r[3].s64 = ctx.r[10].s64 + 236;
	// 825FAD04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAD08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAD10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAD18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAD20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAD28: 4BE6C0F9  bl 0x82466e20
	ctx.lr = 0x825FAD2C;
	sub_82466E20(ctx, base);
	// 825FAD2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAD38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAD40 size=116
    let mut pc: u32 = 0x825FAD40;
    'dispatch: loop {
        match pc {
            0x825FAD40 => {
    //   block [0x825FAD40..0x825FADB4)
	// 825FAD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAD4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAD50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAD54: 390B1CE8  addi r8, r11, 0x1ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 7400;
	// 825FAD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAD5C: 392A86B8  addi r9, r10, -0x7948
	ctx.r[9].s64 = ctx.r[10].s64 + -31048;
	// 825FAD60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAD64: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FAD68: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAD6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAD74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAD84: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FAD88: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 825FAD8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAD90: 386B011C  addi r3, r11, 0x11c
	ctx.r[3].s64 = ctx.r[11].s64 + 284;
	// 825FAD94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FAD98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FADA0: 4BE6C081  bl 0x82466e20
	ctx.lr = 0x825FADA4;
	sub_82466E20(ctx, base);
	// 825FADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FADB8 size=108
    let mut pc: u32 = 0x825FADB8;
    'dispatch: loop {
        match pc {
            0x825FADB8 => {
    //   block [0x825FADB8..0x825FAE24)
	// 825FADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FADC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FADC4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FADC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FADCC: 38EB1D00  addi r7, r11, 0x1d00
	ctx.r[7].s64 = ctx.r[11].s64 + 7424;
	// 825FADD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FADD4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 825FADD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FADDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FADE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FADE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FADE8: 386A014C  addi r3, r10, 0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + 332;
	// 825FADEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FADF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FADF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FADF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FADFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAE10: 4BE6C011  bl 0x82466e20
	ctx.lr = 0x825FAE14;
	sub_82466E20(ctx, base);
	// 825FAE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAE28 size=112
    let mut pc: u32 = 0x825FAE28;
    'dispatch: loop {
        match pc {
            0x825FAE28 => {
    //   block [0x825FAE28..0x825FAE98)
	// 825FAE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAE38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAE3C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAE44: 390B1D18  addi r8, r11, 0x1d18
	ctx.r[8].s64 = ctx.r[11].s64 + 7448;
	// 825FAE48: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825FAE4C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 825FAE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAE60: 386A017C  addi r3, r10, 0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + 380;
	// 825FAE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAE84: 4BE6BF9D  bl 0x82466e20
	ctx.lr = 0x825FAE88;
	sub_82466E20(ctx, base);
	// 825FAE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAE98 size=108
    let mut pc: u32 = 0x825FAE98;
    'dispatch: loop {
        match pc {
            0x825FAE98 => {
    //   block [0x825FAE98..0x825FAF04)
	// 825FAE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAEA4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAEAC: 38EB1DF0  addi r7, r11, 0x1df0
	ctx.r[7].s64 = ctx.r[11].s64 + 7664;
	// 825FAEB0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FAEB4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 825FAEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAEC8: 386A01AC  addi r3, r10, 0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + 428;
	// 825FAECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAEF0: 4BE6BF31  bl 0x82466e20
	ctx.lr = 0x825FAEF4;
	sub_82466E20(ctx, base);
	// 825FAEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAF08 size=108
    let mut pc: u32 = 0x825FAF08;
    'dispatch: loop {
        match pc {
            0x825FAF08 => {
    //   block [0x825FAF08..0x825FAF74)
	// 825FAF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAF14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAF1C: 38EB1E68  addi r7, r11, 0x1e68
	ctx.r[7].s64 = ctx.r[11].s64 + 7784;
	// 825FAF20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FAF24: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 825FAF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAF38: 386A01DC  addi r3, r10, 0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + 476;
	// 825FAF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAF60: 4BE6BEC1  bl 0x82466e20
	ctx.lr = 0x825FAF64;
	sub_82466E20(ctx, base);
	// 825FAF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAF78 size=112
    let mut pc: u32 = 0x825FAF78;
    'dispatch: loop {
        match pc {
            0x825FAF78 => {
    //   block [0x825FAF78..0x825FAFE8)
	// 825FAF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAF88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAF8C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAF94: 390B1EB0  addi r8, r11, 0x1eb0
	ctx.r[8].s64 = ctx.r[11].s64 + 7856;
	// 825FAF98: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 825FAF9C: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 825FAFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAFB0: 386A020C  addi r3, r10, 0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + 524;
	// 825FAFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAFD4: 4BE6BE4D  bl 0x82466e20
	ctx.lr = 0x825FAFD8;
	sub_82466E20(ctx, base);
	// 825FAFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FAFE8 size=76
    let mut pc: u32 = 0x825FAFE8;
    'dispatch: loop {
        match pc {
            0x825FAFE8 => {
    //   block [0x825FAFE8..0x825FB034)
	// 825FAFE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAFEC: 814B1C54  lwz r10, 0x1c54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7252 as u32) ) } as u64;
	// 825FAFF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAFF4: 396B9858  addi r11, r11, -0x67a8
	ctx.r[11].s64 = ctx.r[11].s64 + -26536;
	// 825FAFF8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 825FAFFC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825FB000: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FB004: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825FB008: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 825FB00C: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825FB010: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB014: 814A2090  lwz r10, 0x2090(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8336 as u32) ) } as u64;
	// 825FB018: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 825FB01C: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 825FB020: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 825FB024: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 825FB028: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 825FB02C: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 825FB030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB038 size=108
    let mut pc: u32 = 0x825FB038;
    'dispatch: loop {
        match pc {
            0x825FB038 => {
    //   block [0x825FB038..0x825FB0A4)
	// 825FB038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB044: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB04C: 38EB9858  addi r7, r11, -0x67a8
	ctx.r[7].s64 = ctx.r[11].s64 + -26536;
	// 825FB050: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 825FB054: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 825FB058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB05C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB068: 386A023C  addi r3, r10, 0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + 572;
	// 825FB06C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB090: 4BE6BD91  bl 0x82466e20
	ctx.lr = 0x825FB094;
	sub_82466E20(ctx, base);
	// 825FB094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB0A8 size=76
    let mut pc: u32 = 0x825FB0A8;
    'dispatch: loop {
        match pc {
            0x825FB0A8 => {
    //   block [0x825FB0A8..0x825FB0F4)
	// 825FB0A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB0AC: 814B1C54  lwz r10, 0x1c54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7252 as u32) ) } as u64;
	// 825FB0B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB0B4: 396B9AC8  addi r11, r11, -0x6538
	ctx.r[11].s64 = ctx.r[11].s64 + -25912;
	// 825FB0B8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 825FB0BC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825FB0C0: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FB0C4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825FB0C8: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 825FB0CC: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825FB0D0: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB0D4: 814A2090  lwz r10, 0x2090(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8336 as u32) ) } as u64;
	// 825FB0D8: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 825FB0DC: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 825FB0E0: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 825FB0E4: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 825FB0E8: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 825FB0EC: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 825FB0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB0F8 size=116
    let mut pc: u32 = 0x825FB0F8;
    'dispatch: loop {
        match pc {
            0x825FB0F8 => {
    //   block [0x825FB0F8..0x825FB16C)
	// 825FB0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB104: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB10C: 390B9AC8  addi r8, r11, -0x6538
	ctx.r[8].s64 = ctx.r[11].s64 + -25912;
	// 825FB110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB114: 392A8704  addi r9, r10, -0x78fc
	ctx.r[9].s64 = ctx.r[10].s64 + -30972;
	// 825FB118: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB11C: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 825FB120: 38AAF5DC  addi r5, r10, -0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + -2596;
	// 825FB124: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB12C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB13C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB140: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 825FB144: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB148: 386B026C  addi r3, r11, 0x26c
	ctx.r[3].s64 = ctx.r[11].s64 + 620;
	// 825FB14C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FB150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB158: 4BE6BCC9  bl 0x82466e20
	ctx.lr = 0x825FB15C;
	sub_82466E20(ctx, base);
	// 825FB15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB170 size=112
    let mut pc: u32 = 0x825FB170;
    'dispatch: loop {
        match pc {
            0x825FB170 => {
    //   block [0x825FB170..0x825FB1E0)
	// 825FB170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB180: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB184: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FB188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB18C: 390B2098  addi r8, r11, 0x2098
	ctx.r[8].s64 = ctx.r[11].s64 + 8344;
	// 825FB190: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FB194: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 825FB198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB19C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB1A8: 386A029C  addi r3, r10, 0x29c
	ctx.r[3].s64 = ctx.r[10].s64 + 668;
	// 825FB1AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB1B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB1CC: 4BE6BC55  bl 0x82466e20
	ctx.lr = 0x825FB1D0;
	sub_82466E20(ctx, base);
	// 825FB1D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB1E0 size=108
    let mut pc: u32 = 0x825FB1E0;
    'dispatch: loop {
        match pc {
            0x825FB1E0 => {
    //   block [0x825FB1E0..0x825FB24C)
	// 825FB1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB1EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB1F4: 38EB20E0  addi r7, r11, 0x20e0
	ctx.r[7].s64 = ctx.r[11].s64 + 8416;
	// 825FB1F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FB1FC: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 825FB200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB210: 386A02CC  addi r3, r10, 0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + 716;
	// 825FB214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB238: 4BE6BBE9  bl 0x82466e20
	ctx.lr = 0x825FB23C;
	sub_82466E20(ctx, base);
	// 825FB23C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB250 size=108
    let mut pc: u32 = 0x825FB250;
    'dispatch: loop {
        match pc {
            0x825FB250 => {
    //   block [0x825FB250..0x825FB2BC)
	// 825FB250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB25C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB264: 38EB2128  addi r7, r11, 0x2128
	ctx.r[7].s64 = ctx.r[11].s64 + 8488;
	// 825FB268: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FB26C: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 825FB270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB280: 386A02FC  addi r3, r10, 0x2fc
	ctx.r[3].s64 = ctx.r[10].s64 + 764;
	// 825FB284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB2A8: 4BE6BB79  bl 0x82466e20
	ctx.lr = 0x825FB2AC;
	sub_82466E20(ctx, base);
	// 825FB2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB2C0 size=116
    let mut pc: u32 = 0x825FB2C0;
    'dispatch: loop {
        match pc {
            0x825FB2C0 => {
    //   block [0x825FB2C0..0x825FB334)
	// 825FB2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB2CC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB2D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FB2D4: 390A2170  addi r8, r10, 0x2170
	ctx.r[8].s64 = ctx.r[10].s64 + 8560;
	// 825FB2D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB2DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB2E0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FB2E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB2E8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB2F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB2F4: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 825FB2F8: 396B872C  addi r11, r11, -0x78d4
	ctx.r[11].s64 = ctx.r[11].s64 + -30932;
	// 825FB2FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB304: 386A032C  addi r3, r10, 0x32c
	ctx.r[3].s64 = ctx.r[10].s64 + 812;
	// 825FB308: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB310: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB320: 4BE6BB01  bl 0x82466e20
	ctx.lr = 0x825FB324;
	sub_82466E20(ctx, base);
	// 825FB324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB338 size=116
    let mut pc: u32 = 0x825FB338;
    'dispatch: loop {
        match pc {
            0x825FB338 => {
    //   block [0x825FB338..0x825FB3AC)
	// 825FB338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB344: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB348: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FB34C: 390A2218  addi r8, r10, 0x2218
	ctx.r[8].s64 = ctx.r[10].s64 + 8728;
	// 825FB350: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB354: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB358: 38AA032C  addi r5, r10, 0x32c
	ctx.r[5].s64 = ctx.r[10].s64 + 812;
	// 825FB35C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB360: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB36C: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 825FB370: 396B8750  addi r11, r11, -0x78b0
	ctx.r[11].s64 = ctx.r[11].s64 + -30896;
	// 825FB374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB378: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB37C: 386A035C  addi r3, r10, 0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + 860;
	// 825FB380: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB388: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB398: 4BE6BA89  bl 0x82466e20
	ctx.lr = 0x825FB39C;
	sub_82466E20(ctx, base);
	// 825FB39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB3B0 size=36
    let mut pc: u32 = 0x825FB3B0;
    'dispatch: loop {
        match pc {
            0x825FB3B0 => {
    //   block [0x825FB3B0..0x825FB3D4)
	// 825FB3B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB3B4: 814B2094  lwz r10, 0x2094(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8340 as u32) ) } as u64;
	// 825FB3B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB3BC: 396B9E88  addi r11, r11, -0x6178
	ctx.r[11].s64 = ctx.r[11].s64 + -24952;
	// 825FB3C0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FB3C4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB3C8: 814A2260  lwz r10, 0x2260(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8800 as u32) ) } as u64;
	// 825FB3CC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FB3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB3D8 size=116
    let mut pc: u32 = 0x825FB3D8;
    'dispatch: loop {
        match pc {
            0x825FB3D8 => {
    //   block [0x825FB3D8..0x825FB44C)
	// 825FB3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB3E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB3EC: 390B9E88  addi r8, r11, -0x6178
	ctx.r[8].s64 = ctx.r[11].s64 + -24952;
	// 825FB3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB3F4: 392A8790  addi r9, r10, -0x7870
	ctx.r[9].s64 = ctx.r[10].s64 + -30832;
	// 825FB3F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB3FC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825FB400: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB404: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB40C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB41C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB420: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 825FB424: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB428: 386B038C  addi r3, r11, 0x38c
	ctx.r[3].s64 = ctx.r[11].s64 + 908;
	// 825FB42C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FB430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB438: 4BE6B9E9  bl 0x82466e20
	ctx.lr = 0x825FB43C;
	sub_82466E20(ctx, base);
	// 825FB43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB450 size=112
    let mut pc: u32 = 0x825FB450;
    'dispatch: loop {
        match pc {
            0x825FB450 => {
    //   block [0x825FB450..0x825FB4C0)
	// 825FB450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB460: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB464: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB46C: 390B2268  addi r8, r11, 0x2268
	ctx.r[8].s64 = ctx.r[11].s64 + 8808;
	// 825FB470: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825FB474: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 825FB478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB47C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB488: 386A03BC  addi r3, r10, 0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + 956;
	// 825FB48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB4AC: 4BE6B975  bl 0x82466e20
	ctx.lr = 0x825FB4B0;
	sub_82466E20(ctx, base);
	// 825FB4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB4C0 size=108
    let mut pc: u32 = 0x825FB4C0;
    'dispatch: loop {
        match pc {
            0x825FB4C0 => {
    //   block [0x825FB4C0..0x825FB52C)
	// 825FB4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB4CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB4D4: 38EB2328  addi r7, r11, 0x2328
	ctx.r[7].s64 = ctx.r[11].s64 + 9000;
	// 825FB4D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FB4DC: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 825FB4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB4F0: 386A03EC  addi r3, r10, 0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1004;
	// 825FB4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB518: 4BE6B909  bl 0x82466e20
	ctx.lr = 0x825FB51C;
	sub_82466E20(ctx, base);
	// 825FB51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB530 size=112
    let mut pc: u32 = 0x825FB530;
    'dispatch: loop {
        match pc {
            0x825FB530 => {
    //   block [0x825FB530..0x825FB5A0)
	// 825FB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB540: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB544: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB54C: 390B2358  addi r8, r11, 0x2358
	ctx.r[8].s64 = ctx.r[11].s64 + 9048;
	// 825FB550: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825FB554: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 825FB558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB568: 386A041C  addi r3, r10, 0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + 1052;
	// 825FB56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB58C: 4BE6B895  bl 0x82466e20
	ctx.lr = 0x825FB590;
	sub_82466E20(ctx, base);
	// 825FB590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB5A0 size=112
    let mut pc: u32 = 0x825FB5A0;
    'dispatch: loop {
        match pc {
            0x825FB5A0 => {
    //   block [0x825FB5A0..0x825FB610)
	// 825FB5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB5AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB5B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB5B4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB5BC: 390B2460  addi r8, r11, 0x2460
	ctx.r[8].s64 = ctx.r[11].s64 + 9312;
	// 825FB5C0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 825FB5C4: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 825FB5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB5CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB5D8: 386A044C  addi r3, r10, 0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + 1100;
	// 825FB5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB5FC: 4BE6B825  bl 0x82466e20
	ctx.lr = 0x825FB600;
	sub_82466E20(ctx, base);
	// 825FB600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB610 size=116
    let mut pc: u32 = 0x825FB610;
    'dispatch: loop {
        match pc {
            0x825FB610 => {
    //   block [0x825FB610..0x825FB684)
	// 825FB610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB61C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB624: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 825FB628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB62C: 392A87C8  addi r9, r10, -0x7838
	ctx.r[9].s64 = ctx.r[10].s64 + -30776;
	// 825FB630: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB634: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 825FB638: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB63C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB644: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB654: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB658: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 825FB65C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB660: 386B047C  addi r3, r11, 0x47c
	ctx.r[3].s64 = ctx.r[11].s64 + 1148;
	// 825FB664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB670: 4BE6B7B1  bl 0x82466e20
	ctx.lr = 0x825FB674;
	sub_82466E20(ctx, base);
	// 825FB674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB688 size=112
    let mut pc: u32 = 0x825FB688;
    'dispatch: loop {
        match pc {
            0x825FB688 => {
    //   block [0x825FB688..0x825FB6F8)
	// 825FB688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB698: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB69C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB6A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB6A4: 390B2850  addi r8, r11, 0x2850
	ctx.r[8].s64 = ctx.r[11].s64 + 10320;
	// 825FB6A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FB6AC: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 825FB6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB6C0: 386A04AC  addi r3, r10, 0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1196;
	// 825FB6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB6E4: 4BE6B73D  bl 0x82466e20
	ctx.lr = 0x825FB6E8;
	sub_82466E20(ctx, base);
	// 825FB6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB6F8 size=116
    let mut pc: u32 = 0x825FB6F8;
    'dispatch: loop {
        match pc {
            0x825FB6F8 => {
    //   block [0x825FB6F8..0x825FB76C)
	// 825FB6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB704: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB708: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FB70C: 390A2898  addi r8, r10, 0x2898
	ctx.r[8].s64 = ctx.r[10].s64 + 10392;
	// 825FB710: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB714: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB718: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB71C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB720: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB72C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825FB730: 396B87DC  addi r11, r11, -0x7824
	ctx.r[11].s64 = ctx.r[11].s64 + -30756;
	// 825FB734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB73C: 386A04DC  addi r3, r10, 0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + 1244;
	// 825FB740: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB748: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB758: 4BE6B6C9  bl 0x82466e20
	ctx.lr = 0x825FB75C;
	sub_82466E20(ctx, base);
	// 825FB75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB770 size=112
    let mut pc: u32 = 0x825FB770;
    'dispatch: loop {
        match pc {
            0x825FB770 => {
    //   block [0x825FB770..0x825FB7E0)
	// 825FB770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB77C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB780: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB784: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB78C: 390B28E0  addi r8, r11, 0x28e0
	ctx.r[8].s64 = ctx.r[11].s64 + 10464;
	// 825FB790: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FB794: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 825FB798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB7A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB7A8: 386A050C  addi r3, r10, 0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + 1292;
	// 825FB7AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB7CC: 4BE6B655  bl 0x82466e20
	ctx.lr = 0x825FB7D0;
	sub_82466E20(ctx, base);
	// 825FB7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB7E0 size=112
    let mut pc: u32 = 0x825FB7E0;
    'dispatch: loop {
        match pc {
            0x825FB7E0 => {
    //   block [0x825FB7E0..0x825FB850)
	// 825FB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB7EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB7F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB7F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB7FC: 390B2970  addi r8, r11, 0x2970
	ctx.r[8].s64 = ctx.r[11].s64 + 10608;
	// 825FB800: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FB804: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 825FB808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB818: 386A053C  addi r3, r10, 0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + 1340;
	// 825FB81C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB83C: 4BE6B5E5  bl 0x82466e20
	ctx.lr = 0x825FB840;
	sub_82466E20(ctx, base);
	// 825FB840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB850 size=24
    let mut pc: u32 = 0x825FB850;
    'dispatch: loop {
        match pc {
            0x825FB850 => {
    //   block [0x825FB850..0x825FB868)
	// 825FB850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB854: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FB858: 394A9F90  addi r10, r10, -0x6070
	ctx.r[10].s64 = ctx.r[10].s64 + -24688;
	// 825FB85C: 816B29E8  lwz r11, 0x29e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10728 as u32) ) } as u64;
	// 825FB860: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825FB864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB868 size=116
    let mut pc: u32 = 0x825FB868;
    'dispatch: loop {
        match pc {
            0x825FB868 => {
    //   block [0x825FB868..0x825FB8DC)
	// 825FB868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB878: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB87C: 392B8808  addi r9, r11, -0x77f8
	ctx.r[9].s64 = ctx.r[11].s64 + -30712;
	// 825FB880: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB884: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB888: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FB88C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825FB890: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB894: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 825FB898: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB89C: 396B9F90  addi r11, r11, -0x6070
	ctx.r[11].s64 = ctx.r[11].s64 + -24688;
	// 825FB8A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FB8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB8A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FB8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB8B0: 386A056C  addi r3, r10, 0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + 1388;
	// 825FB8B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB8B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FB8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB8C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FB8C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FB8C8: 4BE6B559  bl 0x82466e20
	ctx.lr = 0x825FB8CC;
	sub_82466E20(ctx, base);
	// 825FB8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB8E0 size=112
    let mut pc: u32 = 0x825FB8E0;
    'dispatch: loop {
        match pc {
            0x825FB8E0 => {
    //   block [0x825FB8E0..0x825FB950)
	// 825FB8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB8F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB8F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB8F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB8FC: 390B29F0  addi r8, r11, 0x29f0
	ctx.r[8].s64 = ctx.r[11].s64 + 10736;
	// 825FB900: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FB904: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 825FB908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB918: 386A059C  addi r3, r10, 0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + 1436;
	// 825FB91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB93C: 4BE6B4E5  bl 0x82466e20
	ctx.lr = 0x825FB940;
	sub_82466E20(ctx, base);
	// 825FB940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB950 size=24
    let mut pc: u32 = 0x825FB950;
    'dispatch: loop {
        match pc {
            0x825FB950 => {
    //   block [0x825FB950..0x825FB968)
	// 825FB950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB954: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FB958: 394AA038  addi r10, r10, -0x5fc8
	ctx.r[10].s64 = ctx.r[10].s64 + -24520;
	// 825FB95C: 816B29EC  lwz r11, 0x29ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10732 as u32) ) } as u64;
	// 825FB960: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FB964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB968 size=116
    let mut pc: u32 = 0x825FB968;
    'dispatch: loop {
        match pc {
            0x825FB968 => {
    //   block [0x825FB968..0x825FB9DC)
	// 825FB968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB97C: 390BA038  addi r8, r11, -0x5fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24520;
	// 825FB980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB984: 392A8870  addi r9, r10, -0x7790
	ctx.r[9].s64 = ctx.r[10].s64 + -30608;
	// 825FB988: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB98C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FB990: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB994: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB99C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB9AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB9B0: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 825FB9B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB9B8: 386B05CC  addi r3, r11, 0x5cc
	ctx.r[3].s64 = ctx.r[11].s64 + 1484;
	// 825FB9BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB9C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB9C8: 4BE6B459  bl 0x82466e20
	ctx.lr = 0x825FB9CC;
	sub_82466E20(ctx, base);
	// 825FB9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB9E0 size=112
    let mut pc: u32 = 0x825FB9E0;
    'dispatch: loop {
        match pc {
            0x825FB9E0 => {
    //   block [0x825FB9E0..0x825FBA50)
	// 825FB9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB9F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB9F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB9F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB9FC: 390B2A68  addi r8, r11, 0x2a68
	ctx.r[8].s64 = ctx.r[11].s64 + 10856;
	// 825FBA00: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825FBA04: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 825FBA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBA0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBA18: 386A05FC  addi r3, r10, 0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + 1532;
	// 825FBA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBA3C: 4BE6B3E5  bl 0x82466e20
	ctx.lr = 0x825FBA40;
	sub_82466E20(ctx, base);
	// 825FBA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBA50 size=112
    let mut pc: u32 = 0x825FBA50;
    'dispatch: loop {
        match pc {
            0x825FBA50 => {
    //   block [0x825FBA50..0x825FBAC0)
	// 825FBA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBA5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBA64: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBA68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBA6C: 390B2B58  addi r8, r11, 0x2b58
	ctx.r[8].s64 = ctx.r[11].s64 + 11096;
	// 825FBA70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBA74: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 825FBA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBA7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBA88: 386A062C  addi r3, r10, 0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + 1580;
	// 825FBA8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBAAC: 4BE6B375  bl 0x82466e20
	ctx.lr = 0x825FBAB0;
	sub_82466E20(ctx, base);
	// 825FBAB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FBAC0 size=24
    let mut pc: u32 = 0x825FBAC0;
    'dispatch: loop {
        match pc {
            0x825FBAC0 => {
    //   block [0x825FBAC0..0x825FBAD8)
	// 825FBAC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBAC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FBAC8: 394AA0E0  addi r10, r10, -0x5f20
	ctx.r[10].s64 = ctx.r[10].s64 + -24352;
	// 825FBACC: 816B2BB8  lwz r11, 0x2bb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11192 as u32) ) } as u64;
	// 825FBAD0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825FBAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBAD8 size=116
    let mut pc: u32 = 0x825FBAD8;
    'dispatch: loop {
        match pc {
            0x825FBAD8 => {
    //   block [0x825FBAD8..0x825FBB4C)
	// 825FBAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBAE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBAE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBAEC: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 825FBAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBAF4: 392A88BC  addi r9, r10, -0x7744
	ctx.r[9].s64 = ctx.r[10].s64 + -30532;
	// 825FBAF8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBAFC: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 825FBB00: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBB04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBB0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBB1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FBB20: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825FBB24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBB28: 386B065C  addi r3, r11, 0x65c
	ctx.r[3].s64 = ctx.r[11].s64 + 1628;
	// 825FBB2C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FBB30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBB38: 4BE6B2E9  bl 0x82466e20
	ctx.lr = 0x825FBB3C;
	sub_82466E20(ctx, base);
	// 825FBB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBB50 size=116
    let mut pc: u32 = 0x825FBB50;
    'dispatch: loop {
        match pc {
            0x825FBB50 => {
    //   block [0x825FBB50..0x825FBBC4)
	// 825FBB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBB5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FBB60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBB64: 392B88F4  addi r9, r11, -0x770c
	ctx.r[9].s64 = ctx.r[11].s64 + -30476;
	// 825FBB68: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBB6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBB70: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FBB74: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 825FBB78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBB7C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 825FBB80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBB84: 396B2BC8  addi r11, r11, 0x2bc8
	ctx.r[11].s64 = ctx.r[11].s64 + 11208;
	// 825FBB88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FBB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBB90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FBB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBB98: 386A068C  addi r3, r10, 0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + 1676;
	// 825FBB9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FBBA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FBBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBBA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FBBAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FBBB0: 4BE6B271  bl 0x82466e20
	ctx.lr = 0x825FBBB4;
	sub_82466E20(ctx, base);
	// 825FBBB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBBC8 size=112
    let mut pc: u32 = 0x825FBBC8;
    'dispatch: loop {
        match pc {
            0x825FBBC8 => {
    //   block [0x825FBBC8..0x825FBC38)
	// 825FBBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBBD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBBD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBBDC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FBBE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBBE4: 390B2D90  addi r8, r11, 0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + 11664;
	// 825FBBE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBBEC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 825FBBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBBF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBBF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBC00: 386A06BC  addi r3, r10, 0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + 1724;
	// 825FBC04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBC24: 4BE6B1FD  bl 0x82466e20
	ctx.lr = 0x825FBC28;
	sub_82466E20(ctx, base);
	// 825FBC28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FBC38 size=48
    let mut pc: u32 = 0x825FBC38;
    'dispatch: loop {
        match pc {
            0x825FBC38 => {
    //   block [0x825FBC38..0x825FBC68)
	// 825FBC38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBC3C: 814B2DF0  lwz r10, 0x2df0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11760 as u32) ) } as u64;
	// 825FBC40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBC44: 396BA338  addi r11, r11, -0x5cc8
	ctx.r[11].s64 = ctx.r[11].s64 + -23752;
	// 825FBC48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FBC4C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBC50: 814A2DF4  lwz r10, 0x2df4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11764 as u32) ) } as u64;
	// 825FBC54: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825FBC58: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBC5C: 814A2BC4  lwz r10, 0x2bc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11204 as u32) ) } as u64;
	// 825FBC60: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FBC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBC68 size=116
    let mut pc: u32 = 0x825FBC68;
    'dispatch: loop {
        match pc {
            0x825FBC68 => {
    //   block [0x825FBC68..0x825FBCDC)
	// 825FBC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBC74: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBC7C: 390BA338  addi r8, r11, -0x5cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -23752;
	// 825FBC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBC84: 392A89B0  addi r9, r10, -0x7650
	ctx.r[9].s64 = ctx.r[10].s64 + -30288;
	// 825FBC88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBC8C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 825FBC90: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBC94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBC9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBCAC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FBCB0: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825FBCB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBCB8: 386B06EC  addi r3, r11, 0x6ec
	ctx.r[3].s64 = ctx.r[11].s64 + 1772;
	// 825FBCBC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FBCC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBCC8: 4BE6B159  bl 0x82466e20
	ctx.lr = 0x825FBCCC;
	sub_82466E20(ctx, base);
	// 825FBCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBCE0 size=112
    let mut pc: u32 = 0x825FBCE0;
    'dispatch: loop {
        match pc {
            0x825FBCE0 => {
    //   block [0x825FBCE0..0x825FBD50)
	// 825FBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBCEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBCF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBCF4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBCF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBCFC: 390B2DF8  addi r8, r11, 0x2df8
	ctx.r[8].s64 = ctx.r[11].s64 + 11768;
	// 825FBD00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FBD04: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 825FBD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBD18: 386A071C  addi r3, r10, 0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + 1820;
	// 825FBD1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBD3C: 4BE6B0E5  bl 0x82466e20
	ctx.lr = 0x825FBD40;
	sub_82466E20(ctx, base);
	// 825FBD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBD50 size=112
    let mut pc: u32 = 0x825FBD50;
    'dispatch: loop {
        match pc {
            0x825FBD50 => {
    //   block [0x825FBD50..0x825FBDC0)
	// 825FBD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBD5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBD64: 38AAFBAC  addi r5, r10, -0x454
	ctx.r[5].s64 = ctx.r[10].s64 + -1108;
	// 825FBD68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBD6C: 390B2E40  addi r8, r11, 0x2e40
	ctx.r[8].s64 = ctx.r[11].s64 + 11840;
	// 825FBD70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBD74: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825FBD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBD7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBD88: 386A074C  addi r3, r10, 0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + 1868;
	// 825FBD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBDAC: 4BE6B075  bl 0x82466e20
	ctx.lr = 0x825FBDB0;
	sub_82466E20(ctx, base);
	// 825FBDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBDC0 size=116
    let mut pc: u32 = 0x825FBDC0;
    'dispatch: loop {
        match pc {
            0x825FBDC0 => {
    //   block [0x825FBDC0..0x825FBE34)
	// 825FBDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBDCC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBDD0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FBDD4: 390A2EA0  addi r8, r10, 0x2ea0
	ctx.r[8].s64 = ctx.r[10].s64 + 11936;
	// 825FBDD8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBDDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FBDE0: 38AAFFCC  addi r5, r10, -0x34
	ctx.r[5].s64 = ctx.r[10].s64 + -52;
	// 825FBDE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBDE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBDF4: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 825FBDF8: 396B89EC  addi r11, r11, -0x7614
	ctx.r[11].s64 = ctx.r[11].s64 + -30228;
	// 825FBDFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBE00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBE04: 386A077C  addi r3, r10, 0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + 1916;
	// 825FBE08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FBE0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBE10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FBE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBE20: 4BE6B001  bl 0x82466e20
	ctx.lr = 0x825FBE24;
	sub_82466E20(ctx, base);
	// 825FBE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBE38 size=108
    let mut pc: u32 = 0x825FBE38;
    'dispatch: loop {
        match pc {
            0x825FBE38 => {
    //   block [0x825FBE38..0x825FBEA4)
	// 825FBE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBE44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBE4C: 38EB2F18  addi r7, r11, 0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + 12056;
	// 825FBE50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBE54: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 825FBE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBE5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBE68: 386A07AC  addi r3, r10, 0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1964;
	// 825FBE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBE90: 4BE6AF91  bl 0x82466e20
	ctx.lr = 0x825FBE94;
	sub_82466E20(ctx, base);
	// 825FBE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBEA8 size=108
    let mut pc: u32 = 0x825FBEA8;
    'dispatch: loop {
        match pc {
            0x825FBEA8 => {
    //   block [0x825FBEA8..0x825FBF14)
	// 825FBEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBEB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBEB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBEBC: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 825FBEC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBEC4: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 825FBEC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBED8: 386A07DC  addi r3, r10, 0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + 2012;
	// 825FBEDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBEE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBEE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBEEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBEF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBEFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBF00: 4BE6AF21  bl 0x82466e20
	ctx.lr = 0x825FBF04;
	sub_82466E20(ctx, base);
	// 825FBF04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBF18 size=108
    let mut pc: u32 = 0x825FBF18;
    'dispatch: loop {
        match pc {
            0x825FBF18 => {
    //   block [0x825FBF18..0x825FBF84)
	// 825FBF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBF24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBF2C: 38EB2FA8  addi r7, r11, 0x2fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 12200;
	// 825FBF30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBF34: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 825FBF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBF48: 386A080C  addi r3, r10, 0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + 2060;
	// 825FBF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBF70: 4BE6AEB1  bl 0x82466e20
	ctx.lr = 0x825FBF74;
	sub_82466E20(ctx, base);
	// 825FBF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBF88 size=108
    let mut pc: u32 = 0x825FBF88;
    'dispatch: loop {
        match pc {
            0x825FBF88 => {
    //   block [0x825FBF88..0x825FBFF4)
	// 825FBF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBF94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBF98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBF9C: 38EB2FF0  addi r7, r11, 0x2ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 12272;
	// 825FBFA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FBFA4: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 825FBFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBFAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBFB8: 386A083C  addi r3, r10, 0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + 2108;
	// 825FBFBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBFDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBFE0: 4BE6AE41  bl 0x82466e20
	ctx.lr = 0x825FBFE4;
	sub_82466E20(ctx, base);
	// 825FBFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBFF8 size=108
    let mut pc: u32 = 0x825FBFF8;
    'dispatch: loop {
        match pc {
            0x825FBFF8 => {
    //   block [0x825FBFF8..0x825FC064)
	// 825FBFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC004: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC008: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC00C: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 825FC010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC014: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825FC018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC028: 386A086C  addi r3, r10, 0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + 2156;
	// 825FC02C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC050: 4BE6ADD1  bl 0x82466e20
	ctx.lr = 0x825FC054;
	sub_82466E20(ctx, base);
	// 825FC054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC068 size=112
    let mut pc: u32 = 0x825FC068;
    'dispatch: loop {
        match pc {
            0x825FC068 => {
    //   block [0x825FC068..0x825FC0D8)
	// 825FC068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC078: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC07C: 392A8A2C  addi r9, r10, -0x75d4
	ctx.r[9].s64 = ctx.r[10].s64 + -30164;
	// 825FC080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC084: 390B30D0  addi r8, r11, 0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + 12496;
	// 825FC088: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FC08C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 825FC090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC0A0: 386A089C  addi r3, r10, 0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + 2204;
	// 825FC0A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC0A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FC0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC0C4: 4BE6AD5D  bl 0x82466e20
	ctx.lr = 0x825FC0C8;
	sub_82466E20(ctx, base);
	// 825FC0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC0D8 size=108
    let mut pc: u32 = 0x825FC0D8;
    'dispatch: loop {
        match pc {
            0x825FC0D8 => {
    //   block [0x825FC0D8..0x825FC144)
	// 825FC0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC0E4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC0EC: 38EB3118  addi r7, r11, 0x3118
	ctx.r[7].s64 = ctx.r[11].s64 + 12568;
	// 825FC0F0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FC0F4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825FC0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC108: 386A08CC  addi r3, r10, 0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + 2252;
	// 825FC10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC130: 4BE6ACF1  bl 0x82466e20
	ctx.lr = 0x825FC134;
	sub_82466E20(ctx, base);
	// 825FC134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC148 size=108
    let mut pc: u32 = 0x825FC148;
    'dispatch: loop {
        match pc {
            0x825FC148 => {
    //   block [0x825FC148..0x825FC1B4)
	// 825FC148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC154: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC15C: 38EB3190  addi r7, r11, 0x3190
	ctx.r[7].s64 = ctx.r[11].s64 + 12688;
	// 825FC160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC164: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825FC168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC178: 386A08FC  addi r3, r10, 0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2300;
	// 825FC17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC1A0: 4BE6AC81  bl 0x82466e20
	ctx.lr = 0x825FC1A4;
	sub_82466E20(ctx, base);
	// 825FC1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC1B8 size=108
    let mut pc: u32 = 0x825FC1B8;
    'dispatch: loop {
        match pc {
            0x825FC1B8 => {
    //   block [0x825FC1B8..0x825FC224)
	// 825FC1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC1C4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC1C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC1CC: 38EB31C0  addi r7, r11, 0x31c0
	ctx.r[7].s64 = ctx.r[11].s64 + 12736;
	// 825FC1D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC1D4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825FC1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC1E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC1E8: 386A092C  addi r3, r10, 0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + 2348;
	// 825FC1EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC20C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC210: 4BE6AC11  bl 0x82466e20
	ctx.lr = 0x825FC214;
	sub_82466E20(ctx, base);
	// 825FC214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC228 size=24
    let mut pc: u32 = 0x825FC228;
    'dispatch: loop {
        match pc {
            0x825FC228 => {
    //   block [0x825FC228..0x825FC240)
	// 825FC228: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC22C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC230: 394AA548  addi r10, r10, -0x5ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -23224;
	// 825FC234: 816B31D8  lwz r11, 0x31d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12760 as u32) ) } as u64;
	// 825FC238: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FC23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC240 size=112
    let mut pc: u32 = 0x825FC240;
    'dispatch: loop {
        match pc {
            0x825FC240 => {
    //   block [0x825FC240..0x825FC2B0)
	// 825FC240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC24C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC250: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC254: 392A8A6C  addi r9, r10, -0x7594
	ctx.r[9].s64 = ctx.r[10].s64 + -30100;
	// 825FC258: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC25C: 390BA548  addi r8, r11, -0x5ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -23224;
	// 825FC260: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FC264: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825FC268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC26C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC278: 386A095C  addi r3, r10, 0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + 2396;
	// 825FC27C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC280: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC29C: 4BE6AB85  bl 0x82466e20
	ctx.lr = 0x825FC2A0;
	sub_82466E20(ctx, base);
	// 825FC2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC2B0 size=96
    let mut pc: u32 = 0x825FC2B0;
    'dispatch: loop {
        match pc {
            0x825FC2B0 => {
    //   block [0x825FC2B0..0x825FC310)
	// 825FC2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC2BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC2C4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825FC2C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC2D0: 386A098C  addi r3, r10, 0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + 2444;
	// 825FC2D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC2DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC2F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FC2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC2F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FC2FC: 4BE6AB25  bl 0x82466e20
	ctx.lr = 0x825FC300;
	sub_82466E20(ctx, base);
	// 825FC300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC310 size=112
    let mut pc: u32 = 0x825FC310;
    'dispatch: loop {
        match pc {
            0x825FC310 => {
    //   block [0x825FC310..0x825FC380)
	// 825FC310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC31C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC320: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC324: 38AA098C  addi r5, r10, 0x98c
	ctx.r[5].s64 = ctx.r[10].s64 + 2444;
	// 825FC328: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC32C: 390B31DC  addi r8, r11, 0x31dc
	ctx.r[8].s64 = ctx.r[11].s64 + 12764;
	// 825FC330: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FC334: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825FC338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC33C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC348: 386A09BC  addi r3, r10, 0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + 2492;
	// 825FC34C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FC350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC36C: 4BE6AAB5  bl 0x82466e20
	ctx.lr = 0x825FC370;
	sub_82466E20(ctx, base);
	// 825FC370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC380 size=24
    let mut pc: u32 = 0x825FC380;
    'dispatch: loop {
        match pc {
            0x825FC380 => {
    //   block [0x825FC380..0x825FC398)
	// 825FC380: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC384: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC388: 394AA620  addi r10, r10, -0x59e0
	ctx.r[10].s64 = ctx.r[10].s64 + -23008;
	// 825FC38C: 816B3210  lwz r11, 0x3210(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12816 as u32) ) } as u64;
	// 825FC390: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825FC394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC398 size=112
    let mut pc: u32 = 0x825FC398;
    'dispatch: loop {
        match pc {
            0x825FC398 => {
    //   block [0x825FC398..0x825FC408)
	// 825FC398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC3A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC3AC: 392A8A98  addi r9, r10, -0x7568
	ctx.r[9].s64 = ctx.r[10].s64 + -30056;
	// 825FC3B0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC3B4: 390BA620  addi r8, r11, -0x59e0
	ctx.r[8].s64 = ctx.r[11].s64 + -23008;
	// 825FC3B8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825FC3BC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825FC3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC3C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC3D0: 386A09EC  addi r3, r10, 0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + 2540;
	// 825FC3D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC3D8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FC3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC3EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC3F4: 4BE6AA2D  bl 0x82466e20
	ctx.lr = 0x825FC3F8;
	sub_82466E20(ctx, base);
	// 825FC3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC408 size=108
    let mut pc: u32 = 0x825FC408;
    'dispatch: loop {
        match pc {
            0x825FC408 => {
    //   block [0x825FC408..0x825FC474)
	// 825FC408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC418: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC41C: 38EB3218  addi r7, r11, 0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + 12824;
	// 825FC420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC424: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825FC428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC438: 386A0A1C  addi r3, r10, 0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + 2588;
	// 825FC43C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC460: 4BE6A9C1  bl 0x82466e20
	ctx.lr = 0x825FC464;
	sub_82466E20(ctx, base);
	// 825FC464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC478 size=24
    let mut pc: u32 = 0x825FC478;
    'dispatch: loop {
        match pc {
            0x825FC478 => {
    //   block [0x825FC478..0x825FC490)
	// 825FC478: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC47C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC480: 394AA710  addi r10, r10, -0x58f0
	ctx.r[10].s64 = ctx.r[10].s64 + -22768;
	// 825FC484: 816B3214  lwz r11, 0x3214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12820 as u32) ) } as u64;
	// 825FC488: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FC48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC490 size=112
    let mut pc: u32 = 0x825FC490;
    'dispatch: loop {
        match pc {
            0x825FC490 => {
    //   block [0x825FC490..0x825FC500)
	// 825FC490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC49C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC4A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC4A4: 392A8AC8  addi r9, r10, -0x7538
	ctx.r[9].s64 = ctx.r[10].s64 + -30008;
	// 825FC4A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC4AC: 390BA710  addi r8, r11, -0x58f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22768;
	// 825FC4B0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825FC4B4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825FC4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC4BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC4C8: 386A0A4C  addi r3, r10, 0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + 2636;
	// 825FC4CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC4D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC4EC: 4BE6A935  bl 0x82466e20
	ctx.lr = 0x825FC4F0;
	sub_82466E20(ctx, base);
	// 825FC4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC500 size=40
    let mut pc: u32 = 0x825FC500;
    'dispatch: loop {
        match pc {
            0x825FC500 => {
    //   block [0x825FC500..0x825FC528)
	// 825FC500: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC504: 814B3248  lwz r10, 0x3248(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12872 as u32) ) } as u64;
	// 825FC508: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC50C: 396BA770  addi r11, r11, -0x5890
	ctx.r[11].s64 = ctx.r[11].s64 + -22672;
	// 825FC510: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825FC514: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825FC518: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FC51C: 814A324C  lwz r10, 0x324c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12876 as u32) ) } as u64;
	// 825FC520: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825FC524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC528 size=112
    let mut pc: u32 = 0x825FC528;
    'dispatch: loop {
        match pc {
            0x825FC528 => {
    //   block [0x825FC528..0x825FC598)
	// 825FC528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC534: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC538: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC53C: 392A8C40  addi r9, r10, -0x73c0
	ctx.r[9].s64 = ctx.r[10].s64 + -29632;
	// 825FC540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC544: 390BA770  addi r8, r11, -0x5890
	ctx.r[8].s64 = ctx.r[11].s64 + -22672;
	// 825FC548: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FC54C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825FC550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC560: 386A0A7C  addi r3, r10, 0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + 2684;
	// 825FC564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC568: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FC56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC584: 4BE6A89D  bl 0x82466e20
	ctx.lr = 0x825FC588;
	sub_82466E20(ctx, base);
	// 825FC588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC598 size=108
    let mut pc: u32 = 0x825FC598;
    'dispatch: loop {
        match pc {
            0x825FC598 => {
    //   block [0x825FC598..0x825FC604)
	// 825FC598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC5A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC5A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC5AC: 38EB3254  addi r7, r11, 0x3254
	ctx.r[7].s64 = ctx.r[11].s64 + 12884;
	// 825FC5B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC5B4: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 825FC5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC5BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC5C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC5C8: 386A0AAC  addi r3, r10, 0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + 2732;
	// 825FC5CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC5F0: 4BE6A831  bl 0x82466e20
	ctx.lr = 0x825FC5F4;
	sub_82466E20(ctx, base);
	// 825FC5F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC608 size=108
    let mut pc: u32 = 0x825FC608;
    'dispatch: loop {
        match pc {
            0x825FC608 => {
    //   block [0x825FC608..0x825FC674)
	// 825FC608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC614: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC618: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC61C: 38EB3284  addi r7, r11, 0x3284
	ctx.r[7].s64 = ctx.r[11].s64 + 12932;
	// 825FC620: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC624: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 825FC628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC62C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC638: 386A0ADC  addi r3, r10, 0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + 2780;
	// 825FC63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC660: 4BE6A7C1  bl 0x82466e20
	ctx.lr = 0x825FC664;
	sub_82466E20(ctx, base);
	// 825FC664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC678 size=108
    let mut pc: u32 = 0x825FC678;
    'dispatch: loop {
        match pc {
            0x825FC678 => {
    //   block [0x825FC678..0x825FC6E4)
	// 825FC678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC684: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC688: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC68C: 38EB329C  addi r7, r11, 0x329c
	ctx.r[7].s64 = ctx.r[11].s64 + 12956;
	// 825FC690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC694: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825FC698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC69C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC6A8: 386A0B0C  addi r3, r10, 0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + 2828;
	// 825FC6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC6D0: 4BE6A751  bl 0x82466e20
	ctx.lr = 0x825FC6D4;
	sub_82466E20(ctx, base);
	// 825FC6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC6E8 size=108
    let mut pc: u32 = 0x825FC6E8;
    'dispatch: loop {
        match pc {
            0x825FC6E8 => {
    //   block [0x825FC6E8..0x825FC754)
	// 825FC6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC6F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC6F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC6FC: 38EB32D0  addi r7, r11, 0x32d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13008;
	// 825FC700: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FC704: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 825FC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC70C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC718: 386A0B3C  addi r3, r10, 0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + 2876;
	// 825FC71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC740: 4BE6A6E1  bl 0x82466e20
	ctx.lr = 0x825FC744;
	sub_82466E20(ctx, base);
	// 825FC744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC758 size=108
    let mut pc: u32 = 0x825FC758;
    'dispatch: loop {
        match pc {
            0x825FC758 => {
    //   block [0x825FC758..0x825FC7C4)
	// 825FC758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC764: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC768: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC76C: 38EB3360  addi r7, r11, 0x3360
	ctx.r[7].s64 = ctx.r[11].s64 + 13152;
	// 825FC770: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC774: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825FC778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC77C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC788: 386A0B6C  addi r3, r10, 0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + 2924;
	// 825FC78C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC7AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC7B0: 4BE6A671  bl 0x82466e20
	ctx.lr = 0x825FC7B4;
	sub_82466E20(ctx, base);
	// 825FC7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC7C8 size=108
    let mut pc: u32 = 0x825FC7C8;
    'dispatch: loop {
        match pc {
            0x825FC7C8 => {
    //   block [0x825FC7C8..0x825FC834)
	// 825FC7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC7D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC7D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC7DC: 38EB3378  addi r7, r11, 0x3378
	ctx.r[7].s64 = ctx.r[11].s64 + 13176;
	// 825FC7E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC7E4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825FC7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC7EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC7F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC7F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC7F8: 386A0B9C  addi r3, r10, 0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + 2972;
	// 825FC7FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC81C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC820: 4BE6A601  bl 0x82466e20
	ctx.lr = 0x825FC824;
	sub_82466E20(ctx, base);
	// 825FC824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC838 size=112
    let mut pc: u32 = 0x825FC838;
    'dispatch: loop {
        match pc {
            0x825FC838 => {
    //   block [0x825FC838..0x825FC8A8)
	// 825FC838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC844: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC848: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC84C: 392A8C94  addi r9, r10, -0x736c
	ctx.r[9].s64 = ctx.r[10].s64 + -29548;
	// 825FC850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC854: 390B33A8  addi r8, r11, 0x33a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13224;
	// 825FC858: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FC85C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825FC860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC864: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC870: 386A0BCC  addi r3, r10, 0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + 3020;
	// 825FC874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC878: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC894: 4BE6A58D  bl 0x82466e20
	ctx.lr = 0x825FC898;
	sub_82466E20(ctx, base);
	// 825FC898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC8A8 size=108
    let mut pc: u32 = 0x825FC8A8;
    'dispatch: loop {
        match pc {
            0x825FC8A8 => {
    //   block [0x825FC8A8..0x825FC914)
	// 825FC8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC8B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC8B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC8BC: 38EB3420  addi r7, r11, 0x3420
	ctx.r[7].s64 = ctx.r[11].s64 + 13344;
	// 825FC8C0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FC8C4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825FC8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC8CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC8D8: 386A0BFC  addi r3, r10, 0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3068;
	// 825FC8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC900: 4BE6A521  bl 0x82466e20
	ctx.lr = 0x825FC904;
	sub_82466E20(ctx, base);
	// 825FC904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC918 size=24
    let mut pc: u32 = 0x825FC918;
    'dispatch: loop {
        match pc {
            0x825FC918 => {
    //   block [0x825FC918..0x825FC930)
	// 825FC918: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC91C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC920: 394AA848  addi r10, r10, -0x57b8
	ctx.r[10].s64 = ctx.r[10].s64 + -22456;
	// 825FC924: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FC928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FC92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC930 size=108
    let mut pc: u32 = 0x825FC930;
    'dispatch: loop {
        match pc {
            0x825FC930 => {
    //   block [0x825FC930..0x825FC99C)
	// 825FC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC93C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC944: 38EBA848  addi r7, r11, -0x57b8
	ctx.r[7].s64 = ctx.r[11].s64 + -22456;
	// 825FC948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC94C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825FC950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC960: 386A0C2C  addi r3, r10, 0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3116;
	// 825FC964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC988: 4BE6A499  bl 0x82466e20
	ctx.lr = 0x825FC98C;
	sub_82466E20(ctx, base);
	// 825FC98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC9A0 size=24
    let mut pc: u32 = 0x825FC9A0;
    'dispatch: loop {
        match pc {
            0x825FC9A0 => {
    //   block [0x825FC9A0..0x825FC9B8)
	// 825FC9A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC9A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC9A8: 394AA878  addi r10, r10, -0x5788
	ctx.r[10].s64 = ctx.r[10].s64 + -22408;
	// 825FC9AC: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FC9B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FC9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC9B8 size=108
    let mut pc: u32 = 0x825FC9B8;
    'dispatch: loop {
        match pc {
            0x825FC9B8 => {
    //   block [0x825FC9B8..0x825FCA24)
	// 825FC9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC9C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC9CC: 38EBA878  addi r7, r11, -0x5788
	ctx.r[7].s64 = ctx.r[11].s64 + -22408;
	// 825FC9D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC9D4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825FC9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC9DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC9E8: 386A0C5C  addi r3, r10, 0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3164;
	// 825FC9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCA10: 4BE6A411  bl 0x82466e20
	ctx.lr = 0x825FCA14;
	sub_82466E20(ctx, base);
	// 825FCA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCA28 size=108
    let mut pc: u32 = 0x825FCA28;
    'dispatch: loop {
        match pc {
            0x825FCA28 => {
    //   block [0x825FCA28..0x825FCA94)
	// 825FCA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCA34: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCA3C: 38EB34F8  addi r7, r11, 0x34f8
	ctx.r[7].s64 = ctx.r[11].s64 + 13560;
	// 825FCA40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCA44: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825FCA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCA4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCA50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCA58: 386A0C8C  addi r3, r10, 0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3212;
	// 825FCA5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCA80: 4BE6A3A1  bl 0x82466e20
	ctx.lr = 0x825FCA84;
	sub_82466E20(ctx, base);
	// 825FCA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FCA98 size=24
    let mut pc: u32 = 0x825FCA98;
    'dispatch: loop {
        match pc {
            0x825FCA98 => {
    //   block [0x825FCA98..0x825FCAB0)
	// 825FCA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCA9C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FCAA0: 394AA8A8  addi r10, r10, -0x5758
	ctx.r[10].s64 = ctx.r[10].s64 + -22360;
	// 825FCAA4: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FCAA8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FCAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCAB0 size=108
    let mut pc: u32 = 0x825FCAB0;
    'dispatch: loop {
        match pc {
            0x825FCAB0 => {
    //   block [0x825FCAB0..0x825FCB1C)
	// 825FCAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCABC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FCAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCAC4: 38EBA8A8  addi r7, r11, -0x5758
	ctx.r[7].s64 = ctx.r[11].s64 + -22360;
	// 825FCAC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCACC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825FCAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCAD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCAE0: 386A0CBC  addi r3, r10, 0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + 3260;
	// 825FCAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCB08: 4BE6A319  bl 0x82466e20
	ctx.lr = 0x825FCB0C;
	sub_82466E20(ctx, base);
	// 825FCB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCB20 size=112
    let mut pc: u32 = 0x825FCB20;
    'dispatch: loop {
        match pc {
            0x825FCB20 => {
    //   block [0x825FCB20..0x825FCB90)
	// 825FCB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCB2C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FCB30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCB34: 392A8CD8  addi r9, r10, -0x7328
	ctx.r[9].s64 = ctx.r[10].s64 + -29480;
	// 825FCB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCB3C: 390B3514  addi r8, r11, 0x3514
	ctx.r[8].s64 = ctx.r[11].s64 + 13588;
	// 825FCB40: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FCB44: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825FCB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCB4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FCB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCB58: 386A0CEC  addi r3, r10, 0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + 3308;
	// 825FCB5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FCB60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FCB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCB7C: 4BE6A2A5  bl 0x82466e20
	ctx.lr = 0x825FCB80;
	sub_82466E20(ctx, base);
	// 825FCB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCB90 size=108
    let mut pc: u32 = 0x825FCB90;
    'dispatch: loop {
        match pc {
            0x825FCB90 => {
    //   block [0x825FCB90..0x825FCBFC)
	// 825FCB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCB9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCBA4: 38EB3544  addi r7, r11, 0x3544
	ctx.r[7].s64 = ctx.r[11].s64 + 13636;
	// 825FCBA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCBAC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825FCBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCBC0: 386A0D1C  addi r3, r10, 0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3356;
	// 825FCBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCBE8: 4BE6A239  bl 0x82466e20
	ctx.lr = 0x825FCBEC;
	sub_82466E20(ctx, base);
	// 825FCBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCC00 size=108
    let mut pc: u32 = 0x825FCC00;
    'dispatch: loop {
        match pc {
            0x825FCC00 => {
    //   block [0x825FCC00..0x825FCC6C)
	// 825FCC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCC0C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCC14: 38EB3574  addi r7, r11, 0x3574
	ctx.r[7].s64 = ctx.r[11].s64 + 13684;
	// 825FCC18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCC1C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825FCC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCC24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCC30: 386A0D4C  addi r3, r10, 0xd4c
	ctx.r[3].s64 = ctx.r[10].s64 + 3404;
	// 825FCC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCC58: 4BE6A1C9  bl 0x82466e20
	ctx.lr = 0x825FCC5C;
	sub_82466E20(ctx, base);
	// 825FCC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCC70 size=108
    let mut pc: u32 = 0x825FCC70;
    'dispatch: loop {
        match pc {
            0x825FCC70 => {
    //   block [0x825FCC70..0x825FCCDC)
	// 825FCC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCC7C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCC84: 38EB358C  addi r7, r11, 0x358c
	ctx.r[7].s64 = ctx.r[11].s64 + 13708;
	// 825FCC88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCC8C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825FCC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCC94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCCA0: 386A0D7C  addi r3, r10, 0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3452;
	// 825FCCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCCC8: 4BE6A159  bl 0x82466e20
	ctx.lr = 0x825FCCCC;
	sub_82466E20(ctx, base);
	// 825FCCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCCE0 size=112
    let mut pc: u32 = 0x825FCCE0;
    'dispatch: loop {
        match pc {
            0x825FCCE0 => {
    //   block [0x825FCCE0..0x825FCD50)
	// 825FCCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCCEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCCF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCCF4: 38AA0DDC  addi r5, r10, 0xddc
	ctx.r[5].s64 = ctx.r[10].s64 + 3548;
	// 825FCCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCCFC: 390B35BC  addi r8, r11, 0x35bc
	ctx.r[8].s64 = ctx.r[11].s64 + 13756;
	// 825FCD00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FCD04: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825FCD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FCD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCD18: 386A0DAC  addi r3, r10, 0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + 3500;
	// 825FCD1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FCD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCD3C: 4BE6A0E5  bl 0x82466e20
	ctx.lr = 0x825FCD40;
	sub_82466E20(ctx, base);
	// 825FCD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCD50 size=108
    let mut pc: u32 = 0x825FCD50;
    'dispatch: loop {
        match pc {
            0x825FCD50 => {
    //   block [0x825FCD50..0x825FCDBC)
	// 825FCD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCD5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCD64: 38EB35D4  addi r7, r11, 0x35d4
	ctx.r[7].s64 = ctx.r[11].s64 + 13780;
	// 825FCD68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCD6C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825FCD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCD78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCD80: 386A0DDC  addi r3, r10, 0xddc
	ctx.r[3].s64 = ctx.r[10].s64 + 3548;
	// 825FCD84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCDA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCDA8: 4BE6A079  bl 0x82466e20
	ctx.lr = 0x825FCDAC;
	sub_82466E20(ctx, base);
	// 825FCDAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCDC0 size=108
    let mut pc: u32 = 0x825FCDC0;
    'dispatch: loop {
        match pc {
            0x825FCDC0 => {
    //   block [0x825FCDC0..0x825FCE2C)
	// 825FCDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCDCC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCDD4: 38EB3604  addi r7, r11, 0x3604
	ctx.r[7].s64 = ctx.r[11].s64 + 13828;
	// 825FCDD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCDDC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825FCDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCDE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCDE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCDF0: 386A0E0C  addi r3, r10, 0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + 3596;
	// 825FCDF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCE18: 4BE6A009  bl 0x82466e20
	ctx.lr = 0x825FCE1C;
	sub_82466E20(ctx, base);
	// 825FCE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCE30 size=108
    let mut pc: u32 = 0x825FCE30;
    'dispatch: loop {
        match pc {
            0x825FCE30 => {
    //   block [0x825FCE30..0x825FCE9C)
	// 825FCE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCE3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCE44: 38EB361C  addi r7, r11, 0x361c
	ctx.r[7].s64 = ctx.r[11].s64 + 13852;
	// 825FCE48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCE4C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825FCE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCE58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCE60: 386A0E3C  addi r3, r10, 0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + 3644;
	// 825FCE64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCE88: 4BE69F99  bl 0x82466e20
	ctx.lr = 0x825FCE8C;
	sub_82466E20(ctx, base);
	// 825FCE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCEA0 size=108
    let mut pc: u32 = 0x825FCEA0;
    'dispatch: loop {
        match pc {
            0x825FCEA0 => {
    //   block [0x825FCEA0..0x825FCF0C)
	// 825FCEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCEAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCEB4: 38EB3650  addi r7, r11, 0x3650
	ctx.r[7].s64 = ctx.r[11].s64 + 13904;
	// 825FCEB8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FCEBC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825FCEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCEC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCED0: 386A0E6C  addi r3, r10, 0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3692;
	// 825FCED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCEF8: 4BE69F29  bl 0x82466e20
	ctx.lr = 0x825FCEFC;
	sub_82466E20(ctx, base);
	// 825FCEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCF10 size=108
    let mut pc: u32 = 0x825FCF10;
    'dispatch: loop {
        match pc {
            0x825FCF10 => {
    //   block [0x825FCF10..0x825FCF7C)
	// 825FCF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCF1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCF24: 38EB36F8  addi r7, r11, 0x36f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14072;
	// 825FCF28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCF2C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825FCF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCF34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCF38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCF40: 386A0E9C  addi r3, r10, 0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3740;
	// 825FCF44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCF68: 4BE69EB9  bl 0x82466e20
	ctx.lr = 0x825FCF6C;
	sub_82466E20(ctx, base);
	// 825FCF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCF80 size=108
    let mut pc: u32 = 0x825FCF80;
    'dispatch: loop {
        match pc {
            0x825FCF80 => {
    //   block [0x825FCF80..0x825FCFEC)
	// 825FCF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCF8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCF94: 38EB3728  addi r7, r11, 0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + 14120;
	// 825FCF98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCF9C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825FCFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCFA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCFB0: 386A0ECC  addi r3, r10, 0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + 3788;
	// 825FCFB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCFD8: 4BE69E49  bl 0x82466e20
	ctx.lr = 0x825FCFDC;
	sub_82466E20(ctx, base);
	// 825FCFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCFF0 size=108
    let mut pc: u32 = 0x825FCFF0;
    'dispatch: loop {
        match pc {
            0x825FCFF0 => {
    //   block [0x825FCFF0..0x825FD05C)
	// 825FCFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCFFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD004: 38EB3740  addi r7, r11, 0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + 14144;
	// 825FD008: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD00C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825FD010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD020: 386A0EFC  addi r3, r10, 0xefc
	ctx.r[3].s64 = ctx.r[10].s64 + 3836;
	// 825FD024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD048: 4BE69DD9  bl 0x82466e20
	ctx.lr = 0x825FD04C;
	sub_82466E20(ctx, base);
	// 825FD04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD060 size=112
    let mut pc: u32 = 0x825FD060;
    'dispatch: loop {
        match pc {
            0x825FD060 => {
    //   block [0x825FD060..0x825FD0D0)
	// 825FD060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD070: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD074: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD07C: 390B3770  addi r8, r11, 0x3770
	ctx.r[8].s64 = ctx.r[11].s64 + 14192;
	// 825FD080: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FD084: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825FD088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD098: 386A0F2C  addi r3, r10, 0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3884;
	// 825FD09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FD0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD0BC: 4BE69D65  bl 0x82466e20
	ctx.lr = 0x825FD0C0;
	sub_82466E20(ctx, base);
	// 825FD0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD0D0 size=24
    let mut pc: u32 = 0x825FD0D0;
    'dispatch: loop {
        match pc {
            0x825FD0D0 => {
    //   block [0x825FD0D0..0x825FD0E8)
	// 825FD0D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD0D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD0D8: 394AA8D8  addi r10, r10, -0x5728
	ctx.r[10].s64 = ctx.r[10].s64 + -22312;
	// 825FD0DC: 816B364C  lwz r11, 0x364c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13900 as u32) ) } as u64;
	// 825FD0E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FD0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD0E8 size=112
    let mut pc: u32 = 0x825FD0E8;
    'dispatch: loop {
        match pc {
            0x825FD0E8 => {
    //   block [0x825FD0E8..0x825FD158)
	// 825FD0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD0F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD0F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD0FC: 392A8D04  addi r9, r10, -0x72fc
	ctx.r[9].s64 = ctx.r[10].s64 + -29436;
	// 825FD100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD104: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 825FD108: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FD10C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825FD110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD114: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD120: 386A0F5C  addi r3, r10, 0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3932;
	// 825FD124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD144: 4BE69CDD  bl 0x82466e20
	ctx.lr = 0x825FD148;
	sub_82466E20(ctx, base);
	// 825FD148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD158 size=108
    let mut pc: u32 = 0x825FD158;
    'dispatch: loop {
        match pc {
            0x825FD158 => {
    //   block [0x825FD158..0x825FD1C4)
	// 825FD158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD164: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD16C: 38EB381C  addi r7, r11, 0x381c
	ctx.r[7].s64 = ctx.r[11].s64 + 14364;
	// 825FD170: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD174: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825FD178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD188: 386A0F8C  addi r3, r10, 0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3980;
	// 825FD18C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD1B0: 4BE69C71  bl 0x82466e20
	ctx.lr = 0x825FD1B4;
	sub_82466E20(ctx, base);
	// 825FD1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD1C8 size=116
    let mut pc: u32 = 0x825FD1C8;
    'dispatch: loop {
        match pc {
            0x825FD1C8 => {
    //   block [0x825FD1C8..0x825FD23C)
	// 825FD1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD1D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD1DC: 390B3850  addi r8, r11, 0x3850
	ctx.r[8].s64 = ctx.r[11].s64 + 14416;
	// 825FD1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD1E4: 392A8D48  addi r9, r10, -0x72b8
	ctx.r[9].s64 = ctx.r[10].s64 + -29368;
	// 825FD1E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD1EC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825FD1F0: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD1F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD1FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD20C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FD210: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825FD214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD218: 386B0FBC  addi r3, r11, 0xfbc
	ctx.r[3].s64 = ctx.r[11].s64 + 4028;
	// 825FD21C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD228: 4BE69BF9  bl 0x82466e20
	ctx.lr = 0x825FD22C;
	sub_82466E20(ctx, base);
	// 825FD22C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD240 size=24
    let mut pc: u32 = 0x825FD240;
    'dispatch: loop {
        match pc {
            0x825FD240 => {
    //   block [0x825FD240..0x825FD258)
	// 825FD240: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD244: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD248: 394AA950  addi r10, r10, -0x56b0
	ctx.r[10].s64 = ctx.r[10].s64 + -22192;
	// 825FD24C: 816B384C  lwz r11, 0x384c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14412 as u32) ) } as u64;
	// 825FD250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FD254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD258 size=112
    let mut pc: u32 = 0x825FD258;
    'dispatch: loop {
        match pc {
            0x825FD258 => {
    //   block [0x825FD258..0x825FD2C8)
	// 825FD258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD264: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD26C: 392A8D84  addi r9, r10, -0x727c
	ctx.r[9].s64 = ctx.r[10].s64 + -29308;
	// 825FD270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD274: 390BA950  addi r8, r11, -0x56b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22192;
	// 825FD278: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FD27C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 825FD280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD290: 386A0FEC  addi r3, r10, 0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + 4076;
	// 825FD294: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD298: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD2AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD2B4: 4BE69B6D  bl 0x82466e20
	ctx.lr = 0x825FD2B8;
	sub_82466E20(ctx, base);
	// 825FD2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD2C8 size=108
    let mut pc: u32 = 0x825FD2C8;
    'dispatch: loop {
        match pc {
            0x825FD2C8 => {
    //   block [0x825FD2C8..0x825FD334)
	// 825FD2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD2D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD2D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD2DC: 38EB3910  addi r7, r11, 0x3910
	ctx.r[7].s64 = ctx.r[11].s64 + 14608;
	// 825FD2E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD2E4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 825FD2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD2EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD2F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD2F8: 386A101C  addi r3, r10, 0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + 4124;
	// 825FD2FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD31C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD320: 4BE69B01  bl 0x82466e20
	ctx.lr = 0x825FD324;
	sub_82466E20(ctx, base);
	// 825FD324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD338 size=108
    let mut pc: u32 = 0x825FD338;
    'dispatch: loop {
        match pc {
            0x825FD338 => {
    //   block [0x825FD338..0x825FD3A4)
	// 825FD338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD344: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD34C: 38EB3928  addi r7, r11, 0x3928
	ctx.r[7].s64 = ctx.r[11].s64 + 14632;
	// 825FD350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD354: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 825FD358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD368: 386A104C  addi r3, r10, 0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + 4172;
	// 825FD36C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD390: 4BE69A91  bl 0x82466e20
	ctx.lr = 0x825FD394;
	sub_82466E20(ctx, base);
	// 825FD394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD3A8 size=24
    let mut pc: u32 = 0x825FD3A8;
    'dispatch: loop {
        match pc {
            0x825FD3A8 => {
    //   block [0x825FD3A8..0x825FD3C0)
	// 825FD3A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD3AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD3B0: 394AA998  addi r10, r10, -0x5668
	ctx.r[10].s64 = ctx.r[10].s64 + -22120;
	// 825FD3B4: 816B3958  lwz r11, 0x3958(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14680 as u32) ) } as u64;
	// 825FD3B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FD3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD3C0 size=112
    let mut pc: u32 = 0x825FD3C0;
    'dispatch: loop {
        match pc {
            0x825FD3C0 => {
    //   block [0x825FD3C0..0x825FD430)
	// 825FD3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD3CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD3D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD3D4: 392A8DC0  addi r9, r10, -0x7240
	ctx.r[9].s64 = ctx.r[10].s64 + -29248;
	// 825FD3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD3DC: 390BA998  addi r8, r11, -0x5668
	ctx.r[8].s64 = ctx.r[11].s64 + -22120;
	// 825FD3E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FD3E4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 825FD3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD3EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD3F8: 386A107C  addi r3, r10, 0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + 4220;
	// 825FD3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD400: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD41C: 4BE69A05  bl 0x82466e20
	ctx.lr = 0x825FD420;
	sub_82466E20(ctx, base);
	// 825FD420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD430 size=112
    let mut pc: u32 = 0x825FD430;
    'dispatch: loop {
        match pc {
            0x825FD430 => {
    //   block [0x825FD430..0x825FD4A0)
	// 825FD430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD440: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD444: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD44C: 390B395C  addi r8, r11, 0x395c
	ctx.r[8].s64 = ctx.r[11].s64 + 14684;
	// 825FD450: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FD454: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 825FD458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD468: 386A10AC  addi r3, r10, 0x10ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4268;
	// 825FD46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FD470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD48C: 4BE69995  bl 0x82466e20
	ctx.lr = 0x825FD490;
	sub_82466E20(ctx, base);
	// 825FD490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD4A0 size=108
    let mut pc: u32 = 0x825FD4A0;
    'dispatch: loop {
        match pc {
            0x825FD4A0 => {
    //   block [0x825FD4A0..0x825FD50C)
	// 825FD4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD4AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD4B4: 38EB398C  addi r7, r11, 0x398c
	ctx.r[7].s64 = ctx.r[11].s64 + 14732;
	// 825FD4B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD4BC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 825FD4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD4D0: 386A10DC  addi r3, r10, 0x10dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4316;
	// 825FD4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD4F8: 4BE69929  bl 0x82466e20
	ctx.lr = 0x825FD4FC;
	sub_82466E20(ctx, base);
	// 825FD4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD510 size=108
    let mut pc: u32 = 0x825FD510;
    'dispatch: loop {
        match pc {
            0x825FD510 => {
    //   block [0x825FD510..0x825FD57C)
	// 825FD510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD51C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD524: 38EB39C0  addi r7, r11, 0x39c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14784;
	// 825FD528: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FD52C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825FD530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD540: 386A110C  addi r3, r10, 0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + 4364;
	// 825FD544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD568: 4BE698B9  bl 0x82466e20
	ctx.lr = 0x825FD56C;
	sub_82466E20(ctx, base);
	// 825FD56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD580 size=108
    let mut pc: u32 = 0x825FD580;
    'dispatch: loop {
        match pc {
            0x825FD580 => {
    //   block [0x825FD580..0x825FD5EC)
	// 825FD580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD58C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD594: 38EB3A20  addi r7, r11, 0x3a20
	ctx.r[7].s64 = ctx.r[11].s64 + 14880;
	// 825FD598: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD59C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 825FD5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD5B0: 386A113C  addi r3, r10, 0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + 4412;
	// 825FD5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD5D8: 4BE69849  bl 0x82466e20
	ctx.lr = 0x825FD5DC;
	sub_82466E20(ctx, base);
	// 825FD5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD5F0 size=108
    let mut pc: u32 = 0x825FD5F0;
    'dispatch: loop {
        match pc {
            0x825FD5F0 => {
    //   block [0x825FD5F0..0x825FD65C)
	// 825FD5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD5FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD604: 38EB3A50  addi r7, r11, 0x3a50
	ctx.r[7].s64 = ctx.r[11].s64 + 14928;
	// 825FD608: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825FD60C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 825FD610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD620: 386A116C  addi r3, r10, 0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + 4460;
	// 825FD624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD648: 4BE697D9  bl 0x82466e20
	ctx.lr = 0x825FD64C;
	sub_82466E20(ctx, base);
	// 825FD64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD660 size=108
    let mut pc: u32 = 0x825FD660;
    'dispatch: loop {
        match pc {
            0x825FD660 => {
    //   block [0x825FD660..0x825FD6CC)
	// 825FD660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD66C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD674: 38EB3B70  addi r7, r11, 0x3b70
	ctx.r[7].s64 = ctx.r[11].s64 + 15216;
	// 825FD678: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD67C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 825FD680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD690: 386A119C  addi r3, r10, 0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + 4508;
	// 825FD694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD6B8: 4BE69769  bl 0x82466e20
	ctx.lr = 0x825FD6BC;
	sub_82466E20(ctx, base);
	// 825FD6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD6D0 size=108
    let mut pc: u32 = 0x825FD6D0;
    'dispatch: loop {
        match pc {
            0x825FD6D0 => {
    //   block [0x825FD6D0..0x825FD73C)
	// 825FD6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD6DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD6E4: 38EB3B88  addi r7, r11, 0x3b88
	ctx.r[7].s64 = ctx.r[11].s64 + 15240;
	// 825FD6E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD6EC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 825FD6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD6F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD700: 386A11CC  addi r3, r10, 0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4556;
	// 825FD704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD728: 4BE696F9  bl 0x82466e20
	ctx.lr = 0x825FD72C;
	sub_82466E20(ctx, base);
	// 825FD72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD740 size=108
    let mut pc: u32 = 0x825FD740;
    'dispatch: loop {
        match pc {
            0x825FD740 => {
    //   block [0x825FD740..0x825FD7AC)
	// 825FD740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD74C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD754: 38EB3BA0  addi r7, r11, 0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + 15264;
	// 825FD758: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD75C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 825FD760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD770: 386A11FC  addi r3, r10, 0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4604;
	// 825FD774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD798: 4BE69689  bl 0x82466e20
	ctx.lr = 0x825FD79C;
	sub_82466E20(ctx, base);
	// 825FD79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD7B0 size=108
    let mut pc: u32 = 0x825FD7B0;
    'dispatch: loop {
        match pc {
            0x825FD7B0 => {
    //   block [0x825FD7B0..0x825FD81C)
	// 825FD7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD7BC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD7C4: 38EB3BB8  addi r7, r11, 0x3bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15288;
	// 825FD7C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD7CC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 825FD7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD7D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD7E0: 386A122C  addi r3, r10, 0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + 4652;
	// 825FD7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD808: 4BE69619  bl 0x82466e20
	ctx.lr = 0x825FD80C;
	sub_82466E20(ctx, base);
	// 825FD80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD820 size=108
    let mut pc: u32 = 0x825FD820;
    'dispatch: loop {
        match pc {
            0x825FD820 => {
    //   block [0x825FD820..0x825FD88C)
	// 825FD820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD82C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD834: 38EB3BD0  addi r7, r11, 0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 15312;
	// 825FD838: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD83C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825FD840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD84C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD850: 386A125C  addi r3, r10, 0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + 4700;
	// 825FD854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD878: 4BE695A9  bl 0x82466e20
	ctx.lr = 0x825FD87C;
	sub_82466E20(ctx, base);
	// 825FD87C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD890 size=108
    let mut pc: u32 = 0x825FD890;
    'dispatch: loop {
        match pc {
            0x825FD890 => {
    //   block [0x825FD890..0x825FD8FC)
	// 825FD890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD89C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD8A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD8A4: 38EB3BE8  addi r7, r11, 0x3be8
	ctx.r[7].s64 = ctx.r[11].s64 + 15336;
	// 825FD8A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD8AC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 825FD8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD8B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD8C0: 386A128C  addi r3, r10, 0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + 4748;
	// 825FD8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD8DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD8E8: 4BE69539  bl 0x82466e20
	ctx.lr = 0x825FD8EC;
	sub_82466E20(ctx, base);
	// 825FD8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD900 size=108
    let mut pc: u32 = 0x825FD900;
    'dispatch: loop {
        match pc {
            0x825FD900 => {
    //   block [0x825FD900..0x825FD96C)
	// 825FD900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD90C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD914: 38EB3C00  addi r7, r11, 0x3c00
	ctx.r[7].s64 = ctx.r[11].s64 + 15360;
	// 825FD918: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FD91C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 825FD920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD930: 386A12BC  addi r3, r10, 0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4796;
	// 825FD934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD958: 4BE694C9  bl 0x82466e20
	ctx.lr = 0x825FD95C;
	sub_82466E20(ctx, base);
	// 825FD95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD970 size=108
    let mut pc: u32 = 0x825FD970;
    'dispatch: loop {
        match pc {
            0x825FD970 => {
    //   block [0x825FD970..0x825FD9DC)
	// 825FD970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD97C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD984: 38EB3C90  addi r7, r11, 0x3c90
	ctx.r[7].s64 = ctx.r[11].s64 + 15504;
	// 825FD988: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FD98C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 825FD990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD9A0: 386A12EC  addi r3, r10, 0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4844;
	// 825FD9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD9C8: 4BE69459  bl 0x82466e20
	ctx.lr = 0x825FD9CC;
	sub_82466E20(ctx, base);
	// 825FD9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD9E0 size=108
    let mut pc: u32 = 0x825FD9E0;
    'dispatch: loop {
        match pc {
            0x825FD9E0 => {
    //   block [0x825FD9E0..0x825FDA4C)
	// 825FD9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD9EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD9F4: 38EB3D50  addi r7, r11, 0x3d50
	ctx.r[7].s64 = ctx.r[11].s64 + 15696;
	// 825FD9F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FD9FC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 825FDA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDA10: 386A131C  addi r3, r10, 0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + 4892;
	// 825FDA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDA38: 4BE693E9  bl 0x82466e20
	ctx.lr = 0x825FDA3C;
	sub_82466E20(ctx, base);
	// 825FDA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDA50 size=108
    let mut pc: u32 = 0x825FDA50;
    'dispatch: loop {
        match pc {
            0x825FDA50 => {
    //   block [0x825FDA50..0x825FDABC)
	// 825FDA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDA5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDA64: 38EB3E28  addi r7, r11, 0x3e28
	ctx.r[7].s64 = ctx.r[11].s64 + 15912;
	// 825FDA68: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FDA6C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 825FDA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDA80: 386A134C  addi r3, r10, 0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + 4940;
	// 825FDA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDAA8: 4BE69379  bl 0x82466e20
	ctx.lr = 0x825FDAAC;
	sub_82466E20(ctx, base);
	// 825FDAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDAC0 size=108
    let mut pc: u32 = 0x825FDAC0;
    'dispatch: loop {
        match pc {
            0x825FDAC0 => {
    //   block [0x825FDAC0..0x825FDB2C)
	// 825FDAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDACC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDAD4: 38EB3EE8  addi r7, r11, 0x3ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 16104;
	// 825FDAD8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FDADC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 825FDAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDAE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDAF0: 386A137C  addi r3, r10, 0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + 4988;
	// 825FDAF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDB18: 4BE69309  bl 0x82466e20
	ctx.lr = 0x825FDB1C;
	sub_82466E20(ctx, base);
	// 825FDB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDB30 size=112
    let mut pc: u32 = 0x825FDB30;
    'dispatch: loop {
        match pc {
            0x825FDB30 => {
    //   block [0x825FDB30..0x825FDBA0)
	// 825FDB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDB3C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FDB40: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825FDB44: 38EA3F90  addi r7, r10, 0x3f90
	ctx.r[7].s64 = ctx.r[10].s64 + 16272;
	// 825FDB48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDB4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FDB50: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 825FDB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDB58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDB5C: 396B8DD8  addi r11, r11, -0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + -29224;
	// 825FDB60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDB64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDB68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDB6C: 386A13AC  addi r3, r10, 0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5036;
	// 825FDB70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDB74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FDB78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDB7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FDB80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDB84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDB88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDB8C: 4BE69295  bl 0x82466e20
	ctx.lr = 0x825FDB90;
	sub_82466E20(ctx, base);
	// 825FDB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDBA0 size=112
    let mut pc: u32 = 0x825FDBA0;
    'dispatch: loop {
        match pc {
            0x825FDBA0 => {
    //   block [0x825FDBA0..0x825FDC10)
	// 825FDBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDBAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDBB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDBB4: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FDBB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDBBC: 390B40C8  addi r8, r11, 0x40c8
	ctx.r[8].s64 = ctx.r[11].s64 + 16584;
	// 825FDBC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FDBC4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 825FDBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDBD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDBD8: 386A13DC  addi r3, r10, 0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5084;
	// 825FDBDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FDBE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDBFC: 4BE69225  bl 0x82466e20
	ctx.lr = 0x825FDC00;
	sub_82466E20(ctx, base);
	// 825FDC00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDC04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDC08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDC10 size=108
    let mut pc: u32 = 0x825FDC10;
    'dispatch: loop {
        match pc {
            0x825FDC10 => {
    //   block [0x825FDC10..0x825FDC7C)
	// 825FDC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDC1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDC24: 38EB40F8  addi r7, r11, 0x40f8
	ctx.r[7].s64 = ctx.r[11].s64 + 16632;
	// 825FDC28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FDC2C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 825FDC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDC40: 386A140C  addi r3, r10, 0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + 5132;
	// 825FDC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDC68: 4BE691B9  bl 0x82466e20
	ctx.lr = 0x825FDC6C;
	sub_82466E20(ctx, base);
	// 825FDC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDC80 size=108
    let mut pc: u32 = 0x825FDC80;
    'dispatch: loop {
        match pc {
            0x825FDC80 => {
    //   block [0x825FDC80..0x825FDCEC)
	// 825FDC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDC8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDC94: 38EB4158  addi r7, r11, 0x4158
	ctx.r[7].s64 = ctx.r[11].s64 + 16728;
	// 825FDC98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825FDC9C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 825FDCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDCA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDCB0: 386A143C  addi r3, r10, 0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + 5180;
	// 825FDCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDCD8: 4BE69149  bl 0x82466e20
	ctx.lr = 0x825FDCDC;
	sub_82466E20(ctx, base);
	// 825FDCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDCF0 size=108
    let mut pc: u32 = 0x825FDCF0;
    'dispatch: loop {
        match pc {
            0x825FDCF0 => {
    //   block [0x825FDCF0..0x825FDD5C)
	// 825FDCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDCFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDD04: 38EB4260  addi r7, r11, 0x4260
	ctx.r[7].s64 = ctx.r[11].s64 + 16992;
	// 825FDD08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FDD0C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 825FDD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDD18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDD20: 386A146C  addi r3, r10, 0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + 5228;
	// 825FDD24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDD48: 4BE690D9  bl 0x82466e20
	ctx.lr = 0x825FDD4C;
	sub_82466E20(ctx, base);
	// 825FDD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDD60 size=108
    let mut pc: u32 = 0x825FDD60;
    'dispatch: loop {
        match pc {
            0x825FDD60 => {
    //   block [0x825FDD60..0x825FDDCC)
	// 825FDD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDD6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDD74: 38EB4338  addi r7, r11, 0x4338
	ctx.r[7].s64 = ctx.r[11].s64 + 17208;
	// 825FDD78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FDD7C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 825FDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDD90: 386A149C  addi r3, r10, 0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + 5276;
	// 825FDD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDDB8: 4BE69069  bl 0x82466e20
	ctx.lr = 0x825FDDBC;
	sub_82466E20(ctx, base);
	// 825FDDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDDD0 size=108
    let mut pc: u32 = 0x825FDDD0;
    'dispatch: loop {
        match pc {
            0x825FDDD0 => {
    //   block [0x825FDDD0..0x825FDE3C)
	// 825FDDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDDDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDDE4: 38EB4380  addi r7, r11, 0x4380
	ctx.r[7].s64 = ctx.r[11].s64 + 17280;
	// 825FDDE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FDDEC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 825FDDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDE00: 386A14CC  addi r3, r10, 0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5324;
	// 825FDE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDE28: 4BE68FF9  bl 0x82466e20
	ctx.lr = 0x825FDE2C;
	sub_82466E20(ctx, base);
	// 825FDE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDE40 size=108
    let mut pc: u32 = 0x825FDE40;
    'dispatch: loop {
        match pc {
            0x825FDE40 => {
    //   block [0x825FDE40..0x825FDEAC)
	// 825FDE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDE4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDE50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDE54: 38EB4398  addi r7, r11, 0x4398
	ctx.r[7].s64 = ctx.r[11].s64 + 17304;
	// 825FDE58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FDE5C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 825FDE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDE68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDE70: 386A14FC  addi r3, r10, 0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5372;
	// 825FDE74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDE94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDE98: 4BE68F89  bl 0x82466e20
	ctx.lr = 0x825FDE9C;
	sub_82466E20(ctx, base);
	// 825FDE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDEB0 size=108
    let mut pc: u32 = 0x825FDEB0;
    'dispatch: loop {
        match pc {
            0x825FDEB0 => {
    //   block [0x825FDEB0..0x825FDF1C)
	// 825FDEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDEBC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDEC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDEC4: 38EB43F8  addi r7, r11, 0x43f8
	ctx.r[7].s64 = ctx.r[11].s64 + 17400;
	// 825FDEC8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FDECC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 825FDED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDEE0: 386A152C  addi r3, r10, 0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + 5420;
	// 825FDEE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDF08: 4BE68F19  bl 0x82466e20
	ctx.lr = 0x825FDF0C;
	sub_82466E20(ctx, base);
	// 825FDF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDF20 size=116
    let mut pc: u32 = 0x825FDF20;
    'dispatch: loop {
        match pc {
            0x825FDF20 => {
    //   block [0x825FDF20..0x825FDF94)
	// 825FDF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDF2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDF30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDF34: 390B44B8  addi r8, r11, 0x44b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17592;
	// 825FDF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDF3C: 392A8E54  addi r9, r10, -0x71ac
	ctx.r[9].s64 = ctx.r[10].s64 + -29100;
	// 825FDF40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDF44: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825FDF48: 38AA14FC  addi r5, r10, 0x14fc
	ctx.r[5].s64 = ctx.r[10].s64 + 5372;
	// 825FDF4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDF54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDF64: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FDF68: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 825FDF6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FDF70: 386B155C  addi r3, r11, 0x155c
	ctx.r[3].s64 = ctx.r[11].s64 + 5468;
	// 825FDF74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FDF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDF80: 4BE68EA1  bl 0x82466e20
	ctx.lr = 0x825FDF84;
	sub_82466E20(ctx, base);
	// 825FDF84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDF98 size=112
    let mut pc: u32 = 0x825FDF98;
    'dispatch: loop {
        match pc {
            0x825FDF98 => {
    //   block [0x825FDF98..0x825FE008)
	// 825FDF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDFA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDFAC: 38AA389C  addi r5, r10, 0x389c
	ctx.r[5].s64 = ctx.r[10].s64 + 14492;
	// 825FDFB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDFB4: 390B4548  addi r8, r11, 0x4548
	ctx.r[8].s64 = ctx.r[11].s64 + 17736;
	// 825FDFB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FDFBC: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 825FDFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDFC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDFC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDFD0: 386A158C  addi r3, r10, 0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + 5516;
	// 825FDFD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FDFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDFF4: 4BE68E2D  bl 0x82466e20
	ctx.lr = 0x825FDFF8;
	sub_82466E20(ctx, base);
	// 825FDFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE008 size=96
    let mut pc: u32 = 0x825FE008;
    'dispatch: loop {
        match pc {
            0x825FE008 => {
    //   block [0x825FE008..0x825FE068)
	// 825FE008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE014: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE01C: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 825FE020: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE028: 386A15BC  addi r3, r10, 0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5564;
	// 825FE02C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE034: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE048: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE050: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE054: 4BE68DCD  bl 0x82466e20
	ctx.lr = 0x825FE058;
	sub_82466E20(ctx, base);
	// 825FE058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE068 size=24
    let mut pc: u32 = 0x825FE068;
    'dispatch: loop {
        match pc {
            0x825FE068 => {
    //   block [0x825FE068..0x825FE080)
	// 825FE068: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE06C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE070: 394AAA10  addi r10, r10, -0x55f0
	ctx.r[10].s64 = ctx.r[10].s64 + -22000;
	// 825FE074: 816B45A8  lwz r11, 0x45a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17832 as u32) ) } as u64;
	// 825FE078: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FE07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE080 size=116
    let mut pc: u32 = 0x825FE080;
    'dispatch: loop {
        match pc {
            0x825FE080 => {
    //   block [0x825FE080..0x825FE0F4)
	// 825FE080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE08C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE090: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE094: 390BAA10  addi r8, r11, -0x55f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22000;
	// 825FE098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE09C: 392A8EA0  addi r9, r10, -0x7160
	ctx.r[9].s64 = ctx.r[10].s64 + -29024;
	// 825FE0A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE0A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FE0A8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE0AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE0B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE0BC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FE0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE0C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FE0C8: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 825FE0CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FE0D0: 386B15EC  addi r3, r11, 0x15ec
	ctx.r[3].s64 = ctx.r[11].s64 + 5612;
	// 825FE0D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FE0D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE0E0: 4BE68D41  bl 0x82466e20
	ctx.lr = 0x825FE0E4;
	sub_82466E20(ctx, base);
	// 825FE0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE0F8 size=104
    let mut pc: u32 = 0x825FE0F8;
    'dispatch: loop {
        match pc {
            0x825FE0F8 => {
    //   block [0x825FE0F8..0x825FE160)
	// 825FE0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE104: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE10C: 392A8ECC  addi r9, r10, -0x7134
	ctx.r[9].s64 = ctx.r[10].s64 + -28980;
	// 825FE110: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE118: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE11C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE12C: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 825FE130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE138: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE140: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE144: 386A161C  addi r3, r10, 0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + 5660;
	// 825FE148: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FE14C: 4BE68CD5  bl 0x82466e20
	ctx.lr = 0x825FE150;
	sub_82466E20(ctx, base);
	// 825FE150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE160 size=96
    let mut pc: u32 = 0x825FE160;
    'dispatch: loop {
        match pc {
            0x825FE160 => {
    //   block [0x825FE160..0x825FE1C0)
	// 825FE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE174: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 825FE178: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE180: 386A164C  addi r3, r10, 0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + 5708;
	// 825FE184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE18C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE1A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE1A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE1AC: 4BE68C75  bl 0x82466e20
	ctx.lr = 0x825FE1B0;
	sub_82466E20(ctx, base);
	// 825FE1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE1C0 size=96
    let mut pc: u32 = 0x825FE1C0;
    'dispatch: loop {
        match pc {
            0x825FE1C0 => {
    //   block [0x825FE1C0..0x825FE220)
	// 825FE1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE1CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE1D4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 825FE1D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE1E0: 386A167C  addi r3, r10, 0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + 5756;
	// 825FE1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE1EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE20C: 4BE68C15  bl 0x82466e20
	ctx.lr = 0x825FE210;
	sub_82466E20(ctx, base);
	// 825FE210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE220 size=100
    let mut pc: u32 = 0x825FE220;
    'dispatch: loop {
        match pc {
            0x825FE220 => {
    //   block [0x825FE220..0x825FE284)
	// 825FE220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE22C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE234: 38AA161C  addi r5, r10, 0x161c
	ctx.r[5].s64 = ctx.r[10].s64 + 5660;
	// 825FE238: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE240: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 825FE244: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE254: 386A16AC  addi r3, r10, 0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5804;
	// 825FE258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE25C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE260: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE270: 4BE68BB1  bl 0x82466e20
	ctx.lr = 0x825FE274;
	sub_82466E20(ctx, base);
	// 825FE274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE288 size=112
    let mut pc: u32 = 0x825FE288;
    'dispatch: loop {
        match pc {
            0x825FE288 => {
    //   block [0x825FE288..0x825FE2F8)
	// 825FE288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE294: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE298: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE29C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE2A4: 390B45B0  addi r8, r11, 0x45b0
	ctx.r[8].s64 = ctx.r[11].s64 + 17840;
	// 825FE2A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FE2AC: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 825FE2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE2B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE2C0: 386A16DC  addi r3, r10, 0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5852;
	// 825FE2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE2E4: 4BE68B3D  bl 0x82466e20
	ctx.lr = 0x825FE2E8;
	sub_82466E20(ctx, base);
	// 825FE2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE2F8 size=112
    let mut pc: u32 = 0x825FE2F8;
    'dispatch: loop {
        match pc {
            0x825FE2F8 => {
    //   block [0x825FE2F8..0x825FE368)
	// 825FE2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE308: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE30C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE314: 390B45F8  addi r8, r11, 0x45f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17912;
	// 825FE318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE31C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 825FE320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE330: 386A170C  addi r3, r10, 0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + 5900;
	// 825FE334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE354: 4BE68ACD  bl 0x82466e20
	ctx.lr = 0x825FE358;
	sub_82466E20(ctx, base);
	// 825FE358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE368 size=100
    let mut pc: u32 = 0x825FE368;
    'dispatch: loop {
        match pc {
            0x825FE368 => {
    //   block [0x825FE368..0x825FE3CC)
	// 825FE368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE37C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE388: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 825FE38C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE39C: 386A173C  addi r3, r10, 0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + 5948;
	// 825FE3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE3A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE3B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE3B8: 4BE68A69  bl 0x82466e20
	ctx.lr = 0x825FE3BC;
	sub_82466E20(ctx, base);
	// 825FE3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE3D0 size=112
    let mut pc: u32 = 0x825FE3D0;
    'dispatch: loop {
        match pc {
            0x825FE3D0 => {
    //   block [0x825FE3D0..0x825FE440)
	// 825FE3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE3E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE3E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE3EC: 390B4610  addi r8, r11, 0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + 17936;
	// 825FE3F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FE3F4: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 825FE3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE3FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE408: 386A176C  addi r3, r10, 0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + 5996;
	// 825FE40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE42C: 4BE689F5  bl 0x82466e20
	ctx.lr = 0x825FE430;
	sub_82466E20(ctx, base);
	// 825FE430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE440 size=96
    let mut pc: u32 = 0x825FE440;
    'dispatch: loop {
        match pc {
            0x825FE440 => {
    //   block [0x825FE440..0x825FE4A0)
	// 825FE440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE44C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE454: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 825FE458: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE460: 386A179C  addi r3, r10, 0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + 6044;
	// 825FE464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE46C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE48C: 4BE68995  bl 0x82466e20
	ctx.lr = 0x825FE490;
	sub_82466E20(ctx, base);
	// 825FE490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE4A0 size=112
    let mut pc: u32 = 0x825FE4A0;
    'dispatch: loop {
        match pc {
            0x825FE4A0 => {
    //   block [0x825FE4A0..0x825FE510)
	// 825FE4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE4AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE4B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE4B4: 38AA179C  addi r5, r10, 0x179c
	ctx.r[5].s64 = ctx.r[10].s64 + 6044;
	// 825FE4B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE4BC: 390B4640  addi r8, r11, 0x4640
	ctx.r[8].s64 = ctx.r[11].s64 + 17984;
	// 825FE4C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE4C4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 825FE4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE4CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE4D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE4D8: 386A17CC  addi r3, r10, 0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6092;
	// 825FE4DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE4FC: 4BE68925  bl 0x82466e20
	ctx.lr = 0x825FE500;
	sub_82466E20(ctx, base);
	// 825FE500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE510 size=112
    let mut pc: u32 = 0x825FE510;
    'dispatch: loop {
        match pc {
            0x825FE510 => {
    //   block [0x825FE510..0x825FE580)
	// 825FE510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE51C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE524: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FE528: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE52C: 390B4658  addi r8, r11, 0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + 18008;
	// 825FE530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE534: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 825FE538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE548: 386A17FC  addi r3, r10, 0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6140;
	// 825FE54C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE55C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE56C: 4BE688B5  bl 0x82466e20
	ctx.lr = 0x825FE570;
	sub_82466E20(ctx, base);
	// 825FE570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE580 size=112
    let mut pc: u32 = 0x825FE580;
    'dispatch: loop {
        match pc {
            0x825FE580 => {
    //   block [0x825FE580..0x825FE5F0)
	// 825FE580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE58C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE594: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE598: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE59C: 390B4670  addi r8, r11, 0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + 18032;
	// 825FE5A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FE5A4: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 825FE5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE5AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE5B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE5B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE5B8: 386A182C  addi r3, r10, 0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + 6188;
	// 825FE5BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE5C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE5C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE5D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE5D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE5D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE5DC: 4BE68845  bl 0x82466e20
	ctx.lr = 0x825FE5E0;
	sub_82466E20(ctx, base);
	// 825FE5E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE5F0 size=36
    let mut pc: u32 = 0x825FE5F0;
    'dispatch: loop {
        match pc {
            0x825FE5F0 => {
    //   block [0x825FE5F0..0x825FE614)
	// 825FE5F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE5F4: 814B46C0  lwz r10, 0x46c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18112 as u32) ) } as u64;
	// 825FE5F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE5FC: 396BAA40  addi r11, r11, -0x55c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21952;
	// 825FE600: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FE604: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FE608: 814A46B8  lwz r10, 0x46b8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(18104 as u32) ) } as u64;
	// 825FE60C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FE610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE618 size=108
    let mut pc: u32 = 0x825FE618;
    'dispatch: loop {
        match pc {
            0x825FE618 => {
    //   block [0x825FE618..0x825FE684)
	// 825FE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE624: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE62C: 38EBAA40  addi r7, r11, -0x55c0
	ctx.r[7].s64 = ctx.r[11].s64 + -21952;
	// 825FE630: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FE634: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 825FE638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE63C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE648: 386A185C  addi r3, r10, 0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + 6236;
	// 825FE64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE670: 4BE687B1  bl 0x82466e20
	ctx.lr = 0x825FE674;
	sub_82466E20(ctx, base);
	// 825FE674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE688 size=24
    let mut pc: u32 = 0x825FE688;
    'dispatch: loop {
        match pc {
            0x825FE688 => {
    //   block [0x825FE688..0x825FE6A0)
	// 825FE688: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE68C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE690: 394AAAE8  addi r10, r10, -0x5518
	ctx.r[10].s64 = ctx.r[10].s64 + -21784;
	// 825FE694: 816B46B8  lwz r11, 0x46b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18104 as u32) ) } as u64;
	// 825FE698: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 825FE69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE6A0 size=116
    let mut pc: u32 = 0x825FE6A0;
    'dispatch: loop {
        match pc {
            0x825FE6A0 => {
    //   block [0x825FE6A0..0x825FE714)
	// 825FE6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE6AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE6B0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825FE6B4: 390AAAE8  addi r8, r10, -0x5518
	ctx.r[8].s64 = ctx.r[10].s64 + -21784;
	// 825FE6B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE6BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FE6C0: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 825FE6C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE6C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FE6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE6D4: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 825FE6D8: 396B8F6C  addi r11, r11, -0x7094
	ctx.r[11].s64 = ctx.r[11].s64 + -28820;
	// 825FE6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE6E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE6E4: 386A188C  addi r3, r10, 0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + 6284;
	// 825FE6E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FE6EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE6F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FE6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE700: 4BE68721  bl 0x82466e20
	ctx.lr = 0x825FE704;
	sub_82466E20(ctx, base);
	// 825FE704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE718 size=112
    let mut pc: u32 = 0x825FE718;
    'dispatch: loop {
        match pc {
            0x825FE718 => {
    //   block [0x825FE718..0x825FE788)
	// 825FE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE72C: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 825FE730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE734: 390B46C8  addi r8, r11, 0x46c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18120;
	// 825FE738: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FE73C: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 825FE740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE750: 386A18BC  addi r3, r10, 0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6332;
	// 825FE754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE774: 4BE686AD  bl 0x82466e20
	ctx.lr = 0x825FE778;
	sub_82466E20(ctx, base);
	// 825FE778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE788 size=24
    let mut pc: u32 = 0x825FE788;
    'dispatch: loop {
        match pc {
            0x825FE788 => {
    //   block [0x825FE788..0x825FE7A0)
	// 825FE788: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE78C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE790: 394AABD8  addi r10, r10, -0x5428
	ctx.r[10].s64 = ctx.r[10].s64 + -21544;
	// 825FE794: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FE798: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 825FE79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE7A0 size=116
    let mut pc: u32 = 0x825FE7A0;
    'dispatch: loop {
        match pc {
            0x825FE7A0 => {
    //   block [0x825FE7A0..0x825FE814)
	// 825FE7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE7AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FE7B0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE7B4: 392B8F30  addi r9, r11, -0x70d0
	ctx.r[9].s64 = ctx.r[11].s64 + -28880;
	// 825FE7B8: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE7BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE7C0: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 825FE7C4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 825FE7C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE7CC: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 825FE7D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE7D4: 396BABD8  addi r11, r11, -0x5428
	ctx.r[11].s64 = ctx.r[11].s64 + -21544;
	// 825FE7D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FE7DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE7E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FE7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE7E8: 386A18EC  addi r3, r10, 0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6380;
	// 825FE7EC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FE7F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FE7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE7F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FE7FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE800: 4BE68621  bl 0x82466e20
	ctx.lr = 0x825FE804;
	sub_82466E20(ctx, base);
	// 825FE804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE818 size=100
    let mut pc: u32 = 0x825FE818;
    'dispatch: loop {
        match pc {
            0x825FE818 => {
    //   block [0x825FE818..0x825FE87C)
	// 825FE818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE82C: 38AA1A0C  addi r5, r10, 0x1a0c
	ctx.r[5].s64 = ctx.r[10].s64 + 6668;
	// 825FE830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE838: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 825FE83C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE84C: 386A191C  addi r3, r10, 0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + 6428;
	// 825FE850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE858: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE860: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE868: 4BE685B9  bl 0x82466e20
	ctx.lr = 0x825FE86C;
	sub_82466E20(ctx, base);
	// 825FE86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE880 size=108
    let mut pc: u32 = 0x825FE880;
    'dispatch: loop {
        match pc {
            0x825FE880 => {
    //   block [0x825FE880..0x825FE8EC)
	// 825FE880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE88C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE890: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE894: 38EB4740  addi r7, r11, 0x4740
	ctx.r[7].s64 = ctx.r[11].s64 + 18240;
	// 825FE898: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FE89C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 825FE8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE8A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE8B0: 386A194C  addi r3, r10, 0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + 6476;
	// 825FE8B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE8D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE8D8: 4BE68549  bl 0x82466e20
	ctx.lr = 0x825FE8DC;
	sub_82466E20(ctx, base);
	// 825FE8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE8F0 size=112
    let mut pc: u32 = 0x825FE8F0;
    'dispatch: loop {
        match pc {
            0x825FE8F0 => {
    //   block [0x825FE8F0..0x825FE960)
	// 825FE8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE8FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE900: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE904: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE90C: 390B47A0  addi r8, r11, 0x47a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18336;
	// 825FE910: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FE914: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 825FE918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE91C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE928: 386A197C  addi r3, r10, 0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + 6524;
	// 825FE92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE94C: 4BE684D5  bl 0x82466e20
	ctx.lr = 0x825FE950;
	sub_82466E20(ctx, base);
	// 825FE950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE960 size=108
    let mut pc: u32 = 0x825FE960;
    'dispatch: loop {
        match pc {
            0x825FE960 => {
    //   block [0x825FE960..0x825FE9CC)
	// 825FE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE96C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE974: 38EB4800  addi r7, r11, 0x4800
	ctx.r[7].s64 = ctx.r[11].s64 + 18432;
	// 825FE978: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE97C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 825FE980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE990: 386A19AC  addi r3, r10, 0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6572;
	// 825FE994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE9B8: 4BE68469  bl 0x82466e20
	ctx.lr = 0x825FE9BC;
	sub_82466E20(ctx, base);
	// 825FE9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE9D0 size=28
    let mut pc: u32 = 0x825FE9D0;
    'dispatch: loop {
        match pc {
            0x825FE9D0 => {
    //   block [0x825FE9D0..0x825FE9EC)
	// 825FE9D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE9D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE9D8: 394AACE0  addi r10, r10, -0x5320
	ctx.r[10].s64 = ctx.r[10].s64 + -21280;
	// 825FE9DC: 816B46C4  lwz r11, 0x46c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18116 as u32) ) } as u64;
	// 825FE9E0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FE9E4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825FE9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE9F0 size=112
    let mut pc: u32 = 0x825FE9F0;
    'dispatch: loop {
        match pc {
            0x825FE9F0 => {
    //   block [0x825FE9F0..0x825FEA60)
	// 825FE9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE9FC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEA00: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 825FEA04: 38EAACE0  addi r7, r10, -0x5320
	ctx.r[7].s64 = ctx.r[10].s64 + -21280;
	// 825FEA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEA0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEA10: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 825FEA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEA18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FEA1C: 396B9030  addi r11, r11, -0x6fd0
	ctx.r[11].s64 = ctx.r[11].s64 + -28624;
	// 825FEA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FEA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEA28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEA2C: 386A19DC  addi r3, r10, 0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6620;
	// 825FEA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEA34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEA38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEA3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEA40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEA44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEA48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FEA4C: 4BE683D5  bl 0x82466e20
	ctx.lr = 0x825FEA50;
	sub_82466E20(ctx, base);
	// 825FEA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEA60 size=24
    let mut pc: u32 = 0x825FEA60;
    'dispatch: loop {
        match pc {
            0x825FEA60 => {
    //   block [0x825FEA60..0x825FEA78)
	// 825FEA60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEA64: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEA68: 394AAE48  addi r10, r10, -0x51b8
	ctx.r[10].s64 = ctx.r[10].s64 + -20920;
	// 825FEA6C: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEA70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEA78 size=116
    let mut pc: u32 = 0x825FEA78;
    'dispatch: loop {
        match pc {
            0x825FEA78 => {
    //   block [0x825FEA78..0x825FEAEC)
	// 825FEA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEA84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEA88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEA8C: 392B9008  addi r9, r11, -0x6ff8
	ctx.r[9].s64 = ctx.r[11].s64 + -28664;
	// 825FEA90: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FEA94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEA98: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 825FEA9C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825FEAA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FEAA4: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 825FEAA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEAAC: 396BAE48  addi r11, r11, -0x51b8
	ctx.r[11].s64 = ctx.r[11].s64 + -20920;
	// 825FEAB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FEAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEAB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FEABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEAC0: 386A1A0C  addi r3, r10, 0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 6668;
	// 825FEAC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FEAC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FEACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEAD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FEAD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FEAD8: 4BE68349  bl 0x82466e20
	ctx.lr = 0x825FEADC;
	sub_82466E20(ctx, base);
	// 825FEADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEAF0 size=108
    let mut pc: u32 = 0x825FEAF0;
    'dispatch: loop {
        match pc {
            0x825FEAF0 => {
    //   block [0x825FEAF0..0x825FEB5C)
	// 825FEAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEAFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEB04: 38EB4820  addi r7, r11, 0x4820
	ctx.r[7].s64 = ctx.r[11].s64 + 18464;
	// 825FEB08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FEB0C: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 825FEB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEB14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FEB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEB20: 386A1A3C  addi r3, r10, 0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 6716;
	// 825FEB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FEB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FEB48: 4BE682D9  bl 0x82466e20
	ctx.lr = 0x825FEB4C;
	sub_82466E20(ctx, base);
	// 825FEB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEB60 size=24
    let mut pc: u32 = 0x825FEB60;
    'dispatch: loop {
        match pc {
            0x825FEB60 => {
    //   block [0x825FEB60..0x825FEB78)
	// 825FEB60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEB64: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEB68: 394AAEF0  addi r10, r10, -0x5110
	ctx.r[10].s64 = ctx.r[10].s64 + -20752;
	// 825FEB6C: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEB70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEB78 size=116
    let mut pc: u32 = 0x825FEB78;
    'dispatch: loop {
        match pc {
            0x825FEB78 => {
    //   block [0x825FEB78..0x825FEBEC)
	// 825FEB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEB84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEB88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FEB8C: 390AAEF0  addi r8, r10, -0x5110
	ctx.r[8].s64 = ctx.r[10].s64 + -20752;
	// 825FEB90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEB94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEB98: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FEB9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEBA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FEBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEBA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEBAC: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 825FEBB0: 396B9090  addi r11, r11, -0x6f70
	ctx.r[11].s64 = ctx.r[11].s64 + -28528;
	// 825FEBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEBB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEBBC: 386A1A6C  addi r3, r10, 0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 6764;
	// 825FEBC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEBC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEBC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEBD8: 4BE68249  bl 0x82466e20
	ctx.lr = 0x825FEBDC;
	sub_82466E20(ctx, base);
	// 825FEBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEBF0 size=112
    let mut pc: u32 = 0x825FEBF0;
    'dispatch: loop {
        match pc {
            0x825FEBF0 => {
    //   block [0x825FEBF0..0x825FEC60)
	// 825FEBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEBFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC00: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEC04: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FEC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEC0C: 390B4880  addi r8, r11, 0x4880
	ctx.r[8].s64 = ctx.r[11].s64 + 18560;
	// 825FEC10: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FEC14: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 825FEC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEC1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEC28: 386A1A9C  addi r3, r10, 0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 6812;
	// 825FEC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEC4C: 4BE681D5  bl 0x82466e20
	ctx.lr = 0x825FEC50;
	sub_82466E20(ctx, base);
	// 825FEC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEC60 size=112
    let mut pc: u32 = 0x825FEC60;
    'dispatch: loop {
        match pc {
            0x825FEC60 => {
    //   block [0x825FEC60..0x825FECD0)
	// 825FEC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEC6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEC74: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FEC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEC7C: 390B4910  addi r8, r11, 0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + 18704;
	// 825FEC80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FEC84: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 825FEC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEC98: 386A1ACC  addi r3, r10, 0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + 6860;
	// 825FEC9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FECA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FECA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FECAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FECB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FECB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FECB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FECBC: 4BE68165  bl 0x82466e20
	ctx.lr = 0x825FECC0;
	sub_82466E20(ctx, base);
	// 825FECC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FECC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FECC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FECCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FECD0 size=112
    let mut pc: u32 = 0x825FECD0;
    'dispatch: loop {
        match pc {
            0x825FECD0 => {
    //   block [0x825FECD0..0x825FED40)
	// 825FECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FECD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FECDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FECE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FECE4: 38AA18EC  addi r5, r10, 0x18ec
	ctx.r[5].s64 = ctx.r[10].s64 + 6380;
	// 825FECE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FECEC: 390B4970  addi r8, r11, 0x4970
	ctx.r[8].s64 = ctx.r[11].s64 + 18800;
	// 825FECF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FECF4: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 825FECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FECFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FED04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FED08: 386A1AFC  addi r3, r10, 0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + 6908;
	// 825FED0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FED10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FED14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FED18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FED1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FED20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FED24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FED28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FED2C: 4BE680F5  bl 0x82466e20
	ctx.lr = 0x825FED30;
	sub_82466E20(ctx, base);
	// 825FED30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FED40 size=112
    let mut pc: u32 = 0x825FED40;
    'dispatch: loop {
        match pc {
            0x825FED40 => {
    //   block [0x825FED40..0x825FEDB0)
	// 825FED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FED48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FED4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FED54: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FED58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FED5C: 390B49A0  addi r8, r11, 0x49a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18848;
	// 825FED60: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FED64: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 825FED68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FED6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FED74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FED78: 386A1B2C  addi r3, r10, 0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6956;
	// 825FED7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FED80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FED84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FED88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FED8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FED90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FED94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FED98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FED9C: 4BE68085  bl 0x82466e20
	ctx.lr = 0x825FEDA0;
	sub_82466E20(ctx, base);
	// 825FEDA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEDB0 size=112
    let mut pc: u32 = 0x825FEDB0;
    'dispatch: loop {
        match pc {
            0x825FEDB0 => {
    //   block [0x825FEDB0..0x825FEE20)
	// 825FEDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEDBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEDC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEDC4: 38AA1A0C  addi r5, r10, 0x1a0c
	ctx.r[5].s64 = ctx.r[10].s64 + 6668;
	// 825FEDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEDCC: 390B4A30  addi r8, r11, 0x4a30
	ctx.r[8].s64 = ctx.r[11].s64 + 18992;
	// 825FEDD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FEDD4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 825FEDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEDE8: 386A1B5C  addi r3, r10, 0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7004;
	// 825FEDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEE0C: 4BE68015  bl 0x82466e20
	ctx.lr = 0x825FEE10;
	sub_82466E20(ctx, base);
	// 825FEE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


