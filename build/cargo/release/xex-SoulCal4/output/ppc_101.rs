pub fn sub_8263CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CD08 size=112
    let mut pc: u32 = 0x8263CD08;
    'dispatch: loop {
        match pc {
            0x8263CD08 => {
    //   block [0x8263CD08..0x8263CD78)
	// 8263CD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CD14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CD1C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CD24: 390B70C0  addi r8, r11, 0x70c0
	ctx.r[8].s64 = ctx.r[11].s64 + 28864;
	// 8263CD28: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263CD2C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8263CD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CD34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CD40: 386ABE0C  addi r3, r10, -0x41f4
	ctx.r[3].s64 = ctx.r[10].s64 + -16884;
	// 8263CD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CD64: 4BE2A0BD  bl 0x82466e20
	ctx.lr = 0x8263CD68;
	sub_82466E20(ctx, base);
	// 8263CD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CD78 size=116
    let mut pc: u32 = 0x8263CD78;
    'dispatch: loop {
        match pc {
            0x8263CD78 => {
    //   block [0x8263CD78..0x8263CDEC)
	// 8263CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CD84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CD88: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD8C: 392B6838  addi r9, r11, 0x6838
	ctx.r[9].s64 = ctx.r[11].s64 + 26680;
	// 8263CD90: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CD94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CD98: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8263CD9C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8263CDA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CDA4: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8263CDA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CDAC: 396B7138  addi r11, r11, 0x7138
	ctx.r[11].s64 = ctx.r[11].s64 + 28984;
	// 8263CDB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263CDB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CDB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CDC0: 386ABE3C  addi r3, r10, -0x41c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	// 8263CDC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263CDC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263CDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CDD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263CDD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263CDD8: 4BE2A049  bl 0x82466e20
	ctx.lr = 0x8263CDDC;
	sub_82466E20(ctx, base);
	// 8263CDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CDF0 size=36
    let mut pc: u32 = 0x8263CDF0;
    'dispatch: loop {
        match pc {
            0x8263CDF0 => {
    //   block [0x8263CDF0..0x8263CE14)
	// 8263CDF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CDF4: 814B71D0  lwz r10, 0x71d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29136 as u32) ) } as u64;
	// 8263CDF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CDFC: 396BB638  addi r11, r11, -0x49c8
	ctx.r[11].s64 = ctx.r[11].s64 + -18888;
	// 8263CE00: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8263CE04: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263CE08: 814A71C8  lwz r10, 0x71c8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29128 as u32) ) } as u64;
	// 8263CE0C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8263CE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CE18 size=108
    let mut pc: u32 = 0x8263CE18;
    'dispatch: loop {
        match pc {
            0x8263CE18 => {
    //   block [0x8263CE18..0x8263CE84)
	// 8263CE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CE24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CE2C: 38EBB638  addi r7, r11, -0x49c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18888;
	// 8263CE30: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263CE34: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8263CE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CE3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263CE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CE48: 386ABE6C  addi r3, r10, -0x4194
	ctx.r[3].s64 = ctx.r[10].s64 + -16788;
	// 8263CE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263CE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263CE70: 4BE29FB1  bl 0x82466e20
	ctx.lr = 0x8263CE74;
	sub_82466E20(ctx, base);
	// 8263CE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CE88 size=24
    let mut pc: u32 = 0x8263CE88;
    'dispatch: loop {
        match pc {
            0x8263CE88 => {
    //   block [0x8263CE88..0x8263CEA0)
	// 8263CE88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CE8C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CE90: 394AB6E0  addi r10, r10, -0x4920
	ctx.r[10].s64 = ctx.r[10].s64 + -18720;
	// 8263CE94: 816B71C8  lwz r11, 0x71c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29128 as u32) ) } as u64;
	// 8263CE98: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263CE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CEA0 size=116
    let mut pc: u32 = 0x8263CEA0;
    'dispatch: loop {
        match pc {
            0x8263CEA0 => {
    //   block [0x8263CEA0..0x8263CF14)
	// 8263CEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CEAC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CEB0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8263CEB4: 390AB6E0  addi r8, r10, -0x4920
	ctx.r[8].s64 = ctx.r[10].s64 + -18720;
	// 8263CEB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CEBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CEC0: 38AABE6C  addi r5, r10, -0x4194
	ctx.r[5].s64 = ctx.r[10].s64 + -16788;
	// 8263CEC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CEC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263CECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CED4: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8263CED8: 396B68F4  addi r11, r11, 0x68f4
	ctx.r[11].s64 = ctx.r[11].s64 + 26868;
	// 8263CEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CEE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CEE4: 386ABE9C  addi r3, r10, -0x4164
	ctx.r[3].s64 = ctx.r[10].s64 + -16740;
	// 8263CEE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263CEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CEF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263CEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CF00: 4BE29F21  bl 0x82466e20
	ctx.lr = 0x8263CF04;
	sub_82466E20(ctx, base);
	// 8263CF04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CF18 size=112
    let mut pc: u32 = 0x8263CF18;
    'dispatch: loop {
        match pc {
            0x8263CF18 => {
    //   block [0x8263CF18..0x8263CF88)
	// 8263CF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CF24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CF28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CF2C: 38AABE6C  addi r5, r10, -0x4194
	ctx.r[5].s64 = ctx.r[10].s64 + -16788;
	// 8263CF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CF34: 390B71D8  addi r8, r11, 0x71d8
	ctx.r[8].s64 = ctx.r[11].s64 + 29144;
	// 8263CF38: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263CF3C: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8263CF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CF44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CF48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CF50: 386ABECC  addi r3, r10, -0x4134
	ctx.r[3].s64 = ctx.r[10].s64 + -16692;
	// 8263CF54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CF5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CF74: 4BE29EAD  bl 0x82466e20
	ctx.lr = 0x8263CF78;
	sub_82466E20(ctx, base);
	// 8263CF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CF88 size=24
    let mut pc: u32 = 0x8263CF88;
    'dispatch: loop {
        match pc {
            0x8263CF88 => {
    //   block [0x8263CF88..0x8263CFA0)
	// 8263CF88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CF8C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CF90: 394AB7A0  addi r10, r10, -0x4860
	ctx.r[10].s64 = ctx.r[10].s64 + -18528;
	// 8263CF94: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263CF98: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CFA0 size=116
    let mut pc: u32 = 0x8263CFA0;
    'dispatch: loop {
        match pc {
            0x8263CFA0 => {
    //   block [0x8263CFA0..0x8263D014)
	// 8263CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CFAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CFB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CFB4: 392B68B8  addi r9, r11, 0x68b8
	ctx.r[9].s64 = ctx.r[11].s64 + 26808;
	// 8263CFB8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263CFBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CFC0: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8263CFC4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263CFC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CFCC: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8263CFD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CFD4: 396BB7A0  addi r11, r11, -0x4860
	ctx.r[11].s64 = ctx.r[11].s64 + -18528;
	// 8263CFD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263CFDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CFE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263CFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CFE8: 386ABEFC  addi r3, r10, -0x4104
	ctx.r[3].s64 = ctx.r[10].s64 + -16644;
	// 8263CFEC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263CFF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263CFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CFF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263CFFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D000: 4BE29E21  bl 0x82466e20
	ctx.lr = 0x8263D004;
	sub_82466E20(ctx, base);
	// 8263D004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D018 size=100
    let mut pc: u32 = 0x8263D018;
    'dispatch: loop {
        match pc {
            0x8263D018 => {
    //   block [0x8263D018..0x8263D07C)
	// 8263D018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D02C: 38AAC04C  addi r5, r10, -0x3fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -16308;
	// 8263D030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D038: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8263D03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D04C: 386ABF2C  addi r3, r10, -0x40d4
	ctx.r[3].s64 = ctx.r[10].s64 + -16596;
	// 8263D050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D058: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D068: 4BE29DB9  bl 0x82466e20
	ctx.lr = 0x8263D06C;
	sub_82466E20(ctx, base);
	// 8263D06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D080 size=100
    let mut pc: u32 = 0x8263D080;
    'dispatch: loop {
        match pc {
            0x8263D080 => {
    //   block [0x8263D080..0x8263D0E4)
	// 8263D080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D08C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D094: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D0A0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8263D0A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D0B4: 386ABF5C  addi r3, r10, -0x40a4
	ctx.r[3].s64 = ctx.r[10].s64 + -16548;
	// 8263D0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D0BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D0C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D0C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D0D0: 4BE29D51  bl 0x82466e20
	ctx.lr = 0x8263D0D4;
	sub_82466E20(ctx, base);
	// 8263D0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D0E8 size=108
    let mut pc: u32 = 0x8263D0E8;
    'dispatch: loop {
        match pc {
            0x8263D0E8 => {
    //   block [0x8263D0E8..0x8263D154)
	// 8263D0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D0F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D0FC: 38EB7238  addi r7, r11, 0x7238
	ctx.r[7].s64 = ctx.r[11].s64 + 29240;
	// 8263D100: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263D104: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8263D108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D10C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D118: 386ABF8C  addi r3, r10, -0x4074
	ctx.r[3].s64 = ctx.r[10].s64 + -16500;
	// 8263D11C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D140: 4BE29CE1  bl 0x82466e20
	ctx.lr = 0x8263D144;
	sub_82466E20(ctx, base);
	// 8263D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D158 size=112
    let mut pc: u32 = 0x8263D158;
    'dispatch: loop {
        match pc {
            0x8263D158 => {
    //   block [0x8263D158..0x8263D1C8)
	// 8263D158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D168: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D16C: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D174: 390B7298  addi r8, r11, 0x7298
	ctx.r[8].s64 = ctx.r[11].s64 + 29336;
	// 8263D178: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263D17C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8263D180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D190: 386ABFBC  addi r3, r10, -0x4044
	ctx.r[3].s64 = ctx.r[10].s64 + -16452;
	// 8263D194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D1B4: 4BE29C6D  bl 0x82466e20
	ctx.lr = 0x8263D1B8;
	sub_82466E20(ctx, base);
	// 8263D1B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D1C8 size=108
    let mut pc: u32 = 0x8263D1C8;
    'dispatch: loop {
        match pc {
            0x8263D1C8 => {
    //   block [0x8263D1C8..0x8263D234)
	// 8263D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D1D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D1D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D1DC: 38EB72E0  addi r7, r11, 0x72e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29408;
	// 8263D1E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D1E4: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8263D1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D1EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D1F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D1F8: 386ABFEC  addi r3, r10, -0x4014
	ctx.r[3].s64 = ctx.r[10].s64 + -16404;
	// 8263D1FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D220: 4BE29C01  bl 0x82466e20
	ctx.lr = 0x8263D224;
	sub_82466E20(ctx, base);
	// 8263D224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D238 size=28
    let mut pc: u32 = 0x8263D238;
    'dispatch: loop {
        match pc {
            0x8263D238 => {
    //   block [0x8263D238..0x8263D254)
	// 8263D238: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D23C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D240: 394AB848  addi r10, r10, -0x47b8
	ctx.r[10].s64 = ctx.r[10].s64 + -18360;
	// 8263D244: 816B71D4  lwz r11, 0x71d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29140 as u32) ) } as u64;
	// 8263D248: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D24C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263D250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D258 size=112
    let mut pc: u32 = 0x8263D258;
    'dispatch: loop {
        match pc {
            0x8263D258 => {
    //   block [0x8263D258..0x8263D2C8)
	// 8263D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D264: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D268: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8263D26C: 38EAB848  addi r7, r10, -0x47b8
	ctx.r[7].s64 = ctx.r[10].s64 + -18360;
	// 8263D270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D274: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D278: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8263D27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D280: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D284: 396B69A0  addi r11, r11, 0x69a0
	ctx.r[11].s64 = ctx.r[11].s64 + 27040;
	// 8263D288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D28C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D294: 386AC01C  addi r3, r10, -0x3fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -16356;
	// 8263D298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D29C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D2A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D2A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D2A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D2AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D2B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D2B4: 4BE29B6D  bl 0x82466e20
	ctx.lr = 0x8263D2B8;
	sub_82466E20(ctx, base);
	// 8263D2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D2C8 size=24
    let mut pc: u32 = 0x8263D2C8;
    'dispatch: loop {
        match pc {
            0x8263D2C8 => {
    //   block [0x8263D2C8..0x8263D2E0)
	// 8263D2C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D2CC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D2D0: 394AB998  addi r10, r10, -0x4668
	ctx.r[10].s64 = ctx.r[10].s64 + -18024;
	// 8263D2D4: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D2D8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D2E0 size=116
    let mut pc: u32 = 0x8263D2E0;
    'dispatch: loop {
        match pc {
            0x8263D2E0 => {
    //   block [0x8263D2E0..0x8263D354)
	// 8263D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D2EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D2F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D2F4: 392B6978  addi r9, r11, 0x6978
	ctx.r[9].s64 = ctx.r[11].s64 + 27000;
	// 8263D2F8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D2FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D300: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8263D304: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263D308: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263D30C: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8263D310: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D314: 396BB998  addi r11, r11, -0x4668
	ctx.r[11].s64 = ctx.r[11].s64 + -18024;
	// 8263D318: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263D31C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D320: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263D324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D328: 386AC04C  addi r3, r10, -0x3fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -16308;
	// 8263D32C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263D330: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D338: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263D33C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D340: 4BE29AE1  bl 0x82466e20
	ctx.lr = 0x8263D344;
	sub_82466E20(ctx, base);
	// 8263D344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D358 size=112
    let mut pc: u32 = 0x8263D358;
    'dispatch: loop {
        match pc {
            0x8263D358 => {
    //   block [0x8263D358..0x8263D3C8)
	// 8263D358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D368: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D36C: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263D370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D374: 390B72FC  addi r8, r11, 0x72fc
	ctx.r[8].s64 = ctx.r[11].s64 + 29436;
	// 8263D378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D37C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8263D380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D390: 386AC07C  addi r3, r10, -0x3f84
	ctx.r[3].s64 = ctx.r[10].s64 + -16260;
	// 8263D394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D3B4: 4BE29A6D  bl 0x82466e20
	ctx.lr = 0x8263D3B8;
	sub_82466E20(ctx, base);
	// 8263D3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D3C8 size=24
    let mut pc: u32 = 0x8263D3C8;
    'dispatch: loop {
        match pc {
            0x8263D3C8 => {
    //   block [0x8263D3C8..0x8263D3E0)
	// 8263D3C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D3CC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D3D0: 394ABA40  addi r10, r10, -0x45c0
	ctx.r[10].s64 = ctx.r[10].s64 + -17856;
	// 8263D3D4: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D3D8: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8263D3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D3E0 size=116
    let mut pc: u32 = 0x8263D3E0;
    'dispatch: loop {
        match pc {
            0x8263D3E0 => {
    //   block [0x8263D3E0..0x8263D454)
	// 8263D3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D3EC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D3F0: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8263D3F4: 390ABA40  addi r8, r10, -0x45c0
	ctx.r[8].s64 = ctx.r[10].s64 + -17856;
	// 8263D3F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D3FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D400: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263D404: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D408: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263D40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D414: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8263D418: 396B69FC  addi r11, r11, 0x69fc
	ctx.r[11].s64 = ctx.r[11].s64 + 27132;
	// 8263D41C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D420: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D424: 386AC0AC  addi r3, r10, -0x3f54
	ctx.r[3].s64 = ctx.r[10].s64 + -16212;
	// 8263D428: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D42C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D430: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D440: 4BE299E1  bl 0x82466e20
	ctx.lr = 0x8263D444;
	sub_82466E20(ctx, base);
	// 8263D444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D458 size=112
    let mut pc: u32 = 0x8263D458;
    'dispatch: loop {
        match pc {
            0x8263D458 => {
    //   block [0x8263D458..0x8263D4C8)
	// 8263D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D468: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D46C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263D470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D474: 390B732C  addi r8, r11, 0x732c
	ctx.r[8].s64 = ctx.r[11].s64 + 29484;
	// 8263D478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D47C: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8263D480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D490: 386AC0DC  addi r3, r10, -0x3f24
	ctx.r[3].s64 = ctx.r[10].s64 + -16164;
	// 8263D494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D4A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D4B4: 4BE2996D  bl 0x82466e20
	ctx.lr = 0x8263D4B8;
	sub_82466E20(ctx, base);
	// 8263D4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D4C8 size=108
    let mut pc: u32 = 0x8263D4C8;
    'dispatch: loop {
        match pc {
            0x8263D4C8 => {
    //   block [0x8263D4C8..0x8263D534)
	// 8263D4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D4D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D4D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D4DC: 38EB735C  addi r7, r11, 0x735c
	ctx.r[7].s64 = ctx.r[11].s64 + 29532;
	// 8263D4E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263D4E4: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8263D4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D4EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D4F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D4F8: 386AC10C  addi r3, r10, -0x3ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -16116;
	// 8263D4FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D51C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D520: 4BE29901  bl 0x82466e20
	ctx.lr = 0x8263D524;
	sub_82466E20(ctx, base);
	// 8263D524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D538 size=112
    let mut pc: u32 = 0x8263D538;
    'dispatch: loop {
        match pc {
            0x8263D538 => {
    //   block [0x8263D538..0x8263D5A8)
	// 8263D538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D544: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D548: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D54C: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D554: 390B738C  addi r8, r11, 0x738c
	ctx.r[8].s64 = ctx.r[11].s64 + 29580;
	// 8263D558: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D55C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8263D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D570: 386AC13C  addi r3, r10, -0x3ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -16068;
	// 8263D574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D594: 4BE2988D  bl 0x82466e20
	ctx.lr = 0x8263D598;
	sub_82466E20(ctx, base);
	// 8263D598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D5A8 size=112
    let mut pc: u32 = 0x8263D5A8;
    'dispatch: loop {
        match pc {
            0x8263D5A8 => {
    //   block [0x8263D5A8..0x8263D618)
	// 8263D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D5B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D5B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D5BC: 38AAC31C  addi r5, r10, -0x3ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -15588;
	// 8263D5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D5C4: 390B73BC  addi r8, r11, 0x73bc
	ctx.r[8].s64 = ctx.r[11].s64 + 29628;
	// 8263D5C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D5CC: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8263D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D5D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D5E0: 386AC16C  addi r3, r10, -0x3e94
	ctx.r[3].s64 = ctx.r[10].s64 + -16020;
	// 8263D5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D604: 4BE2981D  bl 0x82466e20
	ctx.lr = 0x8263D608;
	sub_82466E20(ctx, base);
	// 8263D608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D618 size=108
    let mut pc: u32 = 0x8263D618;
    'dispatch: loop {
        match pc {
            0x8263D618 => {
    //   block [0x8263D618..0x8263D684)
	// 8263D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D624: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D62C: 38EB73F0  addi r7, r11, 0x73f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29680;
	// 8263D630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263D634: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8263D638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D63C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D648: 386AC19C  addi r3, r10, -0x3e64
	ctx.r[3].s64 = ctx.r[10].s64 + -15972;
	// 8263D64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D670: 4BE297B1  bl 0x82466e20
	ctx.lr = 0x8263D674;
	sub_82466E20(ctx, base);
	// 8263D674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D688 size=112
    let mut pc: u32 = 0x8263D688;
    'dispatch: loop {
        match pc {
            0x8263D688 => {
    //   block [0x8263D688..0x8263D6F8)
	// 8263D688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D69C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263D6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D6A4: 390B7438  addi r8, r11, 0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + 29752;
	// 8263D6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263D6AC: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8263D6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D6B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D6C0: 386AC1CC  addi r3, r10, -0x3e34
	ctx.r[3].s64 = ctx.r[10].s64 + -15924;
	// 8263D6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D6E4: 4BE2973D  bl 0x82466e20
	ctx.lr = 0x8263D6E8;
	sub_82466E20(ctx, base);
	// 8263D6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D6F8 size=100
    let mut pc: u32 = 0x8263D6F8;
    'dispatch: loop {
        match pc {
            0x8263D6F8 => {
    //   block [0x8263D6F8..0x8263D75C)
	// 8263D6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D70C: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D718: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8263D71C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D72C: 386AC1FC  addi r3, r10, -0x3e04
	ctx.r[3].s64 = ctx.r[10].s64 + -15876;
	// 8263D730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D748: 4BE296D9  bl 0x82466e20
	ctx.lr = 0x8263D74C;
	sub_82466E20(ctx, base);
	// 8263D74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D760 size=112
    let mut pc: u32 = 0x8263D760;
    'dispatch: loop {
        match pc {
            0x8263D760 => {
    //   block [0x8263D760..0x8263D7D0)
	// 8263D760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D76C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D770: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D774: 38AABF5C  addi r5, r10, -0x40a4
	ctx.r[5].s64 = ctx.r[10].s64 + -16548;
	// 8263D778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D77C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 8263D780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263D784: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8263D788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D78C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D798: 386AC22C  addi r3, r10, -0x3dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -15828;
	// 8263D79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D7BC: 4BE29665  bl 0x82466e20
	ctx.lr = 0x8263D7C0;
	sub_82466E20(ctx, base);
	// 8263D7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D7D0 size=112
    let mut pc: u32 = 0x8263D7D0;
    'dispatch: loop {
        match pc {
            0x8263D7D0 => {
    //   block [0x8263D7D0..0x8263D840)
	// 8263D7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D7DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D7E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D7E4: 38AABF5C  addi r5, r10, -0x40a4
	ctx.r[5].s64 = ctx.r[10].s64 + -16548;
	// 8263D7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D7EC: 390B74E0  addi r8, r11, 0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29920;
	// 8263D7F0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263D7F4: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8263D7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D7FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D808: 386AC25C  addi r3, r10, -0x3da4
	ctx.r[3].s64 = ctx.r[10].s64 + -15780;
	// 8263D80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D82C: 4BE295F5  bl 0x82466e20
	ctx.lr = 0x8263D830;
	sub_82466E20(ctx, base);
	// 8263D830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D840 size=108
    let mut pc: u32 = 0x8263D840;
    'dispatch: loop {
        match pc {
            0x8263D840 => {
    //   block [0x8263D840..0x8263D8AC)
	// 8263D840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D84C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D854: 38EB7588  addi r7, r11, 0x7588
	ctx.r[7].s64 = ctx.r[11].s64 + 30088;
	// 8263D858: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263D85C: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8263D860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D870: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 8263D874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D898: 4BE29589  bl 0x82466e20
	ctx.lr = 0x8263D89C;
	sub_82466E20(ctx, base);
	// 8263D89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D8B0 size=24
    let mut pc: u32 = 0x8263D8B0;
    'dispatch: loop {
        match pc {
            0x8263D8B0 => {
    //   block [0x8263D8B0..0x8263D8C8)
	// 8263D8B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D8B4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D8B8: 394ABB78  addi r10, r10, -0x4488
	ctx.r[10].s64 = ctx.r[10].s64 + -17544;
	// 8263D8BC: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D8C0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D8C8 size=116
    let mut pc: u32 = 0x8263D8C8;
    'dispatch: loop {
        match pc {
            0x8263D8C8 => {
    //   block [0x8263D8C8..0x8263D93C)
	// 8263D8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D8D4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D8D8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263D8DC: 390ABB78  addi r8, r10, -0x4488
	ctx.r[8].s64 = ctx.r[10].s64 + -17544;
	// 8263D8E0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D8E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D8E8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D8EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D8F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263D8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D8FC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8263D900: 396B6A34  addi r11, r11, 0x6a34
	ctx.r[11].s64 = ctx.r[11].s64 + 27188;
	// 8263D904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D90C: 386AC2BC  addi r3, r10, -0x3d44
	ctx.r[3].s64 = ctx.r[10].s64 + -15684;
	// 8263D910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D928: 4BE294F9  bl 0x82466e20
	ctx.lr = 0x8263D92C;
	sub_82466E20(ctx, base);
	// 8263D92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D940 size=100
    let mut pc: u32 = 0x8263D940;
    'dispatch: loop {
        match pc {
            0x8263D940 => {
    //   block [0x8263D940..0x8263D9A4)
	// 8263D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D94C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D954: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263D958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D960: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8263D964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D974: 386AC2EC  addi r3, r10, -0x3d14
	ctx.r[3].s64 = ctx.r[10].s64 + -15636;
	// 8263D978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D97C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D990: 4BE29491  bl 0x82466e20
	ctx.lr = 0x8263D994;
	sub_82466E20(ctx, base);
	// 8263D994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D9A8 size=100
    let mut pc: u32 = 0x8263D9A8;
    'dispatch: loop {
        match pc {
            0x8263D9A8 => {
    //   block [0x8263D9A8..0x8263DA0C)
	// 8263D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D9B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D9BC: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D9C8: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8263D9CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D9DC: 386AC31C  addi r3, r10, -0x3ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -15588;
	// 8263D9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D9E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D9F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D9F8: 4BE29429  bl 0x82466e20
	ctx.lr = 0x8263D9FC;
	sub_82466E20(ctx, base);
	// 8263D9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DA10 size=112
    let mut pc: u32 = 0x8263DA10;
    'dispatch: loop {
        match pc {
            0x8263DA10 => {
    //   block [0x8263DA10..0x8263DA80)
	// 8263DA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DA1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DA24: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DA2C: 390B75E8  addi r8, r11, 0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30184;
	// 8263DA30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263DA34: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8263DA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DA3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DA48: 386AC34C  addi r3, r10, -0x3cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -15540;
	// 8263DA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DA6C: 4BE293B5  bl 0x82466e20
	ctx.lr = 0x8263DA70;
	sub_82466E20(ctx, base);
	// 8263DA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DA80 size=112
    let mut pc: u32 = 0x8263DA80;
    'dispatch: loop {
        match pc {
            0x8263DA80 => {
    //   block [0x8263DA80..0x8263DAF0)
	// 8263DA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DA8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DA94: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DA9C: 390B7678  addi r8, r11, 0x7678
	ctx.r[8].s64 = ctx.r[11].s64 + 30328;
	// 8263DAA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263DAA4: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8263DAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DAAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DAB8: 386AC37C  addi r3, r10, -0x3c84
	ctx.r[3].s64 = ctx.r[10].s64 + -15492;
	// 8263DABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DADC: 4BE29345  bl 0x82466e20
	ctx.lr = 0x8263DAE0;
	sub_82466E20(ctx, base);
	// 8263DAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DAF0 size=112
    let mut pc: u32 = 0x8263DAF0;
    'dispatch: loop {
        match pc {
            0x8263DAF0 => {
    //   block [0x8263DAF0..0x8263DB60)
	// 8263DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DAFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DB04: 38AABEFC  addi r5, r10, -0x4104
	ctx.r[5].s64 = ctx.r[10].s64 + -16644;
	// 8263DB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DB0C: 390B76D8  addi r8, r11, 0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + 30424;
	// 8263DB10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DB14: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8263DB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DB1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DB28: 386AC3AC  addi r3, r10, -0x3c54
	ctx.r[3].s64 = ctx.r[10].s64 + -15444;
	// 8263DB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DB4C: 4BE292D5  bl 0x82466e20
	ctx.lr = 0x8263DB50;
	sub_82466E20(ctx, base);
	// 8263DB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DB60 size=112
    let mut pc: u32 = 0x8263DB60;
    'dispatch: loop {
        match pc {
            0x8263DB60 => {
    //   block [0x8263DB60..0x8263DBD0)
	// 8263DB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DB6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DB74: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DB7C: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 8263DB80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263DB84: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8263DB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DB8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DB98: 386AC3DC  addi r3, r10, -0x3c24
	ctx.r[3].s64 = ctx.r[10].s64 + -15396;
	// 8263DB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DBBC: 4BE29265  bl 0x82466e20
	ctx.lr = 0x8263DBC0;
	sub_82466E20(ctx, base);
	// 8263DBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DBD0 size=112
    let mut pc: u32 = 0x8263DBD0;
    'dispatch: loop {
        match pc {
            0x8263DBD0 => {
    //   block [0x8263DBD0..0x8263DC40)
	// 8263DBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DBDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DBE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DBE4: 38AAC04C  addi r5, r10, -0x3fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -16308;
	// 8263DBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DBEC: 390B7798  addi r8, r11, 0x7798
	ctx.r[8].s64 = ctx.r[11].s64 + 30616;
	// 8263DBF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263DBF4: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8263DBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DBFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DC08: 386AC40C  addi r3, r10, -0x3bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -15348;
	// 8263DC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DC2C: 4BE291F5  bl 0x82466e20
	ctx.lr = 0x8263DC30;
	sub_82466E20(ctx, base);
	// 8263DC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DC40 size=112
    let mut pc: u32 = 0x8263DC40;
    'dispatch: loop {
        match pc {
            0x8263DC40 => {
    //   block [0x8263DC40..0x8263DCB0)
	// 8263DC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DC4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC50: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DC54: 38AAC25C  addi r5, r10, -0x3da4
	ctx.r[5].s64 = ctx.r[10].s64 + -15780;
	// 8263DC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DC5C: 390B77B0  addi r8, r11, 0x77b0
	ctx.r[8].s64 = ctx.r[11].s64 + 30640;
	// 8263DC60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DC64: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8263DC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DC6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DC78: 386AC43C  addi r3, r10, -0x3bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15300;
	// 8263DC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DC9C: 4BE29185  bl 0x82466e20
	ctx.lr = 0x8263DCA0;
	sub_82466E20(ctx, base);
	// 8263DCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DCB0 size=112
    let mut pc: u32 = 0x8263DCB0;
    'dispatch: loop {
        match pc {
            0x8263DCB0 => {
    //   block [0x8263DCB0..0x8263DD20)
	// 8263DCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DCBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DCC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DCC4: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263DCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DCCC: 390B77E0  addi r8, r11, 0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30688;
	// 8263DCD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263DCD4: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8263DCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DCDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DCE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DCE8: 386AC46C  addi r3, r10, -0x3b94
	ctx.r[3].s64 = ctx.r[10].s64 + -15252;
	// 8263DCEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DD0C: 4BE29115  bl 0x82466e20
	ctx.lr = 0x8263DD10;
	sub_82466E20(ctx, base);
	// 8263DD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263DD20 size=24
    let mut pc: u32 = 0x8263DD20;
    'dispatch: loop {
        match pc {
            0x8263DD20 => {
    //   block [0x8263DD20..0x8263DD38)
	// 8263DD20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DD24: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DD28: 394ABBF0  addi r10, r10, -0x4410
	ctx.r[10].s64 = ctx.r[10].s64 + -17424;
	// 8263DD2C: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263DD30: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263DD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DD38 size=116
    let mut pc: u32 = 0x8263DD38;
    'dispatch: loop {
        match pc {
            0x8263DD38 => {
    //   block [0x8263DD38..0x8263DDAC)
	// 8263DD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DD44: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DD48: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263DD4C: 390ABBF0  addi r8, r10, -0x4410
	ctx.r[8].s64 = ctx.r[10].s64 + -17424;
	// 8263DD50: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DD54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263DD58: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263DD5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DD60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263DD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DD68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DD6C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8263DD70: 396B6A4C  addi r11, r11, 0x6a4c
	ctx.r[11].s64 = ctx.r[11].s64 + 27212;
	// 8263DD74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DD78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DD7C: 386AC49C  addi r3, r10, -0x3b64
	ctx.r[3].s64 = ctx.r[10].s64 + -15204;
	// 8263DD80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263DD84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DD88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263DD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DD98: 4BE29089  bl 0x82466e20
	ctx.lr = 0x8263DD9C;
	sub_82466E20(ctx, base);
	// 8263DD9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DDA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DDA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DDB0 size=112
    let mut pc: u32 = 0x8263DDB0;
    'dispatch: loop {
        match pc {
            0x8263DDB0 => {
    //   block [0x8263DDB0..0x8263DE20)
	// 8263DDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DDBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DDC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DDC4: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263DDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DDCC: 390B7828  addi r8, r11, 0x7828
	ctx.r[8].s64 = ctx.r[11].s64 + 30760;
	// 8263DDD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DDD4: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8263DDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DDDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DDE8: 386AC4CC  addi r3, r10, -0x3b34
	ctx.r[3].s64 = ctx.r[10].s64 + -15156;
	// 8263DDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DE0C: 4BE29015  bl 0x82466e20
	ctx.lr = 0x8263DE10;
	sub_82466E20(ctx, base);
	// 8263DE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DE20 size=112
    let mut pc: u32 = 0x8263DE20;
    'dispatch: loop {
        match pc {
            0x8263DE20 => {
    //   block [0x8263DE20..0x8263DE90)
	// 8263DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DE2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DE30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DE34: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263DE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DE3C: 390B7858  addi r8, r11, 0x7858
	ctx.r[8].s64 = ctx.r[11].s64 + 30808;
	// 8263DE40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DE44: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8263DE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DE50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DE58: 386AC4FC  addi r3, r10, -0x3b04
	ctx.r[3].s64 = ctx.r[10].s64 + -15108;
	// 8263DE5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DE7C: 4BE28FA5  bl 0x82466e20
	ctx.lr = 0x8263DE80;
	sub_82466E20(ctx, base);
	// 8263DE80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DE90 size=100
    let mut pc: u32 = 0x8263DE90;
    'dispatch: loop {
        match pc {
            0x8263DE90 => {
    //   block [0x8263DE90..0x8263DEF4)
	// 8263DE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DE9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263DEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DEA4: 392A6ABC  addi r9, r10, 0x6abc
	ctx.r[9].s64 = ctx.r[10].s64 + 27324;
	// 8263DEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DEB0: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8263DEB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DEC4: 386AC52C  addi r3, r10, -0x3ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -15060;
	// 8263DEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DECC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263DED0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263DED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DED8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263DEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263DEE0: 4BE28F41  bl 0x82466e20
	ctx.lr = 0x8263DEE4;
	sub_82466E20(ctx, base);
	// 8263DEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263DEF8 size=24
    let mut pc: u32 = 0x8263DEF8;
    'dispatch: loop {
        match pc {
            0x8263DEF8 => {
    //   block [0x8263DEF8..0x8263DF10)
	// 8263DEF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DEFC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DF00: 394ABC98  addi r10, r10, -0x4368
	ctx.r[10].s64 = ctx.r[10].s64 + -17256;
	// 8263DF04: 816B7890  lwz r11, 0x7890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30864 as u32) ) } as u64;
	// 8263DF08: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263DF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DF10 size=112
    let mut pc: u32 = 0x8263DF10;
    'dispatch: loop {
        match pc {
            0x8263DF10 => {
    //   block [0x8263DF10..0x8263DF80)
	// 8263DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DF1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263DF20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263DF24: 392A6BF8  addi r9, r10, 0x6bf8
	ctx.r[9].s64 = ctx.r[10].s64 + 27640;
	// 8263DF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DF2C: 390BBC98  addi r8, r11, -0x4368
	ctx.r[8].s64 = ctx.r[11].s64 + -17256;
	// 8263DF30: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263DF34: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8263DF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DF3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DF48: 386AC55C  addi r3, r10, -0x3aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -15012;
	// 8263DF4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263DF50: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263DF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263DF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DF6C: 4BE28EB5  bl 0x82466e20
	ctx.lr = 0x8263DF70;
	sub_82466E20(ctx, base);
	// 8263DF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DF80 size=112
    let mut pc: u32 = 0x8263DF80;
    'dispatch: loop {
        match pc {
            0x8263DF80 => {
    //   block [0x8263DF80..0x8263DFF0)
	// 8263DF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DF8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DF90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DF94: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263DF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DF9C: 390B7898  addi r8, r11, 0x7898
	ctx.r[8].s64 = ctx.r[11].s64 + 30872;
	// 8263DFA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DFA4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8263DFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DFAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DFB8: 386AC58C  addi r3, r10, -0x3a74
	ctx.r[3].s64 = ctx.r[10].s64 + -14964;
	// 8263DFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DFDC: 4BE28E45  bl 0x82466e20
	ctx.lr = 0x8263DFE0;
	sub_82466E20(ctx, base);
	// 8263DFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DFF0 size=108
    let mut pc: u32 = 0x8263DFF0;
    'dispatch: loop {
        match pc {
            0x8263DFF0 => {
    //   block [0x8263DFF0..0x8263E05C)
	// 8263DFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DFFC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E004: 38EB78C8  addi r7, r11, 0x78c8
	ctx.r[7].s64 = ctx.r[11].s64 + 30920;
	// 8263E008: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263E00C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8263E010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263E01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E020: 386AC5BC  addi r3, r10, -0x3a44
	ctx.r[3].s64 = ctx.r[10].s64 + -14916;
	// 8263E024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263E028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263E048: 4BE28DD9  bl 0x82466e20
	ctx.lr = 0x8263E04C;
	sub_82466E20(ctx, base);
	// 8263E04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E060 size=100
    let mut pc: u32 = 0x8263E060;
    'dispatch: loop {
        match pc {
            0x8263E060 => {
    //   block [0x8263E060..0x8263E0C4)
	// 8263E060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E074: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E080: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8263E084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E094: 386AC5EC  addi r3, r10, -0x3a14
	ctx.r[3].s64 = ctx.r[10].s64 + -14868;
	// 8263E098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263E0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263E0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E0B0: 4BE28D71  bl 0x82466e20
	ctx.lr = 0x8263E0B4;
	sub_82466E20(ctx, base);
	// 8263E0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E0C8 size=112
    let mut pc: u32 = 0x8263E0C8;
    'dispatch: loop {
        match pc {
            0x8263E0C8 => {
    //   block [0x8263E0C8..0x8263E138)
	// 8263E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E0D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E0D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E0DC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E0E4: 390B78E0  addi r8, r11, 0x78e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30944;
	// 8263E0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E0EC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8263E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E100: 386AC61C  addi r3, r10, -0x39e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14820;
	// 8263E104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E124: 4BE28CFD  bl 0x82466e20
	ctx.lr = 0x8263E128;
	sub_82466E20(ctx, base);
	// 8263E128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E138 size=112
    let mut pc: u32 = 0x8263E138;
    'dispatch: loop {
        match pc {
            0x8263E138 => {
    //   block [0x8263E138..0x8263E1A8)
	// 8263E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E148: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E14C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E154: 390B78F8  addi r8, r11, 0x78f8
	ctx.r[8].s64 = ctx.r[11].s64 + 30968;
	// 8263E158: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E15C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8263E160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E170: 386AC64C  addi r3, r10, -0x39b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14772;
	// 8263E174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E194: 4BE28C8D  bl 0x82466e20
	ctx.lr = 0x8263E198;
	sub_82466E20(ctx, base);
	// 8263E198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E1A8 size=112
    let mut pc: u32 = 0x8263E1A8;
    'dispatch: loop {
        match pc {
            0x8263E1A8 => {
    //   block [0x8263E1A8..0x8263E218)
	// 8263E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E1B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E1B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E1BC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E1C4: 390B7928  addi r8, r11, 0x7928
	ctx.r[8].s64 = ctx.r[11].s64 + 31016;
	// 8263E1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E1CC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8263E1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E1D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E1E0: 386AC67C  addi r3, r10, -0x3984
	ctx.r[3].s64 = ctx.r[10].s64 + -14724;
	// 8263E1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E204: 4BE28C1D  bl 0x82466e20
	ctx.lr = 0x8263E208;
	sub_82466E20(ctx, base);
	// 8263E208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E218 size=112
    let mut pc: u32 = 0x8263E218;
    'dispatch: loop {
        match pc {
            0x8263E218 => {
    //   block [0x8263E218..0x8263E288)
	// 8263E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E228: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E22C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E234: 390B7958  addi r8, r11, 0x7958
	ctx.r[8].s64 = ctx.r[11].s64 + 31064;
	// 8263E238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E23C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8263E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E250: 386AC6AC  addi r3, r10, -0x3954
	ctx.r[3].s64 = ctx.r[10].s64 + -14676;
	// 8263E254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E274: 4BE28BAD  bl 0x82466e20
	ctx.lr = 0x8263E278;
	sub_82466E20(ctx, base);
	// 8263E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E288 size=112
    let mut pc: u32 = 0x8263E288;
    'dispatch: loop {
        match pc {
            0x8263E288 => {
    //   block [0x8263E288..0x8263E2F8)
	// 8263E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E294: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E298: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E29C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E2A4: 390B7988  addi r8, r11, 0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + 31112;
	// 8263E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E2AC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8263E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E2B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E2C0: 386AC6DC  addi r3, r10, -0x3924
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	// 8263E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E2E4: 4BE28B3D  bl 0x82466e20
	ctx.lr = 0x8263E2E8;
	sub_82466E20(ctx, base);
	// 8263E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E2F8 size=112
    let mut pc: u32 = 0x8263E2F8;
    'dispatch: loop {
        match pc {
            0x8263E2F8 => {
    //   block [0x8263E2F8..0x8263E368)
	// 8263E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E308: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E30C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E314: 390B79A0  addi r8, r11, 0x79a0
	ctx.r[8].s64 = ctx.r[11].s64 + 31136;
	// 8263E318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E31C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8263E320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E330: 386AC70C  addi r3, r10, -0x38f4
	ctx.r[3].s64 = ctx.r[10].s64 + -14580;
	// 8263E334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E354: 4BE28ACD  bl 0x82466e20
	ctx.lr = 0x8263E358;
	sub_82466E20(ctx, base);
	// 8263E358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E368 size=112
    let mut pc: u32 = 0x8263E368;
    'dispatch: loop {
        match pc {
            0x8263E368 => {
    //   block [0x8263E368..0x8263E3D8)
	// 8263E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E378: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E37C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E384: 390B79B8  addi r8, r11, 0x79b8
	ctx.r[8].s64 = ctx.r[11].s64 + 31160;
	// 8263E388: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E38C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8263E390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E3A0: 386AC73C  addi r3, r10, -0x38c4
	ctx.r[3].s64 = ctx.r[10].s64 + -14532;
	// 8263E3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E3C4: 4BE28A5D  bl 0x82466e20
	ctx.lr = 0x8263E3C8;
	sub_82466E20(ctx, base);
	// 8263E3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E3D8 size=112
    let mut pc: u32 = 0x8263E3D8;
    'dispatch: loop {
        match pc {
            0x8263E3D8 => {
    //   block [0x8263E3D8..0x8263E448)
	// 8263E3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E3E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E3EC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E3F4: 390B7A00  addi r8, r11, 0x7a00
	ctx.r[8].s64 = ctx.r[11].s64 + 31232;
	// 8263E3F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E3FC: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8263E400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E410: 386AC76C  addi r3, r10, -0x3894
	ctx.r[3].s64 = ctx.r[10].s64 + -14484;
	// 8263E414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E434: 4BE289ED  bl 0x82466e20
	ctx.lr = 0x8263E438;
	sub_82466E20(ctx, base);
	// 8263E438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E448 size=112
    let mut pc: u32 = 0x8263E448;
    'dispatch: loop {
        match pc {
            0x8263E448 => {
    //   block [0x8263E448..0x8263E4B8)
	// 8263E448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E458: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E45C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E464: 390B7A48  addi r8, r11, 0x7a48
	ctx.r[8].s64 = ctx.r[11].s64 + 31304;
	// 8263E468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E46C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8263E470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E480: 386AC79C  addi r3, r10, -0x3864
	ctx.r[3].s64 = ctx.r[10].s64 + -14436;
	// 8263E484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E4A4: 4BE2897D  bl 0x82466e20
	ctx.lr = 0x8263E4A8;
	sub_82466E20(ctx, base);
	// 8263E4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E4B8 size=112
    let mut pc: u32 = 0x8263E4B8;
    'dispatch: loop {
        match pc {
            0x8263E4B8 => {
    //   block [0x8263E4B8..0x8263E528)
	// 8263E4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E4C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E4C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E4CC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E4D4: 390B7A60  addi r8, r11, 0x7a60
	ctx.r[8].s64 = ctx.r[11].s64 + 31328;
	// 8263E4D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E4DC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8263E4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E4E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E4F0: 386AC7CC  addi r3, r10, -0x3834
	ctx.r[3].s64 = ctx.r[10].s64 + -14388;
	// 8263E4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E514: 4BE2890D  bl 0x82466e20
	ctx.lr = 0x8263E518;
	sub_82466E20(ctx, base);
	// 8263E518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E528 size=116
    let mut pc: u32 = 0x8263E528;
    'dispatch: loop {
        match pc {
            0x8263E528 => {
    //   block [0x8263E528..0x8263E59C)
	// 8263E528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E534: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263E538: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263E53C: 390A7A90  addi r8, r10, 0x7a90
	ctx.r[8].s64 = ctx.r[10].s64 + 31376;
	// 8263E540: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E548: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E54C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E550: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263E554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E55C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8263E560: 396B6C20  addi r11, r11, 0x6c20
	ctx.r[11].s64 = ctx.r[11].s64 + 27680;
	// 8263E564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E568: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E56C: 386AC7FC  addi r3, r10, -0x3804
	ctx.r[3].s64 = ctx.r[10].s64 + -14340;
	// 8263E570: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263E574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E578: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263E57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E588: 4BE28899  bl 0x82466e20
	ctx.lr = 0x8263E58C;
	sub_82466E20(ctx, base);
	// 8263E58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E5A0 size=116
    let mut pc: u32 = 0x8263E5A0;
    'dispatch: loop {
        match pc {
            0x8263E5A0 => {
    //   block [0x8263E5A0..0x8263E614)
	// 8263E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E5AC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263E5B0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8263E5B4: 390A7B08  addi r8, r10, 0x7b08
	ctx.r[8].s64 = ctx.r[10].s64 + 31496;
	// 8263E5B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E5BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E5C0: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263E5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E5D4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8263E5D8: 396B6C38  addi r11, r11, 0x6c38
	ctx.r[11].s64 = ctx.r[11].s64 + 27704;
	// 8263E5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E5E4: 386AC82C  addi r3, r10, -0x37d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14292;
	// 8263E5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263E5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263E5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E600: 4BE28821  bl 0x82466e20
	ctx.lr = 0x8263E604;
	sub_82466E20(ctx, base);
	// 8263E604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263E618 size=24
    let mut pc: u32 = 0x8263E618;
    'dispatch: loop {
        match pc {
            0x8263E618 => {
    //   block [0x8263E618..0x8263E630)
	// 8263E618: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E61C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263E620: 394ABCB0  addi r10, r10, -0x4350
	ctx.r[10].s64 = ctx.r[10].s64 + -17232;
	// 8263E624: 816B7B98  lwz r11, 0x7b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31640 as u32) ) } as u64;
	// 8263E628: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E630 size=116
    let mut pc: u32 = 0x8263E630;
    'dispatch: loop {
        match pc {
            0x8263E630 => {
    //   block [0x8263E630..0x8263E6A4)
	// 8263E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E63C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E640: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E644: 392B6C64  addi r9, r11, 0x6c64
	ctx.r[9].s64 = ctx.r[11].s64 + 27748;
	// 8263E648: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E64C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E650: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8263E654: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263E658: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263E65C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8263E660: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E664: 396BBCB0  addi r11, r11, -0x4350
	ctx.r[11].s64 = ctx.r[11].s64 + -17232;
	// 8263E668: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263E66C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E670: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263E674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E678: 386AC85C  addi r3, r10, -0x37a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14244;
	// 8263E67C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263E680: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263E684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E688: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263E68C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263E690: 4BE28791  bl 0x82466e20
	ctx.lr = 0x8263E694;
	sub_82466E20(ctx, base);
	// 8263E694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E6A8 size=112
    let mut pc: u32 = 0x8263E6A8;
    'dispatch: loop {
        match pc {
            0x8263E6A8 => {
    //   block [0x8263E6A8..0x8263E718)
	// 8263E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E6B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E6B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E6BC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E6C4: 390B7BA0  addi r8, r11, 0x7ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 31648;
	// 8263E6C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E6CC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8263E6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E6D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E6E0: 386AC88C  addi r3, r10, -0x3774
	ctx.r[3].s64 = ctx.r[10].s64 + -14196;
	// 8263E6E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E704: 4BE2871D  bl 0x82466e20
	ctx.lr = 0x8263E708;
	sub_82466E20(ctx, base);
	// 8263E708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E718 size=112
    let mut pc: u32 = 0x8263E718;
    'dispatch: loop {
        match pc {
            0x8263E718 => {
    //   block [0x8263E718..0x8263E788)
	// 8263E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E728: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E72C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E734: 390B7C00  addi r8, r11, 0x7c00
	ctx.r[8].s64 = ctx.r[11].s64 + 31744;
	// 8263E738: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263E73C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8263E740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E750: 386AC8BC  addi r3, r10, -0x3744
	ctx.r[3].s64 = ctx.r[10].s64 + -14148;
	// 8263E754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E774: 4BE286AD  bl 0x82466e20
	ctx.lr = 0x8263E778;
	sub_82466E20(ctx, base);
	// 8263E778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E788 size=112
    let mut pc: u32 = 0x8263E788;
    'dispatch: loop {
        match pc {
            0x8263E788 => {
    //   block [0x8263E788..0x8263E7F8)
	// 8263E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E794: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E798: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E79C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E7A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E7A4: 390B7CA8  addi r8, r11, 0x7ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 31912;
	// 8263E7A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263E7AC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8263E7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E7B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E7B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E7C0: 386AC8EC  addi r3, r10, -0x3714
	ctx.r[3].s64 = ctx.r[10].s64 + -14100;
	// 8263E7C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E7DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E7E4: 4BE2863D  bl 0x82466e20
	ctx.lr = 0x8263E7E8;
	sub_82466E20(ctx, base);
	// 8263E7E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E7F8 size=112
    let mut pc: u32 = 0x8263E7F8;
    'dispatch: loop {
        match pc {
            0x8263E7F8 => {
    //   block [0x8263E7F8..0x8263E868)
	// 8263E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E808: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E80C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E814: 390B7D20  addi r8, r11, 0x7d20
	ctx.r[8].s64 = ctx.r[11].s64 + 32032;
	// 8263E818: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E81C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8263E820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E830: 386AC91C  addi r3, r10, -0x36e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14052;
	// 8263E834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E854: 4BE285CD  bl 0x82466e20
	ctx.lr = 0x8263E858;
	sub_82466E20(ctx, base);
	// 8263E858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E868 size=112
    let mut pc: u32 = 0x8263E868;
    'dispatch: loop {
        match pc {
            0x8263E868 => {
    //   block [0x8263E868..0x8263E8D8)
	// 8263E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E874: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E878: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E87C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E884: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 8263E888: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263E88C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8263E890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E8A0: 386AC94C  addi r3, r10, -0x36b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14004;
	// 8263E8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E8C4: 4BE2855D  bl 0x82466e20
	ctx.lr = 0x8263E8C8;
	sub_82466E20(ctx, base);
	// 8263E8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E8D8 size=112
    let mut pc: u32 = 0x8263E8D8;
    'dispatch: loop {
        match pc {
            0x8263E8D8 => {
    //   block [0x8263E8D8..0x8263E948)
	// 8263E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E8E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E8E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E8EC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E8F4: 390B7DF8  addi r8, r11, 0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + 32248;
	// 8263E8F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E8FC: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8263E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E910: 386AC97C  addi r3, r10, -0x3684
	ctx.r[3].s64 = ctx.r[10].s64 + -13956;
	// 8263E914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E934: 4BE284ED  bl 0x82466e20
	ctx.lr = 0x8263E938;
	sub_82466E20(ctx, base);
	// 8263E938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E948 size=112
    let mut pc: u32 = 0x8263E948;
    'dispatch: loop {
        match pc {
            0x8263E948 => {
    //   block [0x8263E948..0x8263E9B8)
	// 8263E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E958: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E95C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E964: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 8263E968: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E96C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8263E970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E980: 386AC9AC  addi r3, r10, -0x3654
	ctx.r[3].s64 = ctx.r[10].s64 + -13908;
	// 8263E984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E9A4: 4BE2847D  bl 0x82466e20
	ctx.lr = 0x8263E9A8;
	sub_82466E20(ctx, base);
	// 8263E9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E9B8 size=112
    let mut pc: u32 = 0x8263E9B8;
    'dispatch: loop {
        match pc {
            0x8263E9B8 => {
    //   block [0x8263E9B8..0x8263EA28)
	// 8263E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E9C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E9C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E9CC: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263E9D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E9D4: 390B7EB8  addi r8, r11, 0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 32440;
	// 8263E9D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E9DC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8263E9E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E9E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E9E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E9F0: 386AC9DC  addi r3, r10, -0x3624
	ctx.r[3].s64 = ctx.r[10].s64 + -13860;
	// 8263E9F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E9F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EA00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EA10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EA14: 4BE2840D  bl 0x82466e20
	ctx.lr = 0x8263EA18;
	sub_82466E20(ctx, base);
	// 8263EA18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EA28 size=112
    let mut pc: u32 = 0x8263EA28;
    'dispatch: loop {
        match pc {
            0x8263EA28 => {
    //   block [0x8263EA28..0x8263EA98)
	// 8263EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EA34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EA38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EA3C: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EA44: 390B7EE8  addi r8, r11, 0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 32488;
	// 8263EA48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263EA4C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8263EA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EA54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EA60: 386ACA0C  addi r3, r10, -0x35f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13812;
	// 8263EA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EA84: 4BE2839D  bl 0x82466e20
	ctx.lr = 0x8263EA88;
	sub_82466E20(ctx, base);
	// 8263EA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EA98 size=100
    let mut pc: u32 = 0x8263EA98;
    'dispatch: loop {
        match pc {
            0x8263EA98 => {
    //   block [0x8263EA98..0x8263EAFC)
	// 8263EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EAA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EAAC: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EAB8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8263EABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EACC: 386ACA3C  addi r3, r10, -0x35c4
	ctx.r[3].s64 = ctx.r[10].s64 + -13764;
	// 8263EAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263EADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263EAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EAE8: 4BE28339  bl 0x82466e20
	ctx.lr = 0x8263EAEC;
	sub_82466E20(ctx, base);
	// 8263EAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EB00 size=112
    let mut pc: u32 = 0x8263EB00;
    'dispatch: loop {
        match pc {
            0x8263EB00 => {
    //   block [0x8263EB00..0x8263EB70)
	// 8263EB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EB0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EB14: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EB1C: 390B7F18  addi r8, r11, 0x7f18
	ctx.r[8].s64 = ctx.r[11].s64 + 32536;
	// 8263EB20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263EB24: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8263EB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EB2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EB38: 386ACA6C  addi r3, r10, -0x3594
	ctx.r[3].s64 = ctx.r[10].s64 + -13716;
	// 8263EB3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EB5C: 4BE282C5  bl 0x82466e20
	ctx.lr = 0x8263EB60;
	sub_82466E20(ctx, base);
	// 8263EB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EB70 size=112
    let mut pc: u32 = 0x8263EB70;
    'dispatch: loop {
        match pc {
            0x8263EB70 => {
    //   block [0x8263EB70..0x8263EBE0)
	// 8263EB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EB7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EB84: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EB8C: 390B7F30  addi r8, r11, 0x7f30
	ctx.r[8].s64 = ctx.r[11].s64 + 32560;
	// 8263EB90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263EB94: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8263EB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EB9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EBA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EBA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EBA8: 386ACA9C  addi r3, r10, -0x3564
	ctx.r[3].s64 = ctx.r[10].s64 + -13668;
	// 8263EBAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EBB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EBCC: 4BE28255  bl 0x82466e20
	ctx.lr = 0x8263EBD0;
	sub_82466E20(ctx, base);
	// 8263EBD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EBD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EBD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EBE0 size=112
    let mut pc: u32 = 0x8263EBE0;
    'dispatch: loop {
        match pc {
            0x8263EBE0 => {
    //   block [0x8263EBE0..0x8263EC50)
	// 8263EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EBEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EBF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EBF4: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263EBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EBFC: 390B7F90  addi r8, r11, 0x7f90
	ctx.r[8].s64 = ctx.r[11].s64 + 32656;
	// 8263EC00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263EC04: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8263EC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EC0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EC18: 386ACACC  addi r3, r10, -0x3534
	ctx.r[3].s64 = ctx.r[10].s64 + -13620;
	// 8263EC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EC3C: 4BE281E5  bl 0x82466e20
	ctx.lr = 0x8263EC40;
	sub_82466E20(ctx, base);
	// 8263EC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EC50 size=112
    let mut pc: u32 = 0x8263EC50;
    'dispatch: loop {
        match pc {
            0x8263EC50 => {
    //   block [0x8263EC50..0x8263ECC0)
	// 8263EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EC64: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263EC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EC6C: 390B7FA8  addi r8, r11, 0x7fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 32680;
	// 8263EC70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263EC74: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8263EC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EC7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EC88: 386ACAFC  addi r3, r10, -0x3504
	ctx.r[3].s64 = ctx.r[10].s64 + -13572;
	// 8263EC8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ECA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ECA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ECA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ECAC: 4BE28175  bl 0x82466e20
	ctx.lr = 0x8263ECB0;
	sub_82466E20(ctx, base);
	// 8263ECB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ECB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ECB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ECC0 size=112
    let mut pc: u32 = 0x8263ECC0;
    'dispatch: loop {
        match pc {
            0x8263ECC0 => {
    //   block [0x8263ECC0..0x8263ED30)
	// 8263ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ECC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ECCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ECD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ECD4: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263ECD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ECDC: 390B7FD8  addi r8, r11, 0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32728;
	// 8263ECE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263ECE4: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8263ECE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ECEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ECF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ECF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ECF8: 386ACB2C  addi r3, r10, -0x34d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	// 8263ECFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263ED00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ED08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ED0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ED10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ED14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ED18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ED1C: 4BE28105  bl 0x82466e20
	ctx.lr = 0x8263ED20;
	sub_82466E20(ctx, base);
	// 8263ED20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ED24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ED28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ED2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263ED30 size=24
    let mut pc: u32 = 0x8263ED30;
    'dispatch: loop {
        match pc {
            0x8263ED30 => {
    //   block [0x8263ED30..0x8263ED48)
	// 8263ED30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ED34: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263ED38: 394ABD58  addi r10, r10, -0x42a8
	ctx.r[10].s64 = ctx.r[10].s64 + -17064;
	// 8263ED3C: 816B7B9C  lwz r11, 0x7b9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31644 as u32) ) } as u64;
	// 8263ED40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ED48 size=112
    let mut pc: u32 = 0x8263ED48;
    'dispatch: loop {
        match pc {
            0x8263ED48 => {
    //   block [0x8263ED48..0x8263EDB8)
	// 8263ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ED54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263ED58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263ED5C: 392A6CC0  addi r9, r10, 0x6cc0
	ctx.r[9].s64 = ctx.r[10].s64 + 27840;
	// 8263ED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ED64: 390BBD58  addi r8, r11, -0x42a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17064;
	// 8263ED68: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263ED6C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8263ED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ED74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ED78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ED7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ED80: 386ACB5C  addi r3, r10, -0x34a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13476;
	// 8263ED84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263ED88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263ED8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ED90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ED94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ED98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ED9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EDA4: 4BE2807D  bl 0x82466e20
	ctx.lr = 0x8263EDA8;
	sub_82466E20(ctx, base);
	// 8263EDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EDB8 size=108
    let mut pc: u32 = 0x8263EDB8;
    'dispatch: loop {
        match pc {
            0x8263EDB8 => {
    //   block [0x8263EDB8..0x8263EE24)
	// 8263EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EDC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EDCC: 38EB7FF0  addi r7, r11, 0x7ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 32752;
	// 8263EDD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263EDD4: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8263EDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EDDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EDE8: 386ACB8C  addi r3, r10, -0x3474
	ctx.r[3].s64 = ctx.r[10].s64 + -13428;
	// 8263EDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EE10: 4BE28011  bl 0x82466e20
	ctx.lr = 0x8263EE14;
	sub_82466E20(ctx, base);
	// 8263EE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EE28 size=108
    let mut pc: u32 = 0x8263EE28;
    'dispatch: loop {
        match pc {
            0x8263EE28 => {
    //   block [0x8263EE28..0x8263EE94)
	// 8263EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EE34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EE3C: 38EB8008  addi r7, r11, -0x7ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -32760;
	// 8263EE40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263EE44: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8263EE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EE50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EE58: 386ACBBC  addi r3, r10, -0x3444
	ctx.r[3].s64 = ctx.r[10].s64 + -13380;
	// 8263EE5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EE7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EE80: 4BE27FA1  bl 0x82466e20
	ctx.lr = 0x8263EE84;
	sub_82466E20(ctx, base);
	// 8263EE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EE98 size=116
    let mut pc: u32 = 0x8263EE98;
    'dispatch: loop {
        match pc {
            0x8263EE98 => {
    //   block [0x8263EE98..0x8263EF0C)
	// 8263EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EEA4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EEA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263EEAC: 390B8054  addi r8, r11, -0x7fac
	ctx.r[8].s64 = ctx.r[11].s64 + -32684;
	// 8263EEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EEB4: 392A6D78  addi r9, r10, 0x6d78
	ctx.r[9].s64 = ctx.r[10].s64 + 28024;
	// 8263EEB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EEBC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263EEC0: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EEC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EECC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EEDC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263EEE0: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8263EEE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263EEE8: 386BCBEC  addi r3, r11, -0x3414
	ctx.r[3].s64 = ctx.r[11].s64 + -13332;
	// 8263EEEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263EEF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EEF8: 4BE27F29  bl 0x82466e20
	ctx.lr = 0x8263EEFC;
	sub_82466E20(ctx, base);
	// 8263EEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263EF10 size=24
    let mut pc: u32 = 0x8263EF10;
    'dispatch: loop {
        match pc {
            0x8263EF10 => {
    //   block [0x8263EF10..0x8263EF28)
	// 8263EF10: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EF14: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263EF18: 394ABDA0  addi r10, r10, -0x4260
	ctx.r[10].s64 = ctx.r[10].s64 + -16992;
	// 8263EF1C: 816B806C  lwz r11, -0x7f94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32660 as u32) ) } as u64;
	// 8263EF20: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263EF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EF28 size=116
    let mut pc: u32 = 0x8263EF28;
    'dispatch: loop {
        match pc {
            0x8263EF28 => {
    //   block [0x8263EF28..0x8263EF9C)
	// 8263EF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EF34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263EF3C: 390BBDA0  addi r8, r11, -0x4260
	ctx.r[8].s64 = ctx.r[11].s64 + -16992;
	// 8263EF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EF44: 392A6DD4  addi r9, r10, 0x6dd4
	ctx.r[9].s64 = ctx.r[10].s64 + 28116;
	// 8263EF48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EF4C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263EF50: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EF54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EF5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EF6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263EF70: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8263EF74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263EF78: 386BCC1C  addi r3, r11, -0x33e4
	ctx.r[3].s64 = ctx.r[11].s64 + -13284;
	// 8263EF7C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263EF80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EF88: 4BE27E99  bl 0x82466e20
	ctx.lr = 0x8263EF8C;
	sub_82466E20(ctx, base);
	// 8263EF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EFA0 size=108
    let mut pc: u32 = 0x8263EFA0;
    'dispatch: loop {
        match pc {
            0x8263EFA0 => {
    //   block [0x8263EFA0..0x8263F00C)
	// 8263EFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EFAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EFB4: 38EB8078  addi r7, r11, -0x7f88
	ctx.r[7].s64 = ctx.r[11].s64 + -32648;
	// 8263EFB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263EFBC: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8263EFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EFC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EFD0: 386ACC4C  addi r3, r10, -0x33b4
	ctx.r[3].s64 = ctx.r[10].s64 + -13236;
	// 8263EFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EFF8: 4BE27E29  bl 0x82466e20
	ctx.lr = 0x8263EFFC;
	sub_82466E20(ctx, base);
	// 8263EFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F010 size=112
    let mut pc: u32 = 0x8263F010;
    'dispatch: loop {
        match pc {
            0x8263F010 => {
    //   block [0x8263F010..0x8263F080)
	// 8263F010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F01C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F020: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F024: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F02C: 390B80A8  addi r8, r11, -0x7f58
	ctx.r[8].s64 = ctx.r[11].s64 + -32600;
	// 8263F030: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F034: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8263F038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F048: 386ACC7C  addi r3, r10, -0x3384
	ctx.r[3].s64 = ctx.r[10].s64 + -13188;
	// 8263F04C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F06C: 4BE27DB5  bl 0x82466e20
	ctx.lr = 0x8263F070;
	sub_82466E20(ctx, base);
	// 8263F070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F080 size=112
    let mut pc: u32 = 0x8263F080;
    'dispatch: loop {
        match pc {
            0x8263F080 => {
    //   block [0x8263F080..0x8263F0F0)
	// 8263F080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F08C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F090: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F094: 392A6E18  addi r9, r10, 0x6e18
	ctx.r[9].s64 = ctx.r[10].s64 + 28184;
	// 8263F098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F09C: 390B80C8  addi r8, r11, -0x7f38
	ctx.r[8].s64 = ctx.r[11].s64 + -32568;
	// 8263F0A0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263F0A4: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8263F0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F0AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F0B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F0B8: 386ACCAC  addi r3, r10, -0x3354
	ctx.r[3].s64 = ctx.r[10].s64 + -13140;
	// 8263F0BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F0C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F0C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F0D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F0D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F0DC: 4BE27D45  bl 0x82466e20
	ctx.lr = 0x8263F0E0;
	sub_82466E20(ctx, base);
	// 8263F0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F0F0 size=112
    let mut pc: u32 = 0x8263F0F0;
    'dispatch: loop {
        match pc {
            0x8263F0F0 => {
    //   block [0x8263F0F0..0x8263F160)
	// 8263F0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F0FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F100: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F104: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F10C: 390B8110  addi r8, r11, -0x7ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -32496;
	// 8263F110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F114: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8263F118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F128: 386ACCDC  addi r3, r10, -0x3324
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	// 8263F12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F14C: 4BE27CD5  bl 0x82466e20
	ctx.lr = 0x8263F150;
	sub_82466E20(ctx, base);
	// 8263F150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F160 size=112
    let mut pc: u32 = 0x8263F160;
    'dispatch: loop {
        match pc {
            0x8263F160 => {
    //   block [0x8263F160..0x8263F1D0)
	// 8263F160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F170: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F174: 392A6E44  addi r9, r10, 0x6e44
	ctx.r[9].s64 = ctx.r[10].s64 + 28228;
	// 8263F178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F17C: 390B8128  addi r8, r11, -0x7ed8
	ctx.r[8].s64 = ctx.r[11].s64 + -32472;
	// 8263F180: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8263F184: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8263F188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F18C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F190: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F198: 386ACD0C  addi r3, r10, -0x32f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13044;
	// 8263F19C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F1A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F1BC: 4BE27C65  bl 0x82466e20
	ctx.lr = 0x8263F1C0;
	sub_82466E20(ctx, base);
	// 8263F1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F1D0 size=112
    let mut pc: u32 = 0x8263F1D0;
    'dispatch: loop {
        match pc {
            0x8263F1D0 => {
    //   block [0x8263F1D0..0x8263F240)
	// 8263F1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F1DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F1E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F1E4: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F1EC: 390B81B8  addi r8, r11, -0x7e48
	ctx.r[8].s64 = ctx.r[11].s64 + -32328;
	// 8263F1F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F1F4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8263F1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F208: 386ACD3C  addi r3, r10, -0x32c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12996;
	// 8263F20C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F22C: 4BE27BF5  bl 0x82466e20
	ctx.lr = 0x8263F230;
	sub_82466E20(ctx, base);
	// 8263F230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F240 size=112
    let mut pc: u32 = 0x8263F240;
    'dispatch: loop {
        match pc {
            0x8263F240 => {
    //   block [0x8263F240..0x8263F2B0)
	// 8263F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F24C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F250: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F254: 38AACD9C  addi r5, r10, -0x3264
	ctx.r[5].s64 = ctx.r[10].s64 + -12900;
	// 8263F258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F25C: 390B81D0  addi r8, r11, -0x7e30
	ctx.r[8].s64 = ctx.r[11].s64 + -32304;
	// 8263F260: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263F264: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8263F268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F278: 386ACD6C  addi r3, r10, -0x3294
	ctx.r[3].s64 = ctx.r[10].s64 + -12948;
	// 8263F27C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F29C: 4BE27B85  bl 0x82466e20
	ctx.lr = 0x8263F2A0;
	sub_82466E20(ctx, base);
	// 8263F2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F2B0 size=100
    let mut pc: u32 = 0x8263F2B0;
    'dispatch: loop {
        match pc {
            0x8263F2B0 => {
    //   block [0x8263F2B0..0x8263F314)
	// 8263F2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F2BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F2C4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263F2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F2D0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8263F2D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F2E4: 386ACD9C  addi r3, r10, -0x3264
	ctx.r[3].s64 = ctx.r[10].s64 + -12900;
	// 8263F2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F2EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F2F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263F2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F2F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263F2FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F300: 4BE27B21  bl 0x82466e20
	ctx.lr = 0x8263F304;
	sub_82466E20(ctx, base);
	// 8263F304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263F318 size=24
    let mut pc: u32 = 0x8263F318;
    'dispatch: loop {
        match pc {
            0x8263F318 => {
    //   block [0x8263F318..0x8263F330)
	// 8263F318: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F31C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263F320: 394ABE78  addi r10, r10, -0x4188
	ctx.r[10].s64 = ctx.r[10].s64 + -16776;
	// 8263F324: 816B8248  lwz r11, -0x7db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32184 as u32) ) } as u64;
	// 8263F328: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263F32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F330 size=116
    let mut pc: u32 = 0x8263F330;
    'dispatch: loop {
        match pc {
            0x8263F330 => {
    //   block [0x8263F330..0x8263F3A4)
	// 8263F330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F344: 390BBE78  addi r8, r11, -0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + -16776;
	// 8263F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F34C: 392A6E80  addi r9, r10, 0x6e80
	ctx.r[9].s64 = ctx.r[10].s64 + 28288;
	// 8263F350: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F354: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263F358: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F364: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F374: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263F378: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8263F37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F380: 386BCDCC  addi r3, r11, -0x3234
	ctx.r[3].s64 = ctx.r[11].s64 + -12852;
	// 8263F384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F390: 4BE27A91  bl 0x82466e20
	ctx.lr = 0x8263F394;
	sub_82466E20(ctx, base);
	// 8263F394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F3A8 size=108
    let mut pc: u32 = 0x8263F3A8;
    'dispatch: loop {
        match pc {
            0x8263F3A8 => {
    //   block [0x8263F3A8..0x8263F414)
	// 8263F3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F3B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F3B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F3BC: 38EB824C  addi r7, r11, -0x7db4
	ctx.r[7].s64 = ctx.r[11].s64 + -32180;
	// 8263F3C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F3C4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8263F3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F3CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F3D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F3D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F3D8: 386ACDFC  addi r3, r10, -0x3204
	ctx.r[3].s64 = ctx.r[10].s64 + -12804;
	// 8263F3DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F3FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F400: 4BE27A21  bl 0x82466e20
	ctx.lr = 0x8263F404;
	sub_82466E20(ctx, base);
	// 8263F404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F418 size=112
    let mut pc: u32 = 0x8263F418;
    'dispatch: loop {
        match pc {
            0x8263F418 => {
    //   block [0x8263F418..0x8263F488)
	// 8263F418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F428: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F42C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F434: 390B827C  addi r8, r11, -0x7d84
	ctx.r[8].s64 = ctx.r[11].s64 + -32132;
	// 8263F438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F43C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8263F440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F450: 386ACE2C  addi r3, r10, -0x31d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12756;
	// 8263F454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F474: 4BE279AD  bl 0x82466e20
	ctx.lr = 0x8263F478;
	sub_82466E20(ctx, base);
	// 8263F478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F488 size=112
    let mut pc: u32 = 0x8263F488;
    'dispatch: loop {
        match pc {
            0x8263F488 => {
    //   block [0x8263F488..0x8263F4F8)
	// 8263F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F494: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F498: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F49C: 392A6EA4  addi r9, r10, 0x6ea4
	ctx.r[9].s64 = ctx.r[10].s64 + 28324;
	// 8263F4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F4A4: 390B8298  addi r8, r11, -0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + -32104;
	// 8263F4A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263F4AC: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8263F4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F4B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F4C0: 386ACE5C  addi r3, r10, -0x31a4
	ctx.r[3].s64 = ctx.r[10].s64 + -12708;
	// 8263F4C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F4C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F4DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F4E4: 4BE2793D  bl 0x82466e20
	ctx.lr = 0x8263F4E8;
	sub_82466E20(ctx, base);
	// 8263F4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F4F8 size=112
    let mut pc: u32 = 0x8263F4F8;
    'dispatch: loop {
        match pc {
            0x8263F4F8 => {
    //   block [0x8263F4F8..0x8263F568)
	// 8263F4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F508: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F50C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F514: 390B8340  addi r8, r11, -0x7cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -31936;
	// 8263F518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F51C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8263F520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F530: 386ACE8C  addi r3, r10, -0x3174
	ctx.r[3].s64 = ctx.r[10].s64 + -12660;
	// 8263F534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F554: 4BE278CD  bl 0x82466e20
	ctx.lr = 0x8263F558;
	sub_82466E20(ctx, base);
	// 8263F558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F568 size=108
    let mut pc: u32 = 0x8263F568;
    'dispatch: loop {
        match pc {
            0x8263F568 => {
    //   block [0x8263F568..0x8263F5D4)
	// 8263F568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F574: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F57C: 38EB8358  addi r7, r11, -0x7ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -31912;
	// 8263F580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F584: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8263F588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F58C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F598: 386ACEBC  addi r3, r10, -0x3144
	ctx.r[3].s64 = ctx.r[10].s64 + -12612;
	// 8263F59C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F5A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F5B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F5B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F5BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F5C0: 4BE27861  bl 0x82466e20
	ctx.lr = 0x8263F5C4;
	sub_82466E20(ctx, base);
	// 8263F5C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F5C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F5CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F5D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F5D8 size=112
    let mut pc: u32 = 0x8263F5D8;
    'dispatch: loop {
        match pc {
            0x8263F5D8 => {
    //   block [0x8263F5D8..0x8263F648)
	// 8263F5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F5E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F5E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F5EC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F5F4: 390B8388  addi r8, r11, -0x7c78
	ctx.r[8].s64 = ctx.r[11].s64 + -31864;
	// 8263F5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F5FC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8263F600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F610: 386ACEEC  addi r3, r10, -0x3114
	ctx.r[3].s64 = ctx.r[10].s64 + -12564;
	// 8263F614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F634: 4BE277ED  bl 0x82466e20
	ctx.lr = 0x8263F638;
	sub_82466E20(ctx, base);
	// 8263F638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F648 size=112
    let mut pc: u32 = 0x8263F648;
    'dispatch: loop {
        match pc {
            0x8263F648 => {
    //   block [0x8263F648..0x8263F6B8)
	// 8263F648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F658: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F65C: 392A6ED8  addi r9, r10, 0x6ed8
	ctx.r[9].s64 = ctx.r[10].s64 + 28376;
	// 8263F660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F664: 390B83A8  addi r8, r11, -0x7c58
	ctx.r[8].s64 = ctx.r[11].s64 + -31832;
	// 8263F668: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263F66C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8263F670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F680: 386ACF1C  addi r3, r10, -0x30e4
	ctx.r[3].s64 = ctx.r[10].s64 + -12516;
	// 8263F684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F6A4: 4BE2777D  bl 0x82466e20
	ctx.lr = 0x8263F6A8;
	sub_82466E20(ctx, base);
	// 8263F6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F6B8 size=112
    let mut pc: u32 = 0x8263F6B8;
    'dispatch: loop {
        match pc {
            0x8263F6B8 => {
    //   block [0x8263F6B8..0x8263F728)
	// 8263F6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F6C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F6C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F6CC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F6D4: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8263F6D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263F6DC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8263F6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F6E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F6E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F6F0: 386ACF4C  addi r3, r10, -0x30b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12468;
	// 8263F6F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F714: 4BE2770D  bl 0x82466e20
	ctx.lr = 0x8263F718;
	sub_82466E20(ctx, base);
	// 8263F718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F728 size=112
    let mut pc: u32 = 0x8263F728;
    'dispatch: loop {
        match pc {
            0x8263F728 => {
    //   block [0x8263F728..0x8263F798)
	// 8263F728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F738: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F73C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F744: 390B8498  addi r8, r11, -0x7b68
	ctx.r[8].s64 = ctx.r[11].s64 + -31592;
	// 8263F748: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263F74C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8263F750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F760: 386ACF7C  addi r3, r10, -0x3084
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	// 8263F764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F784: 4BE2769D  bl 0x82466e20
	ctx.lr = 0x8263F788;
	sub_82466E20(ctx, base);
	// 8263F788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F798 size=100
    let mut pc: u32 = 0x8263F798;
    'dispatch: loop {
        match pc {
            0x8263F798 => {
    //   block [0x8263F798..0x8263F7FC)
	// 8263F798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F7A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F7AC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F7B8: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8263F7BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F7CC: 386ACFAC  addi r3, r10, -0x3054
	ctx.r[3].s64 = ctx.r[10].s64 + -12372;
	// 8263F7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F7D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263F7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F7E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263F7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F7E8: 4BE27639  bl 0x82466e20
	ctx.lr = 0x8263F7EC;
	sub_82466E20(ctx, base);
	// 8263F7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F800 size=112
    let mut pc: u32 = 0x8263F800;
    'dispatch: loop {
        match pc {
            0x8263F800 => {
    //   block [0x8263F800..0x8263F870)
	// 8263F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F80C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F810: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F814: 38AACC1C  addi r5, r10, -0x33e4
	ctx.r[5].s64 = ctx.r[10].s64 + -13284;
	// 8263F818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F81C: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 8263F820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263F824: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8263F828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F82C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F838: 386ACFDC  addi r3, r10, -0x3024
	ctx.r[3].s64 = ctx.r[10].s64 + -12324;
	// 8263F83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F85C: 4BE275C5  bl 0x82466e20
	ctx.lr = 0x8263F860;
	sub_82466E20(ctx, base);
	// 8263F860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F870 size=112
    let mut pc: u32 = 0x8263F870;
    'dispatch: loop {
        match pc {
            0x8263F870 => {
    //   block [0x8263F870..0x8263F8E0)
	// 8263F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F87C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F880: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F884: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263F888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F88C: 390B8588  addi r8, r11, -0x7a78
	ctx.r[8].s64 = ctx.r[11].s64 + -31352;
	// 8263F890: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F894: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8263F898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F89C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F8A8: 386AD00C  addi r3, r10, -0x2ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -12276;
	// 8263F8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F8CC: 4BE27555  bl 0x82466e20
	ctx.lr = 0x8263F8D0;
	sub_82466E20(ctx, base);
	// 8263F8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F8E0 size=108
    let mut pc: u32 = 0x8263F8E0;
    'dispatch: loop {
        match pc {
            0x8263F8E0 => {
    //   block [0x8263F8E0..0x8263F94C)
	// 8263F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F8EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F8F4: 38EB85A0  addi r7, r11, -0x7a60
	ctx.r[7].s64 = ctx.r[11].s64 + -31328;
	// 8263F8F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F8FC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8263F900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F910: 386AD03C  addi r3, r10, -0x2fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -12228;
	// 8263F914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F938: 4BE274E9  bl 0x82466e20
	ctx.lr = 0x8263F93C;
	sub_82466E20(ctx, base);
	// 8263F93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F950 size=112
    let mut pc: u32 = 0x8263F950;
    'dispatch: loop {
        match pc {
            0x8263F950 => {
    //   block [0x8263F950..0x8263F9C0)
	// 8263F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F95C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F960: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F964: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263F968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F96C: 390B85D0  addi r8, r11, -0x7a30
	ctx.r[8].s64 = ctx.r[11].s64 + -31280;
	// 8263F970: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263F974: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8263F978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F97C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F988: 386AD06C  addi r3, r10, -0x2f94
	ctx.r[3].s64 = ctx.r[10].s64 + -12180;
	// 8263F98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F9AC: 4BE27475  bl 0x82466e20
	ctx.lr = 0x8263F9B0;
	sub_82466E20(ctx, base);
	// 8263F9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F9C0 size=112
    let mut pc: u32 = 0x8263F9C0;
    'dispatch: loop {
        match pc {
            0x8263F9C0 => {
    //   block [0x8263F9C0..0x8263FA30)
	// 8263F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F9CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F9D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F9D4: 392A6F04  addi r9, r10, 0x6f04
	ctx.r[9].s64 = ctx.r[10].s64 + 28420;
	// 8263F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F9DC: 390B8660  addi r8, r11, -0x79a0
	ctx.r[8].s64 = ctx.r[11].s64 + -31136;
	// 8263F9E0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263F9E4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8263F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F9EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F9F8: 386AD09C  addi r3, r10, -0x2f64
	ctx.r[3].s64 = ctx.r[10].s64 + -12132;
	// 8263F9FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FA00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FA14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FA1C: 4BE27405  bl 0x82466e20
	ctx.lr = 0x8263FA20;
	sub_82466E20(ctx, base);
	// 8263FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FA30 size=112
    let mut pc: u32 = 0x8263FA30;
    'dispatch: loop {
        match pc {
            0x8263FA30 => {
    //   block [0x8263FA30..0x8263FAA0)
	// 8263FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FA3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FA40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FA44: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FA4C: 390B86A8  addi r8, r11, -0x7958
	ctx.r[8].s64 = ctx.r[11].s64 + -31064;
	// 8263FA50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263FA54: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8263FA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FA5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FA60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FA68: 386AD0CC  addi r3, r10, -0x2f34
	ctx.r[3].s64 = ctx.r[10].s64 + -12084;
	// 8263FA6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FA8C: 4BE27395  bl 0x82466e20
	ctx.lr = 0x8263FA90;
	sub_82466E20(ctx, base);
	// 8263FA90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FAA0 size=108
    let mut pc: u32 = 0x8263FAA0;
    'dispatch: loop {
        match pc {
            0x8263FAA0 => {
    //   block [0x8263FAA0..0x8263FB0C)
	// 8263FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FAAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FAB4: 38EB86C0  addi r7, r11, -0x7940
	ctx.r[7].s64 = ctx.r[11].s64 + -31040;
	// 8263FAB8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263FABC: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8263FAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FAC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FAC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FAD0: 386AD0FC  addi r3, r10, -0x2f04
	ctx.r[3].s64 = ctx.r[10].s64 + -12036;
	// 8263FAD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FAF8: 4BE27329  bl 0x82466e20
	ctx.lr = 0x8263FAFC;
	sub_82466E20(ctx, base);
	// 8263FAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FB10 size=116
    let mut pc: u32 = 0x8263FB10;
    'dispatch: loop {
        match pc {
            0x8263FB10 => {
    //   block [0x8263FB10..0x8263FB84)
	// 8263FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FB1C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FB20: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263FB24: 390A8750  addi r8, r10, -0x78b0
	ctx.r[8].s64 = ctx.r[10].s64 + -30896;
	// 8263FB28: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FB2C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263FB30: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263FB34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FB38: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FB44: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8263FB48: 396B6F18  addi r11, r11, 0x6f18
	ctx.r[11].s64 = ctx.r[11].s64 + 28440;
	// 8263FB4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FB50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FB54: 386AD12C  addi r3, r10, -0x2ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -11988;
	// 8263FB58: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263FB5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FB60: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263FB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FB70: 4BE272B1  bl 0x82466e20
	ctx.lr = 0x8263FB74;
	sub_82466E20(ctx, base);
	// 8263FB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FB88 size=108
    let mut pc: u32 = 0x8263FB88;
    'dispatch: loop {
        match pc {
            0x8263FB88 => {
    //   block [0x8263FB88..0x8263FBF4)
	// 8263FB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FB94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FB98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FB9C: 38EB8828  addi r7, r11, -0x77d8
	ctx.r[7].s64 = ctx.r[11].s64 + -30680;
	// 8263FBA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263FBA4: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8263FBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FBAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FBB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FBB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FBB8: 386AD15C  addi r3, r10, -0x2ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -11940;
	// 8263FBBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FBC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FBC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FBD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FBD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FBDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FBE0: 4BE27241  bl 0x82466e20
	ctx.lr = 0x8263FBE4;
	sub_82466E20(ctx, base);
	// 8263FBE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FBF8 size=112
    let mut pc: u32 = 0x8263FBF8;
    'dispatch: loop {
        match pc {
            0x8263FBF8 => {
    //   block [0x8263FBF8..0x8263FC68)
	// 8263FBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FC04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FC0C: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263FC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FC14: 390B8870  addi r8, r11, -0x7790
	ctx.r[8].s64 = ctx.r[11].s64 + -30608;
	// 8263FC18: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263FC1C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8263FC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FC24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FC30: 386AD18C  addi r3, r10, -0x2e74
	ctx.r[3].s64 = ctx.r[10].s64 + -11892;
	// 8263FC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FC54: 4BE271CD  bl 0x82466e20
	ctx.lr = 0x8263FC58;
	sub_82466E20(ctx, base);
	// 8263FC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FC68 size=112
    let mut pc: u32 = 0x8263FC68;
    'dispatch: loop {
        match pc {
            0x8263FC68 => {
    //   block [0x8263FC68..0x8263FCD8)
	// 8263FC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FC74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FC7C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FC84: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 8263FC88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263FC8C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8263FC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FC94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FCA0: 386AD1BC  addi r3, r10, -0x2e44
	ctx.r[3].s64 = ctx.r[10].s64 + -11844;
	// 8263FCA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FCC4: 4BE2715D  bl 0x82466e20
	ctx.lr = 0x8263FCC8;
	sub_82466E20(ctx, base);
	// 8263FCC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FCD8 size=108
    let mut pc: u32 = 0x8263FCD8;
    'dispatch: loop {
        match pc {
            0x8263FCD8 => {
    //   block [0x8263FCD8..0x8263FD44)
	// 8263FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FCE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FCE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FCE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FCEC: 38EB8918  addi r7, r11, -0x76e8
	ctx.r[7].s64 = ctx.r[11].s64 + -30440;
	// 8263FCF0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263FCF4: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8263FCF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FCFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FD00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FD08: 386AD1EC  addi r3, r10, -0x2e14
	ctx.r[3].s64 = ctx.r[10].s64 + -11796;
	// 8263FD0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FD30: 4BE270F1  bl 0x82466e20
	ctx.lr = 0x8263FD34;
	sub_82466E20(ctx, base);
	// 8263FD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FD48 size=108
    let mut pc: u32 = 0x8263FD48;
    'dispatch: loop {
        match pc {
            0x8263FD48 => {
    //   block [0x8263FD48..0x8263FDB4)
	// 8263FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FD54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FD58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FD5C: 38EB8978  addi r7, r11, -0x7688
	ctx.r[7].s64 = ctx.r[11].s64 + -30344;
	// 8263FD60: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263FD64: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8263FD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FD6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FD70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FD78: 386AD21C  addi r3, r10, -0x2de4
	ctx.r[3].s64 = ctx.r[10].s64 + -11748;
	// 8263FD7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FD9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FDA0: 4BE27081  bl 0x82466e20
	ctx.lr = 0x8263FDA4;
	sub_82466E20(ctx, base);
	// 8263FDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FDB8 size=112
    let mut pc: u32 = 0x8263FDB8;
    'dispatch: loop {
        match pc {
            0x8263FDB8 => {
    //   block [0x8263FDB8..0x8263FE28)
	// 8263FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FDC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FDC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FDCC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FDD4: 390B89F0  addi r8, r11, -0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + -30224;
	// 8263FDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263FDDC: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8263FDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FDE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FDF0: 386AD24C  addi r3, r10, -0x2db4
	ctx.r[3].s64 = ctx.r[10].s64 + -11700;
	// 8263FDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FE14: 4BE2700D  bl 0x82466e20
	ctx.lr = 0x8263FE18;
	sub_82466E20(ctx, base);
	// 8263FE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263FE28 size=24
    let mut pc: u32 = 0x8263FE28;
    'dispatch: loop {
        match pc {
            0x8263FE28 => {
    //   block [0x8263FE28..0x8263FE40)
	// 8263FE28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FE2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FE30: 394ABEF0  addi r10, r10, -0x4110
	ctx.r[10].s64 = ctx.r[10].s64 + -16656;
	// 8263FE34: 816B8A38  lwz r11, -0x75c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30152 as u32) ) } as u64;
	// 8263FE38: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FE40 size=116
    let mut pc: u32 = 0x8263FE40;
    'dispatch: loop {
        match pc {
            0x8263FE40 => {
    //   block [0x8263FE40..0x8263FEB4)
	// 8263FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FE4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FE50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263FE54: 390BBEF0  addi r8, r11, -0x4110
	ctx.r[8].s64 = ctx.r[11].s64 + -16656;
	// 8263FE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FE5C: 392A6F7C  addi r9, r10, 0x6f7c
	ctx.r[9].s64 = ctx.r[10].s64 + 28540;
	// 8263FE60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FE64: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263FE68: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263FE6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FE74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FE84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263FE88: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8263FE8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FE90: 386BD27C  addi r3, r11, -0x2d84
	ctx.r[3].s64 = ctx.r[11].s64 + -11652;
	// 8263FE94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FE98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FEA0: 4BE26F81  bl 0x82466e20
	ctx.lr = 0x8263FEA4;
	sub_82466E20(ctx, base);
	// 8263FEA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FEB8 size=112
    let mut pc: u32 = 0x8263FEB8;
    'dispatch: loop {
        match pc {
            0x8263FEB8 => {
    //   block [0x8263FEB8..0x8263FF28)
	// 8263FEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FEC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FEC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FECC: 38AAD27C  addi r5, r10, -0x2d84
	ctx.r[5].s64 = ctx.r[10].s64 + -11652;
	// 8263FED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FED4: 390B8A3C  addi r8, r11, -0x75c4
	ctx.r[8].s64 = ctx.r[11].s64 + -30148;
	// 8263FED8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263FEDC: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8263FEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FEE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FEE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FEEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FEF0: 386AD2AC  addi r3, r10, -0x2d54
	ctx.r[3].s64 = ctx.r[10].s64 + -11604;
	// 8263FEF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FF14: 4BE26F0D  bl 0x82466e20
	ctx.lr = 0x8263FF18;
	sub_82466E20(ctx, base);
	// 8263FF18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263FF28 size=24
    let mut pc: u32 = 0x8263FF28;
    'dispatch: loop {
        match pc {
            0x8263FF28 => {
    //   block [0x8263FF28..0x8263FF40)
	// 8263FF28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FF2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FF30: 394ABF08  addi r10, r10, -0x40f8
	ctx.r[10].s64 = ctx.r[10].s64 + -16632;
	// 8263FF34: 816B8A6C  lwz r11, -0x7594(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30100 as u32) ) } as u64;
	// 8263FF38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263FF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FF40 size=116
    let mut pc: u32 = 0x8263FF40;
    'dispatch: loop {
        match pc {
            0x8263FF40 => {
    //   block [0x8263FF40..0x8263FFB4)
	// 8263FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FF4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263FF54: 390BBF08  addi r8, r11, -0x40f8
	ctx.r[8].s64 = ctx.r[11].s64 + -16632;
	// 8263FF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FF5C: 392A6FB8  addi r9, r10, 0x6fb8
	ctx.r[9].s64 = ctx.r[10].s64 + 28600;
	// 8263FF60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FF64: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263FF68: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 8263FF6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FF74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FF84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263FF88: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8263FF8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FF90: 386BD2DC  addi r3, r11, -0x2d24
	ctx.r[3].s64 = ctx.r[11].s64 + -11556;
	// 8263FF94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FF98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FFA0: 4BE26E81  bl 0x82466e20
	ctx.lr = 0x8263FFA4;
	sub_82466E20(ctx, base);
	// 8263FFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FFB8 size=112
    let mut pc: u32 = 0x8263FFB8;
    'dispatch: loop {
        match pc {
            0x8263FFB8 => {
    //   block [0x8263FFB8..0x82640028)
	// 8263FFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FFC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FFC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FFCC: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 8263FFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FFD4: 390B8A70  addi r8, r11, -0x7590
	ctx.r[8].s64 = ctx.r[11].s64 + -30096;
	// 8263FFD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263FFDC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8263FFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FFE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FFF0: 386AD30C  addi r3, r10, -0x2cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -11508;
	// 8263FFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264000C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640014: 4BE26E0D  bl 0x82466e20
	ctx.lr = 0x82640018;
	sub_82466E20(ctx, base);
	// 82640018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264001C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640028 size=112
    let mut pc: u32 = 0x82640028;
    'dispatch: loop {
        match pc {
            0x82640028 => {
    //   block [0x82640028..0x82640098)
	// 82640028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264002C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640038: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264003C: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 82640040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640044: 390B8AD0  addi r8, r11, -0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + -30000;
	// 82640048: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264004C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82640050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264005C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640060: 386AD33C  addi r3, r10, -0x2cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11460;
	// 82640064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264006C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264007C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640084: 4BE26D9D  bl 0x82466e20
	ctx.lr = 0x82640088;
	sub_82466E20(ctx, base);
	// 82640088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640098 size=112
    let mut pc: u32 = 0x82640098;
    'dispatch: loop {
        match pc {
            0x82640098 => {
    //   block [0x82640098..0x82640108)
	// 82640098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264009C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826400A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826400A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826400A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826400AC: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 826400B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826400B4: 390B8B00  addi r8, r11, -0x7500
	ctx.r[8].s64 = ctx.r[11].s64 + -29952;
	// 826400B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826400BC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826400C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826400C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826400C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826400CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826400D0: 386AD36C  addi r3, r10, -0x2c94
	ctx.r[3].s64 = ctx.r[10].s64 + -11412;
	// 826400D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826400D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826400DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826400E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826400E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826400E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826400EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826400F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826400F4: 4BE26D2D  bl 0x82466e20
	ctx.lr = 0x826400F8;
	sub_82466E20(ctx, base);
	// 826400F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826400FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640108 size=108
    let mut pc: u32 = 0x82640108;
    'dispatch: loop {
        match pc {
            0x82640108 => {
    //   block [0x82640108..0x82640174)
	// 82640108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640114: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264011C: 38EB8B48  addi r7, r11, -0x74b8
	ctx.r[7].s64 = ctx.r[11].s64 + -29880;
	// 82640120: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82640124: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82640128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264012C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640138: 386AD39C  addi r3, r10, -0x2c64
	ctx.r[3].s64 = ctx.r[10].s64 + -11364;
	// 8264013C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264015C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640160: 4BE26CC1  bl 0x82466e20
	ctx.lr = 0x82640164;
	sub_82466E20(ctx, base);
	// 82640164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264016C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640178 size=112
    let mut pc: u32 = 0x82640178;
    'dispatch: loop {
        match pc {
            0x82640178 => {
    //   block [0x82640178..0x826401E8)
	// 82640178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264017C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640188: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264018C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 82640190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640194: 390B8B78  addi r8, r11, -0x7488
	ctx.r[8].s64 = ctx.r[11].s64 + -29832;
	// 82640198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264019C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826401A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826401A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826401A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826401AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826401B0: 386AD3CC  addi r3, r10, -0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + -11316;
	// 826401B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826401B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826401BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826401C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826401C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826401C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826401CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826401D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826401D4: 4BE26C4D  bl 0x82466e20
	ctx.lr = 0x826401D8;
	sub_82466E20(ctx, base);
	// 826401D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826401DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826401E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826401E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826401E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826401E8 size=108
    let mut pc: u32 = 0x826401E8;
    'dispatch: loop {
        match pc {
            0x826401E8 => {
    //   block [0x826401E8..0x82640254)
	// 826401E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826401EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826401F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826401F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826401F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826401FC: 38EB8B98  addi r7, r11, -0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + -29800;
	// 82640200: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82640204: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82640208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264020C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640218: 386AD3FC  addi r3, r10, -0x2c04
	ctx.r[3].s64 = ctx.r[10].s64 + -11268;
	// 8264021C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264022C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264023C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640240: 4BE26BE1  bl 0x82466e20
	ctx.lr = 0x82640244;
	sub_82466E20(ctx, base);
	// 82640244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264024C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640258 size=108
    let mut pc: u32 = 0x82640258;
    'dispatch: loop {
        match pc {
            0x82640258 => {
    //   block [0x82640258..0x826402C4)
	// 82640258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640264: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264026C: 38EB8BE0  addi r7, r11, -0x7420
	ctx.r[7].s64 = ctx.r[11].s64 + -29728;
	// 82640270: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82640274: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82640278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264027C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640288: 386AD42C  addi r3, r10, -0x2bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -11220;
	// 8264028C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826402A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826402A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826402A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826402AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826402B0: 4BE26B71  bl 0x82466e20
	ctx.lr = 0x826402B4;
	sub_82466E20(ctx, base);
	// 826402B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826402B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826402BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826402C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826402C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826402C8 size=116
    let mut pc: u32 = 0x826402C8;
    'dispatch: loop {
        match pc {
            0x826402C8 => {
    //   block [0x826402C8..0x8264033C)
	// 826402C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826402CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826402D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826402D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826402D8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826402DC: 392B6FEC  addi r9, r11, 0x6fec
	ctx.r[9].s64 = ctx.r[11].s64 + 28652;
	// 826402E0: 38AAD8AC  addi r5, r10, -0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + -10068;
	// 826402E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826402E8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826402EC: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826402F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826402F4: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826402F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826402FC: 396B8C40  addi r11, r11, -0x73c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29632;
	// 82640300: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82640304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640308: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640310: 386AD45C  addi r3, r10, -0x2ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -11172;
	// 82640314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640318: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264031C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640320: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82640324: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640328: 4BE26AF9  bl 0x82466e20
	ctx.lr = 0x8264032C;
	sub_82466E20(ctx, base);
	// 8264032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640340 size=100
    let mut pc: u32 = 0x82640340;
    'dispatch: loop {
        match pc {
            0x82640340 => {
    //   block [0x82640340..0x826403A4)
	// 82640340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264034C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640354: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264035C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640360: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82640364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640374: 386AD48C  addi r3, r10, -0x2b74
	ctx.r[3].s64 = ctx.r[10].s64 + -11124;
	// 82640378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264037C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264038C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640390: 4BE26A91  bl 0x82466e20
	ctx.lr = 0x82640394;
	sub_82466E20(ctx, base);
	// 82640394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264039C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826403A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826403A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826403A8 size=100
    let mut pc: u32 = 0x826403A8;
    'dispatch: loop {
        match pc {
            0x826403A8 => {
    //   block [0x826403A8..0x8264040C)
	// 826403A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826403AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826403B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826403B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826403B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826403BC: 38AAD51C  addi r5, r10, -0x2ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -10980;
	// 826403C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826403C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826403C8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826403CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826403D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826403D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826403D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826403DC: 386AD4BC  addi r3, r10, -0x2b44
	ctx.r[3].s64 = ctx.r[10].s64 + -11076;
	// 826403E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826403E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826403E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826403EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826403F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826403F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826403F8: 4BE26A29  bl 0x82466e20
	ctx.lr = 0x826403FC;
	sub_82466E20(ctx, base);
	// 826403FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640410 size=100
    let mut pc: u32 = 0x82640410;
    'dispatch: loop {
        match pc {
            0x82640410 => {
    //   block [0x82640410..0x82640474)
	// 82640410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264041C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640424: 38AAD45C  addi r5, r10, -0x2ba4
	ctx.r[5].s64 = ctx.r[10].s64 + -11172;
	// 82640428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264042C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640430: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82640434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264043C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640444: 386AD4EC  addi r3, r10, -0x2b14
	ctx.r[3].s64 = ctx.r[10].s64 + -11028;
	// 82640448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264044C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264045C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640460: 4BE269C1  bl 0x82466e20
	ctx.lr = 0x82640464;
	sub_82466E20(ctx, base);
	// 82640464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264046C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640478 size=104
    let mut pc: u32 = 0x82640478;
    'dispatch: loop {
        match pc {
            0x82640478 => {
    //   block [0x82640478..0x826404E0)
	// 82640478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264047C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640484: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82640488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264048C: 392A7068  addi r9, r10, 0x7068
	ctx.r[9].s64 = ctx.r[10].s64 + 28776;
	// 82640490: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640498: 38AAD48C  addi r5, r10, -0x2b74
	ctx.r[5].s64 = ctx.r[10].s64 + -11124;
	// 8264049C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826404A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826404A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826404A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826404AC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826404B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826404B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826404B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826404BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826404C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826404C4: 386AD51C  addi r3, r10, -0x2ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -10980;
	// 826404C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826404CC: 4BE26955  bl 0x82466e20
	ctx.lr = 0x826404D0;
	sub_82466E20(ctx, base);
	// 826404D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826404D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826404D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826404DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826404E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826404E0 size=108
    let mut pc: u32 = 0x826404E0;
    'dispatch: loop {
        match pc {
            0x826404E0 => {
    //   block [0x826404E0..0x8264054C)
	// 826404E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826404E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826404E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826404EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826404F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826404F4: 38EB8DD8  addi r7, r11, -0x7228
	ctx.r[7].s64 = ctx.r[11].s64 + -29224;
	// 826404F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826404FC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82640500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264050C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640510: 386AD54C  addi r3, r10, -0x2ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -10932;
	// 82640514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264051C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264052C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640538: 4BE268E9  bl 0x82466e20
	ctx.lr = 0x8264053C;
	sub_82466E20(ctx, base);
	// 8264053C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640550 size=112
    let mut pc: u32 = 0x82640550;
    'dispatch: loop {
        match pc {
            0x82640550 => {
    //   block [0x82640550..0x826405C0)
	// 82640550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264055C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640560: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640564: 38AAD51C  addi r5, r10, -0x2ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -10980;
	// 82640568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264056C: 390B8E08  addi r8, r11, -0x71f8
	ctx.r[8].s64 = ctx.r[11].s64 + -29176;
	// 82640570: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82640574: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82640578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264057C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640588: 386AD57C  addi r3, r10, -0x2a84
	ctx.r[3].s64 = ctx.r[10].s64 + -10884;
	// 8264058C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264059C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826405A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826405A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826405A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826405AC: 4BE26875  bl 0x82466e20
	ctx.lr = 0x826405B0;
	sub_82466E20(ctx, base);
	// 826405B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826405B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826405B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826405BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826405C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826405C0 size=24
    let mut pc: u32 = 0x826405C0;
    'dispatch: loop {
        match pc {
            0x826405C0 => {
    //   block [0x826405C0..0x826405D8)
	// 826405C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826405C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826405C8: 394ABF80  addi r10, r10, -0x4080
	ctx.r[10].s64 = ctx.r[10].s64 + -16512;
	// 826405CC: 816B8EB0  lwz r11, -0x7150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29008 as u32) ) } as u64;
	// 826405D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826405D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826405D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826405D8 size=116
    let mut pc: u32 = 0x826405D8;
    'dispatch: loop {
        match pc {
            0x826405D8 => {
    //   block [0x826405D8..0x8264064C)
	// 826405D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826405DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826405E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826405E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826405E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826405EC: 390BBF80  addi r8, r11, -0x4080
	ctx.r[8].s64 = ctx.r[11].s64 + -16512;
	// 826405F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826405F4: 392A70D0  addi r9, r10, 0x70d0
	ctx.r[9].s64 = ctx.r[10].s64 + 28880;
	// 826405F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826405FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82640600: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640604: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264060C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264061C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82640620: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82640624: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640628: 386BD5AC  addi r3, r11, -0x2a54
	ctx.r[3].s64 = ctx.r[11].s64 + -10836;
	// 8264062C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640638: 4BE267E9  bl 0x82466e20
	ctx.lr = 0x8264063C;
	sub_82466E20(ctx, base);
	// 8264063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640650 size=100
    let mut pc: u32 = 0x82640650;
    'dispatch: loop {
        match pc {
            0x82640650 => {
    //   block [0x82640650..0x826406B4)
	// 82640650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264065C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640664: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264066C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640670: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82640674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264067C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640684: 386AD5DC  addi r3, r10, -0x2a24
	ctx.r[3].s64 = ctx.r[10].s64 + -10788;
	// 82640688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264068C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826406A0: 4BE26781  bl 0x82466e20
	ctx.lr = 0x826406A4;
	sub_82466E20(ctx, base);
	// 826406A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826406A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826406AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826406B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826406B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826406B8 size=100
    let mut pc: u32 = 0x826406B8;
    'dispatch: loop {
        match pc {
            0x826406B8 => {
    //   block [0x826406B8..0x8264071C)
	// 826406B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826406BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826406C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826406C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826406C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826406CC: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826406D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826406D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826406D8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826406DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826406E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826406E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826406E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826406EC: 386AD60C  addi r3, r10, -0x29f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10740;
	// 826406F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826406F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826406F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826406FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640708: 4BE26719  bl 0x82466e20
	ctx.lr = 0x8264070C;
	sub_82466E20(ctx, base);
	// 8264070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640720 size=112
    let mut pc: u32 = 0x82640720;
    'dispatch: loop {
        match pc {
            0x82640720 => {
    //   block [0x82640720..0x82640790)
	// 82640720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264072C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640730: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640734: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264073C: 390B8EB4  addi r8, r11, -0x714c
	ctx.r[8].s64 = ctx.r[11].s64 + -29004;
	// 82640740: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82640744: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82640748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264074C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640758: 386AD63C  addi r3, r10, -0x29c4
	ctx.r[3].s64 = ctx.r[10].s64 + -10692;
	// 8264075C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264076C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264077C: 4BE266A5  bl 0x82466e20
	ctx.lr = 0x82640780;
	sub_82466E20(ctx, base);
	// 82640780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264078C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640790 size=100
    let mut pc: u32 = 0x82640790;
    'dispatch: loop {
        match pc {
            0x82640790 => {
    //   block [0x82640790..0x826407F4)
	// 82640790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264079C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826407A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826407A4: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826407A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826407AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826407B0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826407B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826407B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826407BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826407C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826407C4: 386AD66C  addi r3, r10, -0x2994
	ctx.r[3].s64 = ctx.r[10].s64 + -10644;
	// 826407C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826407CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826407D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826407D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826407D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826407DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826407E0: 4BE26641  bl 0x82466e20
	ctx.lr = 0x826407E4;
	sub_82466E20(ctx, base);
	// 826407E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826407E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826407EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826407F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826407F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826407F8 size=100
    let mut pc: u32 = 0x826407F8;
    'dispatch: loop {
        match pc {
            0x826407F8 => {
    //   block [0x826407F8..0x8264085C)
	// 826407F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826407FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264080C: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640818: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8264081C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264082C: 386AD69C  addi r3, r10, -0x2964
	ctx.r[3].s64 = ctx.r[10].s64 + -10596;
	// 82640830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640834: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640838: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640840: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640848: 4BE265D9  bl 0x82466e20
	ctx.lr = 0x8264084C;
	sub_82466E20(ctx, base);
	// 8264084C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640860 size=100
    let mut pc: u32 = 0x82640860;
    'dispatch: loop {
        match pc {
            0x82640860 => {
    //   block [0x82640860..0x826408C4)
	// 82640860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264086C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640874: 38AAD5DC  addi r5, r10, -0x2a24
	ctx.r[5].s64 = ctx.r[10].s64 + -10788;
	// 82640878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264087C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640880: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82640884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264088C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640894: 386AD6CC  addi r3, r10, -0x2934
	ctx.r[3].s64 = ctx.r[10].s64 + -10548;
	// 82640898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264089C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826408A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826408A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826408A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826408AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826408B0: 4BE26571  bl 0x82466e20
	ctx.lr = 0x826408B4;
	sub_82466E20(ctx, base);
	// 826408B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826408B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826408BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826408C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826408C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826408C8 size=100
    let mut pc: u32 = 0x826408C8;
    'dispatch: loop {
        match pc {
            0x826408C8 => {
    //   block [0x826408C8..0x8264092C)
	// 826408C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826408CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826408D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826408D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826408D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826408DC: 38AAD69C  addi r5, r10, -0x2964
	ctx.r[5].s64 = ctx.r[10].s64 + -10596;
	// 826408E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826408E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826408E8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826408EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826408F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826408F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826408F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826408FC: 386AD6FC  addi r3, r10, -0x2904
	ctx.r[3].s64 = ctx.r[10].s64 + -10500;
	// 82640900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264090C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640918: 4BE26509  bl 0x82466e20
	ctx.lr = 0x8264091C;
	sub_82466E20(ctx, base);
	// 8264091C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640930 size=100
    let mut pc: u32 = 0x82640930;
    'dispatch: loop {
        match pc {
            0x82640930 => {
    //   block [0x82640930..0x82640994)
	// 82640930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264093C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640944: 38AAD5DC  addi r5, r10, -0x2a24
	ctx.r[5].s64 = ctx.r[10].s64 + -10788;
	// 82640948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264094C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640950: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82640954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264095C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640964: 386AD72C  addi r3, r10, -0x28d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10452;
	// 82640968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264096C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264097C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640980: 4BE264A1  bl 0x82466e20
	ctx.lr = 0x82640984;
	sub_82466E20(ctx, base);
	// 82640984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264098C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640998 size=112
    let mut pc: u32 = 0x82640998;
    'dispatch: loop {
        match pc {
            0x82640998 => {
    //   block [0x82640998..0x82640A08)
	// 82640998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264099C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826409A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826409A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826409A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826409AC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826409B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826409B4: 390B8EE4  addi r8, r11, -0x711c
	ctx.r[8].s64 = ctx.r[11].s64 + -28956;
	// 826409B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826409BC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826409C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826409C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826409C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826409CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826409D0: 386AD75C  addi r3, r10, -0x28a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10404;
	// 826409D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826409D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826409DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826409E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826409E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826409E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826409EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826409F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826409F4: 4BE2642D  bl 0x82466e20
	ctx.lr = 0x826409F8;
	sub_82466E20(ctx, base);
	// 826409F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826409FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640A08 size=112
    let mut pc: u32 = 0x82640A08;
    'dispatch: loop {
        match pc {
            0x82640A08 => {
    //   block [0x82640A08..0x82640A78)
	// 82640A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640A1C: 38AAD7EC  addi r5, r10, -0x2814
	ctx.r[5].s64 = ctx.r[10].s64 + -10260;
	// 82640A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640A24: 390B8F14  addi r8, r11, -0x70ec
	ctx.r[8].s64 = ctx.r[11].s64 + -28908;
	// 82640A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640A2C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82640A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640A40: 386AD78C  addi r3, r10, -0x2874
	ctx.r[3].s64 = ctx.r[10].s64 + -10356;
	// 82640A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640A64: 4BE263BD  bl 0x82466e20
	ctx.lr = 0x82640A68;
	sub_82466E20(ctx, base);
	// 82640A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640A78 size=112
    let mut pc: u32 = 0x82640A78;
    'dispatch: loop {
        match pc {
            0x82640A78 => {
    //   block [0x82640A78..0x82640AE8)
	// 82640A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640A8C: 38AAD8AC  addi r5, r10, -0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + -10068;
	// 82640A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640A94: 390B8F2C  addi r8, r11, -0x70d4
	ctx.r[8].s64 = ctx.r[11].s64 + -28884;
	// 82640A98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82640A9C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82640AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640AB0: 386AD7BC  addi r3, r10, -0x2844
	ctx.r[3].s64 = ctx.r[10].s64 + -10308;
	// 82640AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640AD4: 4BE2634D  bl 0x82466e20
	ctx.lr = 0x82640AD8;
	sub_82466E20(ctx, base);
	// 82640AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640AE8 size=112
    let mut pc: u32 = 0x82640AE8;
    'dispatch: loop {
        match pc {
            0x82640AE8 => {
    //   block [0x82640AE8..0x82640B58)
	// 82640AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640AFC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 82640B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640B04: 390B8F5C  addi r8, r11, -0x70a4
	ctx.r[8].s64 = ctx.r[11].s64 + -28836;
	// 82640B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640B0C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82640B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640B20: 386AD7EC  addi r3, r10, -0x2814
	ctx.r[3].s64 = ctx.r[10].s64 + -10260;
	// 82640B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640B44: 4BE262DD  bl 0x82466e20
	ctx.lr = 0x82640B48;
	sub_82466E20(ctx, base);
	// 82640B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640B58 size=112
    let mut pc: u32 = 0x82640B58;
    'dispatch: loop {
        match pc {
            0x82640B58 => {
    //   block [0x82640B58..0x82640BC8)
	// 82640B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640B64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640B6C: 38AAD7EC  addi r5, r10, -0x2814
	ctx.r[5].s64 = ctx.r[10].s64 + -10260;
	// 82640B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640B74: 390B8F74  addi r8, r11, -0x708c
	ctx.r[8].s64 = ctx.r[11].s64 + -28812;
	// 82640B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640B7C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82640B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640B84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640B90: 386AD81C  addi r3, r10, -0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10212;
	// 82640B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640BB4: 4BE2626D  bl 0x82466e20
	ctx.lr = 0x82640BB8;
	sub_82466E20(ctx, base);
	// 82640BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640BC8 size=116
    let mut pc: u32 = 0x82640BC8;
    'dispatch: loop {
        match pc {
            0x82640BC8 => {
    //   block [0x82640BC8..0x82640C3C)
	// 82640BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640BD4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640BD8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82640BDC: 390A8F90  addi r8, r10, -0x7070
	ctx.r[8].s64 = ctx.r[10].s64 + -28784;
	// 82640BE0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640BE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82640BE8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640BEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640BF0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640BFC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82640C00: 396B70E4  addi r11, r11, 0x70e4
	ctx.r[11].s64 = ctx.r[11].s64 + 28900;
	// 82640C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640C08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640C0C: 386AD84C  addi r3, r10, -0x27b4
	ctx.r[3].s64 = ctx.r[10].s64 + -10164;
	// 82640C10: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82640C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640C18: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82640C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640C28: 4BE261F9  bl 0x82466e20
	ctx.lr = 0x82640C2C;
	sub_82466E20(ctx, base);
	// 82640C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82640C40 size=48
    let mut pc: u32 = 0x82640C40;
    'dispatch: loop {
        match pc {
            0x82640C40 => {
    //   block [0x82640C40..0x82640C70)
	// 82640C40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C44: 814B9040  lwz r10, -0x6fc0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28608 as u32) ) } as u64;
	// 82640C48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C4C: 396BC040  addi r11, r11, -0x3fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -16320;
	// 82640C50: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82640C54: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640C58: 814A903C  lwz r10, -0x6fc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28612 as u32) ) } as u64;
	// 82640C5C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82640C60: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640C64: 814A9038  lwz r10, -0x6fc8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28616 as u32) ) } as u64;
	// 82640C68: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 82640C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640C70 size=116
    let mut pc: u32 = 0x82640C70;
    'dispatch: loop {
        match pc {
            0x82640C70 => {
    //   block [0x82640C70..0x82640CE4)
	// 82640C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82640C80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640C84: 392B71B8  addi r9, r11, 0x71b8
	ctx.r[9].s64 = ctx.r[11].s64 + 29112;
	// 82640C88: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640C90: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82640C94: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82640C98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C9C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82640CA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640CA4: 396BC040  addi r11, r11, -0x3fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -16320;
	// 82640CA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82640CAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640CB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82640CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640CB8: 386AD87C  addi r3, r10, -0x2784
	ctx.r[3].s64 = ctx.r[10].s64 + -10116;
	// 82640CBC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82640CC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82640CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640CC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82640CCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640CD0: 4BE26151  bl 0x82466e20
	ctx.lr = 0x82640CD4;
	sub_82466E20(ctx, base);
	// 82640CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640CE8 size=116
    let mut pc: u32 = 0x82640CE8;
    'dispatch: loop {
        match pc {
            0x82640CE8 => {
    //   block [0x82640CE8..0x82640D5C)
	// 82640CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640CF4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640CF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82640CFC: 390B9048  addi r8, r11, -0x6fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -28600;
	// 82640D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640D04: 392A7334  addi r9, r10, 0x7334
	ctx.r[9].s64 = ctx.r[10].s64 + 29492;
	// 82640D08: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D0C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82640D10: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640D14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640D1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640D2C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82640D30: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82640D34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640D38: 386BD8AC  addi r3, r11, -0x2754
	ctx.r[3].s64 = ctx.r[11].s64 + -10068;
	// 82640D3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640D40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640D48: 4BE260D9  bl 0x82466e20
	ctx.lr = 0x82640D4C;
	sub_82466E20(ctx, base);
	// 82640D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640D60 size=112
    let mut pc: u32 = 0x82640D60;
    'dispatch: loop {
        match pc {
            0x82640D60 => {
    //   block [0x82640D60..0x82640DD0)
	// 82640D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640D74: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640D7C: 390B90D8  addi r8, r11, -0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + -28456;
	// 82640D80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640D84: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82640D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640D98: 386AD8DC  addi r3, r10, -0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + -10020;
	// 82640D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640DBC: 4BE26065  bl 0x82466e20
	ctx.lr = 0x82640DC0;
	sub_82466E20(ctx, base);
	// 82640DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640DD0 size=112
    let mut pc: u32 = 0x82640DD0;
    'dispatch: loop {
        match pc {
            0x82640DD0 => {
    //   block [0x82640DD0..0x82640E40)
	// 82640DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640DDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640DE0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640DE4: 38AAB8FC  addi r5, r10, -0x4704
	ctx.r[5].s64 = ctx.r[10].s64 + -18180;
	// 82640DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640DEC: 390B90F0  addi r8, r11, -0x6f10
	ctx.r[8].s64 = ctx.r[11].s64 + -28432;
	// 82640DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640DF4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82640DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640E08: 386AD90C  addi r3, r10, -0x26f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9972;
	// 82640E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640E2C: 4BE25FF5  bl 0x82466e20
	ctx.lr = 0x82640E30;
	sub_82466E20(ctx, base);
	// 82640E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640E40 size=108
    let mut pc: u32 = 0x82640E40;
    'dispatch: loop {
        match pc {
            0x82640E40 => {
    //   block [0x82640E40..0x82640EAC)
	// 82640E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640E4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640E54: 38EB9108  addi r7, r11, -0x6ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -28408;
	// 82640E58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82640E5C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82640E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640E68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640E70: 386AD93C  addi r3, r10, -0x26c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9924;
	// 82640E74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640E98: 4BE25F89  bl 0x82466e20
	ctx.lr = 0x82640E9C;
	sub_82466E20(ctx, base);
	// 82640E9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640EB0 size=112
    let mut pc: u32 = 0x82640EB0;
    'dispatch: loop {
        match pc {
            0x82640EB0 => {
    //   block [0x82640EB0..0x82640F20)
	// 82640EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640EC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640EC4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640ECC: 390B9120  addi r8, r11, -0x6ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -28384;
	// 82640ED0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82640ED4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82640ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640EE8: 386AD96C  addi r3, r10, -0x2694
	ctx.r[3].s64 = ctx.r[10].s64 + -9876;
	// 82640EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640F0C: 4BE25F15  bl 0x82466e20
	ctx.lr = 0x82640F10;
	sub_82466E20(ctx, base);
	// 82640F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640F20 size=108
    let mut pc: u32 = 0x82640F20;
    'dispatch: loop {
        match pc {
            0x82640F20 => {
    //   block [0x82640F20..0x82640F8C)
	// 82640F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640F2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640F34: 38EB9168  addi r7, r11, -0x6e98
	ctx.r[7].s64 = ctx.r[11].s64 + -28312;
	// 82640F38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82640F3C: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 82640F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640F50: 386AD99C  addi r3, r10, -0x2664
	ctx.r[3].s64 = ctx.r[10].s64 + -9828;
	// 82640F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640F78: 4BE25EA9  bl 0x82466e20
	ctx.lr = 0x82640F7C;
	sub_82466E20(ctx, base);
	// 82640F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640F90 size=108
    let mut pc: u32 = 0x82640F90;
    'dispatch: loop {
        match pc {
            0x82640F90 => {
    //   block [0x82640F90..0x82640FFC)
	// 82640F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640F9C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640FA4: 38EB9198  addi r7, r11, -0x6e68
	ctx.r[7].s64 = ctx.r[11].s64 + -28264;
	// 82640FA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82640FAC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82640FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640FB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640FC0: 386AD9CC  addi r3, r10, -0x2634
	ctx.r[3].s64 = ctx.r[10].s64 + -9780;
	// 82640FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640FE8: 4BE25E39  bl 0x82466e20
	ctx.lr = 0x82640FEC;
	sub_82466E20(ctx, base);
	// 82640FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641000 size=112
    let mut pc: u32 = 0x82641000;
    'dispatch: loop {
        match pc {
            0x82641000 => {
    //   block [0x82641000..0x82641070)
	// 82641000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264100C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641014: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264101C: 390B91B0  addi r8, r11, -0x6e50
	ctx.r[8].s64 = ctx.r[11].s64 + -28240;
	// 82641020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641024: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82641028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264102C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641038: 386AD9FC  addi r3, r10, -0x2604
	ctx.r[3].s64 = ctx.r[10].s64 + -9732;
	// 8264103C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264104C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264105C: 4BE25DC5  bl 0x82466e20
	ctx.lr = 0x82641060;
	sub_82466E20(ctx, base);
	// 82641060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264106C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641070 size=112
    let mut pc: u32 = 0x82641070;
    'dispatch: loop {
        match pc {
            0x82641070 => {
    //   block [0x82641070..0x826410E0)
	// 82641070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264107C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82641080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641084: 392A738C  addi r9, r10, 0x738c
	ctx.r[9].s64 = ctx.r[10].s64 + 29580;
	// 82641088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264108C: 390B91E8  addi r8, r11, -0x6e18
	ctx.r[8].s64 = ctx.r[11].s64 + -28184;
	// 82641090: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82641094: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82641098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264109C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826410A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826410A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826410A8: 386ADA2C  addi r3, r10, -0x25d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9684;
	// 826410AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826410B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826410B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826410B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826410BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826410C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826410C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826410C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826410CC: 4BE25D55  bl 0x82466e20
	ctx.lr = 0x826410D0;
	sub_82466E20(ctx, base);
	// 826410D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826410D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826410D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826410DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826410E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826410E0 size=116
    let mut pc: u32 = 0x826410E0;
    'dispatch: loop {
        match pc {
            0x826410E0 => {
    //   block [0x826410E0..0x82641154)
	// 826410E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826410E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826410E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826410EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826410F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826410F4: 390B9290  addi r8, r11, -0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + -28016;
	// 826410F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826410FC: 392A7360  addi r9, r10, 0x7360
	ctx.r[9].s64 = ctx.r[10].s64 + 29536;
	// 82641100: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641104: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82641108: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8264110C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641124: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82641128: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8264112C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82641130: 386BDA5C  addi r3, r11, -0x25a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9636;
	// 82641134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82641138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264113C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641140: 4BE25CE1  bl 0x82466e20
	ctx.lr = 0x82641144;
	sub_82466E20(ctx, base);
	// 82641144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264114C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641158 size=112
    let mut pc: u32 = 0x82641158;
    'dispatch: loop {
        match pc {
            0x82641158 => {
    //   block [0x82641158..0x826411C8)
	// 82641158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641164: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82641168: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264116C: 392A73B8  addi r9, r10, 0x73b8
	ctx.r[9].s64 = ctx.r[10].s64 + 29624;
	// 82641170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641174: 390B92B0  addi r8, r11, -0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + -27984;
	// 82641178: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264117C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82641180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641190: 386ADA8C  addi r3, r10, -0x2574
	ctx.r[3].s64 = ctx.r[10].s64 + -9588;
	// 82641194: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82641198: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264119C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826411A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826411A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826411A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826411AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826411B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826411B4: 4BE25C6D  bl 0x82466e20
	ctx.lr = 0x826411B8;
	sub_82466E20(ctx, base);
	// 826411B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826411BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826411C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826411C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826411C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826411C8 size=112
    let mut pc: u32 = 0x826411C8;
    'dispatch: loop {
        match pc {
            0x826411C8 => {
    //   block [0x826411C8..0x82641238)
	// 826411C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826411CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826411D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826411D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826411D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826411DC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 826411E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826411E4: 390B9310  addi r8, r11, -0x6cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27888;
	// 826411E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826411EC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826411F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826411F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826411F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826411FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641200: 386ADABC  addi r3, r10, -0x2544
	ctx.r[3].s64 = ctx.r[10].s64 + -9540;
	// 82641204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264120C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264121C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641224: 4BE25BFD  bl 0x82466e20
	ctx.lr = 0x82641228;
	sub_82466E20(ctx, base);
	// 82641228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264122C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641238 size=112
    let mut pc: u32 = 0x82641238;
    'dispatch: loop {
        match pc {
            0x82641238 => {
    //   block [0x82641238..0x826412A8)
	// 82641238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264123C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641248: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264124C: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 82641250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641254: 390B9328  addi r8, r11, -0x6cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -27864;
	// 82641258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264125C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82641260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264126C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641270: 386ADAEC  addi r3, r10, -0x2514
	ctx.r[3].s64 = ctx.r[10].s64 + -9492;
	// 82641274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264127C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264128C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641294: 4BE25B8D  bl 0x82466e20
	ctx.lr = 0x82641298;
	sub_82466E20(ctx, base);
	// 82641298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264129C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826412A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826412A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826412A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826412A8 size=112
    let mut pc: u32 = 0x826412A8;
    'dispatch: loop {
        match pc {
            0x826412A8 => {
    //   block [0x826412A8..0x82641318)
	// 826412A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826412AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826412B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826412B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826412B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826412BC: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 826412C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826412C4: 390B9370  addi r8, r11, -0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + -27792;
	// 826412C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826412CC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826412D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826412D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826412D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826412DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826412E0: 386ADB1C  addi r3, r10, -0x24e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9444;
	// 826412E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826412E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826412EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826412F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826412F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826412F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826412FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641304: 4BE25B1D  bl 0x82466e20
	ctx.lr = 0x82641308;
	sub_82466E20(ctx, base);
	// 82641308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264130C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641318 size=112
    let mut pc: u32 = 0x82641318;
    'dispatch: loop {
        match pc {
            0x82641318 => {
    //   block [0x82641318..0x82641388)
	// 82641318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264131C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641328: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264132C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641334: 390B93D0  addi r8, r11, -0x6c30
	ctx.r[8].s64 = ctx.r[11].s64 + -27696;
	// 82641338: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264133C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82641340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641344: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264134C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641350: 386ADB4C  addi r3, r10, -0x24b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9396;
	// 82641354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264135C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264136C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641374: 4BE25AAD  bl 0x82466e20
	ctx.lr = 0x82641378;
	sub_82466E20(ctx, base);
	// 82641378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264137C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641388 size=112
    let mut pc: u32 = 0x82641388;
    'dispatch: loop {
        match pc {
            0x82641388 => {
    //   block [0x82641388..0x826413F8)
	// 82641388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264138C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641398: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264139C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 826413A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826413A4: 390B9430  addi r8, r11, -0x6bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -27600;
	// 826413A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826413AC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826413B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826413B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826413B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826413BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826413C0: 386ADB7C  addi r3, r10, -0x2484
	ctx.r[3].s64 = ctx.r[10].s64 + -9348;
	// 826413C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826413C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826413CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826413D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826413D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826413D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826413DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826413E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826413E4: 4BE25A3D  bl 0x82466e20
	ctx.lr = 0x826413E8;
	sub_82466E20(ctx, base);
	// 826413E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826413EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826413F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826413F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826413F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826413F8 size=112
    let mut pc: u32 = 0x826413F8;
    'dispatch: loop {
        match pc {
            0x826413F8 => {
    //   block [0x826413F8..0x82641468)
	// 826413F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826413FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641408: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264140C: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 82641410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641414: 390B9490  addi r8, r11, -0x6b70
	ctx.r[8].s64 = ctx.r[11].s64 + -27504;
	// 82641418: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8264141C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82641420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264142C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641430: 386ADBAC  addi r3, r10, -0x2454
	ctx.r[3].s64 = ctx.r[10].s64 + -9300;
	// 82641434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264144C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641454: 4BE259CD  bl 0x82466e20
	ctx.lr = 0x82641458;
	sub_82466E20(ctx, base);
	// 82641458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264145C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641468 size=112
    let mut pc: u32 = 0x82641468;
    'dispatch: loop {
        match pc {
            0x82641468 => {
    //   block [0x82641468..0x826414D8)
	// 82641468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264146C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641474: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82641478: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8264147C: 38EA9550  addi r7, r10, -0x6ab0
	ctx.r[7].s64 = ctx.r[10].s64 + -27312;
	// 82641480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641484: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82641488: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8264148C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641490: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641494: 396B73D0  addi r11, r11, 0x73d0
	ctx.r[11].s64 = ctx.r[11].s64 + 29648;
	// 82641498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264149C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826414A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826414A4: 386ADBDC  addi r3, r10, -0x2424
	ctx.r[3].s64 = ctx.r[10].s64 + -9252;
	// 826414A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826414AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826414B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826414B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826414B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826414BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826414C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826414C4: 4BE2595D  bl 0x82466e20
	ctx.lr = 0x826414C8;
	sub_82466E20(ctx, base);
	// 826414C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826414CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826414D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826414D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826414D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826414D8 size=112
    let mut pc: u32 = 0x826414D8;
    'dispatch: loop {
        match pc {
            0x826414D8 => {
    //   block [0x826414D8..0x82641548)
	// 826414D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826414DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826414E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826414E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826414E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826414EC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826414F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826414F4: 390B9718  addi r8, r11, -0x68e8
	ctx.r[8].s64 = ctx.r[11].s64 + -26856;
	// 826414F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826414FC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82641500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264150C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641510: 386ADC0C  addi r3, r10, -0x23f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9204;
	// 82641514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641524: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82641528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641534: 4BE258ED  bl 0x82466e20
	ctx.lr = 0x82641538;
	sub_82466E20(ctx, base);
	// 82641538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264153C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641548 size=112
    let mut pc: u32 = 0x82641548;
    'dispatch: loop {
        match pc {
            0x82641548 => {
    //   block [0x82641548..0x826415B8)
	// 82641548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641558: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264155C: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 82641560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641564: 390B9730  addi r8, r11, -0x68d0
	ctx.r[8].s64 = ctx.r[11].s64 + -26832;
	// 82641568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264156C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82641570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641580: 386ADC3C  addi r3, r10, -0x23c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9156;
	// 82641584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641594: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82641598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826415A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826415A4: 4BE2587D  bl 0x82466e20
	ctx.lr = 0x826415A8;
	sub_82466E20(ctx, base);
	// 826415A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826415AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826415B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826415B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826415B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826415B8 size=112
    let mut pc: u32 = 0x826415B8;
    'dispatch: loop {
        match pc {
            0x826415B8 => {
    //   block [0x826415B8..0x82641628)
	// 826415B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826415BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826415C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826415C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826415C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826415CC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826415D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826415D4: 390B9748  addi r8, r11, -0x68b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26808;
	// 826415D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826415DC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826415E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826415E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826415E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826415EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826415F0: 386ADC6C  addi r3, r10, -0x2394
	ctx.r[3].s64 = ctx.r[10].s64 + -9108;
	// 826415F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826415F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826415FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264160C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641614: 4BE2580D  bl 0x82466e20
	ctx.lr = 0x82641618;
	sub_82466E20(ctx, base);
	// 82641618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264161C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641628 size=108
    let mut pc: u32 = 0x82641628;
    'dispatch: loop {
        match pc {
            0x82641628 => {
    //   block [0x82641628..0x82641694)
	// 82641628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641634: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264163C: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 82641640: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641644: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82641648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264164C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641658: 386ADC9C  addi r3, r10, -0x2364
	ctx.r[3].s64 = ctx.r[10].s64 + -9060;
	// 8264165C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264167C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641680: 4BE257A1  bl 0x82466e20
	ctx.lr = 0x82641684;
	sub_82466E20(ctx, base);
	// 82641684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264168C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641698 size=112
    let mut pc: u32 = 0x82641698;
    'dispatch: loop {
        match pc {
            0x82641698 => {
    //   block [0x82641698..0x82641708)
	// 82641698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826416A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826416A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826416A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826416AC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826416B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826416B4: 390B97A8  addi r8, r11, -0x6858
	ctx.r[8].s64 = ctx.r[11].s64 + -26712;
	// 826416B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826416BC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826416C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826416C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826416C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826416CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826416D0: 386ADCCC  addi r3, r10, -0x2334
	ctx.r[3].s64 = ctx.r[10].s64 + -9012;
	// 826416D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826416D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826416DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826416E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826416E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826416E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826416EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826416F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826416F4: 4BE2572D  bl 0x82466e20
	ctx.lr = 0x826416F8;
	sub_82466E20(ctx, base);
	// 826416F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826416FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641708 size=108
    let mut pc: u32 = 0x82641708;
    'dispatch: loop {
        match pc {
            0x82641708 => {
    //   block [0x82641708..0x82641774)
	// 82641708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641714: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264171C: 38EB97C0  addi r7, r11, -0x6840
	ctx.r[7].s64 = ctx.r[11].s64 + -26688;
	// 82641720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641724: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82641728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264172C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641738: 386ADCFC  addi r3, r10, -0x2304
	ctx.r[3].s64 = ctx.r[10].s64 + -8964;
	// 8264173C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264175C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641760: 4BE256C1  bl 0x82466e20
	ctx.lr = 0x82641764;
	sub_82466E20(ctx, base);
	// 82641764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264176C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641778 size=108
    let mut pc: u32 = 0x82641778;
    'dispatch: loop {
        match pc {
            0x82641778 => {
    //   block [0x82641778..0x826417E4)
	// 82641778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641784: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264178C: 38EB97F0  addi r7, r11, -0x6810
	ctx.r[7].s64 = ctx.r[11].s64 + -26640;
	// 82641790: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82641794: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82641798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264179C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826417A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826417A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826417A8: 386ADD2C  addi r3, r10, -0x22d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8916;
	// 826417AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826417B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826417B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826417B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826417BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826417C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826417C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826417C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826417CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826417D0: 4BE25651  bl 0x82466e20
	ctx.lr = 0x826417D4;
	sub_82466E20(ctx, base);
	// 826417D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826417D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826417DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826417E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826417E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826417E8 size=112
    let mut pc: u32 = 0x826417E8;
    'dispatch: loop {
        match pc {
            0x826417E8 => {
    //   block [0x826417E8..0x82641858)
	// 826417E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826417EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826417F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826417F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826417F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826417FC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641804: 390B9838  addi r8, r11, -0x67c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26568;
	// 82641808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264180C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82641810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641820: 386ADD5C  addi r3, r10, -0x22a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8868;
	// 82641824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264182C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264183C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641844: 4BE255DD  bl 0x82466e20
	ctx.lr = 0x82641848;
	sub_82466E20(ctx, base);
	// 82641848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264184C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641858 size=112
    let mut pc: u32 = 0x82641858;
    'dispatch: loop {
        match pc {
            0x82641858 => {
    //   block [0x82641858..0x826418C8)
	// 82641858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264185C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264186C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641874: 390B9880  addi r8, r11, -0x6780
	ctx.r[8].s64 = ctx.r[11].s64 + -26496;
	// 82641878: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264187C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82641880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264188C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641890: 386ADD8C  addi r3, r10, -0x2274
	ctx.r[3].s64 = ctx.r[10].s64 + -8820;
	// 82641894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826418A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826418A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826418A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826418AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826418B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826418B4: 4BE2556D  bl 0x82466e20
	ctx.lr = 0x826418B8;
	sub_82466E20(ctx, base);
	// 826418B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826418BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826418C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826418C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826418C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826418C8 size=108
    let mut pc: u32 = 0x826418C8;
    'dispatch: loop {
        match pc {
            0x826418C8 => {
    //   block [0x826418C8..0x82641934)
	// 826418C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826418CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826418D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826418D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826418D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826418DC: 38EB9910  addi r7, r11, -0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26352;
	// 826418E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826418E4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826418E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826418EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826418F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826418F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826418F8: 386ADDBC  addi r3, r10, -0x2244
	ctx.r[3].s64 = ctx.r[10].s64 + -8772;
	// 826418FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264191C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641920: 4BE25501  bl 0x82466e20
	ctx.lr = 0x82641924;
	sub_82466E20(ctx, base);
	// 82641924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641938 size=108
    let mut pc: u32 = 0x82641938;
    'dispatch: loop {
        match pc {
            0x82641938 => {
    //   block [0x82641938..0x826419A4)
	// 82641938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641944: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264194C: 38EB9958  addi r7, r11, -0x66a8
	ctx.r[7].s64 = ctx.r[11].s64 + -26280;
	// 82641950: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641954: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82641958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264195C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641968: 386ADDEC  addi r3, r10, -0x2214
	ctx.r[3].s64 = ctx.r[10].s64 + -8724;
	// 8264196C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264197C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264198C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641990: 4BE25491  bl 0x82466e20
	ctx.lr = 0x82641994;
	sub_82466E20(ctx, base);
	// 82641994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264199C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826419A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826419A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826419A8 size=108
    let mut pc: u32 = 0x826419A8;
    'dispatch: loop {
        match pc {
            0x826419A8 => {
    //   block [0x826419A8..0x82641A14)
	// 826419A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826419AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826419B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826419B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826419B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826419BC: 38EB9988  addi r7, r11, -0x6678
	ctx.r[7].s64 = ctx.r[11].s64 + -26232;
	// 826419C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826419C4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826419C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826419CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826419D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826419D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826419D8: 386ADE1C  addi r3, r10, -0x21e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8676;
	// 826419DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826419E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826419E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826419E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826419EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826419F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826419F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826419F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826419FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641A00: 4BE25421  bl 0x82466e20
	ctx.lr = 0x82641A04;
	sub_82466E20(ctx, base);
	// 82641A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641A18 size=112
    let mut pc: u32 = 0x82641A18;
    'dispatch: loop {
        match pc {
            0x82641A18 => {
    //   block [0x82641A18..0x82641A88)
	// 82641A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641A24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641A2C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641A34: 390B99B8  addi r8, r11, -0x6648
	ctx.r[8].s64 = ctx.r[11].s64 + -26184;
	// 82641A38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641A3C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82641A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641A50: 386ADE4C  addi r3, r10, -0x21b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8628;
	// 82641A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641A74: 4BE253AD  bl 0x82466e20
	ctx.lr = 0x82641A78;
	sub_82466E20(ctx, base);
	// 82641A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641A88 size=112
    let mut pc: u32 = 0x82641A88;
    'dispatch: loop {
        match pc {
            0x82641A88 => {
    //   block [0x82641A88..0x82641AF8)
	// 82641A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641A94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641A9C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641AA4: 390B99E8  addi r8, r11, -0x6618
	ctx.r[8].s64 = ctx.r[11].s64 + -26136;
	// 82641AA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641AAC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82641AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641AC0: 386ADE7C  addi r3, r10, -0x2184
	ctx.r[3].s64 = ctx.r[10].s64 + -8580;
	// 82641AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641AE4: 4BE2533D  bl 0x82466e20
	ctx.lr = 0x82641AE8;
	sub_82466E20(ctx, base);
	// 82641AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641AF8 size=112
    let mut pc: u32 = 0x82641AF8;
    'dispatch: loop {
        match pc {
            0x82641AF8 => {
    //   block [0x82641AF8..0x82641B68)
	// 82641AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641B0C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641B14: 390B9A00  addi r8, r11, -0x6600
	ctx.r[8].s64 = ctx.r[11].s64 + -26112;
	// 82641B18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641B1C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82641B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641B30: 386ADEAC  addi r3, r10, -0x2154
	ctx.r[3].s64 = ctx.r[10].s64 + -8532;
	// 82641B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641B54: 4BE252CD  bl 0x82466e20
	ctx.lr = 0x82641B58;
	sub_82466E20(ctx, base);
	// 82641B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641B68 size=108
    let mut pc: u32 = 0x82641B68;
    'dispatch: loop {
        match pc {
            0x82641B68 => {
    //   block [0x82641B68..0x82641BD4)
	// 82641B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641B74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641B7C: 38EB9A18  addi r7, r11, -0x65e8
	ctx.r[7].s64 = ctx.r[11].s64 + -26088;
	// 82641B80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641B84: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82641B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641B98: 386ADEDC  addi r3, r10, -0x2124
	ctx.r[3].s64 = ctx.r[10].s64 + -8484;
	// 82641B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641BC0: 4BE25261  bl 0x82466e20
	ctx.lr = 0x82641BC4;
	sub_82466E20(ctx, base);
	// 82641BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641BD8 size=112
    let mut pc: u32 = 0x82641BD8;
    'dispatch: loop {
        match pc {
            0x82641BD8 => {
    //   block [0x82641BD8..0x82641C48)
	// 82641BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641BE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641BE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641BEC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641BF4: 390B9A48  addi r8, r11, -0x65b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26040;
	// 82641BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641BFC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82641C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641C10: 386ADF0C  addi r3, r10, -0x20f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8436;
	// 82641C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641C34: 4BE251ED  bl 0x82466e20
	ctx.lr = 0x82641C38;
	sub_82466E20(ctx, base);
	// 82641C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641C48 size=108
    let mut pc: u32 = 0x82641C48;
    'dispatch: loop {
        match pc {
            0x82641C48 => {
    //   block [0x82641C48..0x82641CB4)
	// 82641C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641C54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641C5C: 38EB9A60  addi r7, r11, -0x65a0
	ctx.r[7].s64 = ctx.r[11].s64 + -26016;
	// 82641C60: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82641C64: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82641C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641C6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641C78: 386ADF3C  addi r3, r10, -0x20c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8388;
	// 82641C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641CA0: 4BE25181  bl 0x82466e20
	ctx.lr = 0x82641CA4;
	sub_82466E20(ctx, base);
	// 82641CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641CB8 size=112
    let mut pc: u32 = 0x82641CB8;
    'dispatch: loop {
        match pc {
            0x82641CB8 => {
    //   block [0x82641CB8..0x82641D28)
	// 82641CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641CC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641CC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641CCC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641CD4: 390B9B50  addi r8, r11, -0x64b0
	ctx.r[8].s64 = ctx.r[11].s64 + -25776;
	// 82641CD8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82641CDC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82641CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641CF0: 386ADF6C  addi r3, r10, -0x2094
	ctx.r[3].s64 = ctx.r[10].s64 + -8340;
	// 82641CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641D14: 4BE2510D  bl 0x82466e20
	ctx.lr = 0x82641D18;
	sub_82466E20(ctx, base);
	// 82641D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641D28 size=108
    let mut pc: u32 = 0x82641D28;
    'dispatch: loop {
        match pc {
            0x82641D28 => {
    //   block [0x82641D28..0x82641D94)
	// 82641D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641D34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641D38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641D3C: 38EB9D00  addi r7, r11, -0x6300
	ctx.r[7].s64 = ctx.r[11].s64 + -25344;
	// 82641D40: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82641D44: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82641D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641D4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641D50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641D58: 386ADF9C  addi r3, r10, -0x2064
	ctx.r[3].s64 = ctx.r[10].s64 + -8292;
	// 82641D5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641D80: 4BE250A1  bl 0x82466e20
	ctx.lr = 0x82641D84;
	sub_82466E20(ctx, base);
	// 82641D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641D98 size=112
    let mut pc: u32 = 0x82641D98;
    'dispatch: loop {
        match pc {
            0x82641D98 => {
    //   block [0x82641D98..0x82641E08)
	// 82641D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641DA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641DA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641DAC: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641DB4: 390B9E98  addi r8, r11, -0x6168
	ctx.r[8].s64 = ctx.r[11].s64 + -24936;
	// 82641DB8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 82641DBC: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82641DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641DD0: 386ADFCC  addi r3, r10, -0x2034
	ctx.r[3].s64 = ctx.r[10].s64 + -8244;
	// 82641DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641DF4: 4BE2502D  bl 0x82466e20
	ctx.lr = 0x82641DF8;
	sub_82466E20(ctx, base);
	// 82641DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641E08 size=100
    let mut pc: u32 = 0x82641E08;
    'dispatch: loop {
        match pc {
            0x82641E08 => {
    //   block [0x82641E08..0x82641E6C)
	// 82641E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641E14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641E1C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641E28: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82641E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641E3C: 386ADFFC  addi r3, r10, -0x2004
	ctx.r[3].s64 = ctx.r[10].s64 + -8196;
	// 82641E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641E44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641E48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82641E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641E50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82641E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641E58: 4BE24FC9  bl 0x82466e20
	ctx.lr = 0x82641E5C;
	sub_82466E20(ctx, base);
	// 82641E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641E70 size=112
    let mut pc: u32 = 0x82641E70;
    'dispatch: loop {
        match pc {
            0x82641E70 => {
    //   block [0x82641E70..0x82641EE0)
	// 82641E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641E7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E80: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641E84: 38AADFFC  addi r5, r10, -0x2004
	ctx.r[5].s64 = ctx.r[10].s64 + -8196;
	// 82641E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641E8C: 390BA0F0  addi r8, r11, -0x5f10
	ctx.r[8].s64 = ctx.r[11].s64 + -24336;
	// 82641E90: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82641E94: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82641E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641E9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641EA8: 386AE02C  addi r3, r10, -0x1fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -8148;
	// 82641EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641ECC: 4BE24F55  bl 0x82466e20
	ctx.lr = 0x82641ED0;
	sub_82466E20(ctx, base);
	// 82641ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641EE0 size=100
    let mut pc: u32 = 0x82641EE0;
    'dispatch: loop {
        match pc {
            0x82641EE0 => {
    //   block [0x82641EE0..0x82641F44)
	// 82641EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641EF4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641F00: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82641F04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641F14: 386AE05C  addi r3, r10, -0x1fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -8100;
	// 82641F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641F20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82641F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82641F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641F30: 4BE24EF1  bl 0x82466e20
	ctx.lr = 0x82641F34;
	sub_82466E20(ctx, base);
	// 82641F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641F48 size=108
    let mut pc: u32 = 0x82641F48;
    'dispatch: loop {
        match pc {
            0x82641F48 => {
    //   block [0x82641F48..0x82641FB4)
	// 82641F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641F54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641F5C: 38EBA168  addi r7, r11, -0x5e98
	ctx.r[7].s64 = ctx.r[11].s64 + -24216;
	// 82641F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82641F64: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82641F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641F6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641F78: 386AE08C  addi r3, r10, -0x1f74
	ctx.r[3].s64 = ctx.r[10].s64 + -8052;
	// 82641F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641FA0: 4BE24E81  bl 0x82466e20
	ctx.lr = 0x82641FA4;
	sub_82466E20(ctx, base);
	// 82641FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641FB8 size=112
    let mut pc: u32 = 0x82641FB8;
    'dispatch: loop {
        match pc {
            0x82641FB8 => {
    //   block [0x82641FB8..0x82642028)
	// 82641FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641FC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641FC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641FCC: 38AAE05C  addi r5, r10, -0x1fa4
	ctx.r[5].s64 = ctx.r[10].s64 + -8100;
	// 82641FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641FD4: 390BA1B0  addi r8, r11, -0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + -24144;
	// 82641FD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641FDC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82641FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641FF0: 386AE0BC  addi r3, r10, -0x1f44
	ctx.r[3].s64 = ctx.r[10].s64 + -8004;
	// 82641FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642014: 4BE24E0D  bl 0x82466e20
	ctx.lr = 0x82642018;
	sub_82466E20(ctx, base);
	// 82642018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642028 size=100
    let mut pc: u32 = 0x82642028;
    'dispatch: loop {
        match pc {
            0x82642028 => {
    //   block [0x82642028..0x8264208C)
	// 82642028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264203C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642048: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8264204C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264205C: 386AE0EC  addi r3, r10, -0x1f14
	ctx.r[3].s64 = ctx.r[10].s64 + -7956;
	// 82642060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642064: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642068: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642070: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642078: 4BE24DA9  bl 0x82466e20
	ctx.lr = 0x8264207C;
	sub_82466E20(ctx, base);
	// 8264207C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642090 size=100
    let mut pc: u32 = 0x82642090;
    'dispatch: loop {
        match pc {
            0x82642090 => {
    //   block [0x82642090..0x826420F4)
	// 82642090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264209C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826420A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826420A4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826420A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826420AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826420B0: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826420B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826420B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826420BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826420C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826420C4: 386AE11C  addi r3, r10, -0x1ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -7908;
	// 826420C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826420CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826420D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826420D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826420D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826420DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826420E0: 4BE24D41  bl 0x82466e20
	ctx.lr = 0x826420E4;
	sub_82466E20(ctx, base);
	// 826420E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826420E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826420EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826420F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826420F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826420F8 size=112
    let mut pc: u32 = 0x826420F8;
    'dispatch: loop {
        match pc {
            0x826420F8 => {
    //   block [0x826420F8..0x82642168)
	// 826420F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826420FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642108: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264210C: 38AAE0EC  addi r5, r10, -0x1f14
	ctx.r[5].s64 = ctx.r[10].s64 + -7956;
	// 82642110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642114: 390BA1E0  addi r8, r11, -0x5e20
	ctx.r[8].s64 = ctx.r[11].s64 + -24096;
	// 82642118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264211C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82642120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264212C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642130: 386AE14C  addi r3, r10, -0x1eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7860;
	// 82642134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264213C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264214C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642154: 4BE24CCD  bl 0x82466e20
	ctx.lr = 0x82642158;
	sub_82466E20(ctx, base);
	// 82642158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642168 size=112
    let mut pc: u32 = 0x82642168;
    'dispatch: loop {
        match pc {
            0x82642168 => {
    //   block [0x82642168..0x826421D8)
	// 82642168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642178: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264217C: 38AAE11C  addi r5, r10, -0x1ee4
	ctx.r[5].s64 = ctx.r[10].s64 + -7908;
	// 82642180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642184: 390BA240  addi r8, r11, -0x5dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -24000;
	// 82642188: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264218C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82642190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264219C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826421A0: 386AE17C  addi r3, r10, -0x1e84
	ctx.r[3].s64 = ctx.r[10].s64 + -7812;
	// 826421A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826421A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826421AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826421B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826421B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826421B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826421BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826421C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826421C4: 4BE24C5D  bl 0x82466e20
	ctx.lr = 0x826421C8;
	sub_82466E20(ctx, base);
	// 826421C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826421CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826421D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826421D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826421D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826421D8 size=100
    let mut pc: u32 = 0x826421D8;
    'dispatch: loop {
        match pc {
            0x826421D8 => {
    //   block [0x826421D8..0x8264223C)
	// 826421D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826421DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826421E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826421E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826421E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826421EC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826421F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826421F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826421F8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826421FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264220C: 386AE1AC  addi r3, r10, -0x1e54
	ctx.r[3].s64 = ctx.r[10].s64 + -7764;
	// 82642210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642218: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264221C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642220: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642228: 4BE24BF9  bl 0x82466e20
	ctx.lr = 0x8264222C;
	sub_82466E20(ctx, base);
	// 8264222C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642240 size=112
    let mut pc: u32 = 0x82642240;
    'dispatch: loop {
        match pc {
            0x82642240 => {
    //   block [0x82642240..0x826422B0)
	// 82642240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264224C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642250: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642254: 38AAE1AC  addi r5, r10, -0x1e54
	ctx.r[5].s64 = ctx.r[10].s64 + -7764;
	// 82642258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264225C: 390BA2A0  addi r8, r11, -0x5d60
	ctx.r[8].s64 = ctx.r[11].s64 + -23904;
	// 82642260: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82642264: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82642268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264226C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642278: 386AE1DC  addi r3, r10, -0x1e24
	ctx.r[3].s64 = ctx.r[10].s64 + -7716;
	// 8264227C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264228C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264229C: 4BE24B85  bl 0x82466e20
	ctx.lr = 0x826422A0;
	sub_82466E20(ctx, base);
	// 826422A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826422A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826422A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826422AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826422B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826422B0 size=108
    let mut pc: u32 = 0x826422B0;
    'dispatch: loop {
        match pc {
            0x826422B0 => {
    //   block [0x826422B0..0x8264231C)
	// 826422B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826422B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826422B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826422BC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826422C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826422C4: 38EBA390  addi r7, r11, -0x5c70
	ctx.r[7].s64 = ctx.r[11].s64 + -23664;
	// 826422C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826422CC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826422D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826422D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826422D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826422DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826422E0: 386AE20C  addi r3, r10, -0x1df4
	ctx.r[3].s64 = ctx.r[10].s64 + -7668;
	// 826422E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826422E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826422EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826422F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826422F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826422F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826422FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642308: 4BE24B19  bl 0x82466e20
	ctx.lr = 0x8264230C;
	sub_82466E20(ctx, base);
	// 8264230C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642320 size=108
    let mut pc: u32 = 0x82642320;
    'dispatch: loop {
        match pc {
            0x82642320 => {
    //   block [0x82642320..0x8264238C)
	// 82642320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264232C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642334: 38EBA480  addi r7, r11, -0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + -23424;
	// 82642338: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264233C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82642340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642344: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264234C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642350: 386AE23C  addi r3, r10, -0x1dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -7620;
	// 82642354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264235C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264236C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642378: 4BE24AA9  bl 0x82466e20
	ctx.lr = 0x8264237C;
	sub_82466E20(ctx, base);
	// 8264237C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642390 size=108
    let mut pc: u32 = 0x82642390;
    'dispatch: loop {
        match pc {
            0x82642390 => {
    //   block [0x82642390..0x826423FC)
	// 82642390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264239C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826423A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826423A4: 38EBA4C8  addi r7, r11, -0x5b38
	ctx.r[7].s64 = ctx.r[11].s64 + -23352;
	// 826423A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826423AC: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826423B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826423B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826423B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826423BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826423C0: 386AE26C  addi r3, r10, -0x1d94
	ctx.r[3].s64 = ctx.r[10].s64 + -7572;
	// 826423C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826423C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826423CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826423D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826423D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826423D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826423DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826423E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826423E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826423E8: 4BE24A39  bl 0x82466e20
	ctx.lr = 0x826423EC;
	sub_82466E20(ctx, base);
	// 826423EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826423F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826423F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826423F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642400 size=108
    let mut pc: u32 = 0x82642400;
    'dispatch: loop {
        match pc {
            0x82642400 => {
    //   block [0x82642400..0x8264246C)
	// 82642400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264240C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642414: 38EBA5A0  addi r7, r11, -0x5a60
	ctx.r[7].s64 = ctx.r[11].s64 + -23136;
	// 82642418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264241C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82642420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264242C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642430: 386AE29C  addi r3, r10, -0x1d64
	ctx.r[3].s64 = ctx.r[10].s64 + -7524;
	// 82642434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264243C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264244C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642458: 4BE249C9  bl 0x82466e20
	ctx.lr = 0x8264245C;
	sub_82466E20(ctx, base);
	// 8264245C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642470 size=100
    let mut pc: u32 = 0x82642470;
    'dispatch: loop {
        match pc {
            0x82642470 => {
    //   block [0x82642470..0x826424D4)
	// 82642470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264247C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642484: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264248C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642490: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82642494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264249C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826424A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826424A4: 386AE2CC  addi r3, r10, -0x1d34
	ctx.r[3].s64 = ctx.r[10].s64 + -7476;
	// 826424A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826424AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826424B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826424B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826424B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826424BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826424C0: 4BE24961  bl 0x82466e20
	ctx.lr = 0x826424C4;
	sub_82466E20(ctx, base);
	// 826424C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826424C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826424CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826424D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826424D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826424D8 size=112
    let mut pc: u32 = 0x826424D8;
    'dispatch: loop {
        match pc {
            0x826424D8 => {
    //   block [0x826424D8..0x82642548)
	// 826424D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826424DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826424E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826424E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826424E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826424EC: 38AAE2CC  addi r5, r10, -0x1d34
	ctx.r[5].s64 = ctx.r[10].s64 + -7476;
	// 826424F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826424F4: 390BA5B8  addi r8, r11, -0x5a48
	ctx.r[8].s64 = ctx.r[11].s64 + -23112;
	// 826424F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826424FC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82642500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264250C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642510: 386AE2FC  addi r3, r10, -0x1d04
	ctx.r[3].s64 = ctx.r[10].s64 + -7428;
	// 82642514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264251C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264252C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642534: 4BE248ED  bl 0x82466e20
	ctx.lr = 0x82642538;
	sub_82466E20(ctx, base);
	// 82642538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642548 size=108
    let mut pc: u32 = 0x82642548;
    'dispatch: loop {
        match pc {
            0x82642548 => {
    //   block [0x82642548..0x826425B4)
	// 82642548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642554: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264255C: 38EBA600  addi r7, r11, -0x5a00
	ctx.r[7].s64 = ctx.r[11].s64 + -23040;
	// 82642560: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82642564: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82642568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264256C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642578: 386AE32C  addi r3, r10, -0x1cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -7380;
	// 8264257C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264258C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264259C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826425A0: 4BE24881  bl 0x82466e20
	ctx.lr = 0x826425A4;
	sub_82466E20(ctx, base);
	// 826425A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826425A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826425AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826425B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826425B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826425B8 size=112
    let mut pc: u32 = 0x826425B8;
    'dispatch: loop {
        match pc {
            0x826425B8 => {
    //   block [0x826425B8..0x82642628)
	// 826425B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826425BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826425C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826425C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826425C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826425CC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826425D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826425D4: 390BA648  addi r8, r11, -0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22968;
	// 826425D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826425DC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826425E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826425E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826425E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826425EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826425F0: 386AE35C  addi r3, r10, -0x1ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -7332;
	// 826425F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826425F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826425FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264260C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642614: 4BE2480D  bl 0x82466e20
	ctx.lr = 0x82642618;
	sub_82466E20(ctx, base);
	// 82642618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264261C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642628 size=108
    let mut pc: u32 = 0x82642628;
    'dispatch: loop {
        match pc {
            0x82642628 => {
    //   block [0x82642628..0x82642694)
	// 82642628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264262C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642634: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264263C: 38EBA660  addi r7, r11, -0x59a0
	ctx.r[7].s64 = ctx.r[11].s64 + -22944;
	// 82642640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82642644: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82642648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264264C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642658: 386AE38C  addi r3, r10, -0x1c74
	ctx.r[3].s64 = ctx.r[10].s64 + -7284;
	// 8264265C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264266C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264267C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642680: 4BE247A1  bl 0x82466e20
	ctx.lr = 0x82642684;
	sub_82466E20(ctx, base);
	// 82642684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264268C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642698 size=112
    let mut pc: u32 = 0x82642698;
    'dispatch: loop {
        match pc {
            0x82642698 => {
    //   block [0x82642698..0x82642708)
	// 82642698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264269C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826426A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826426A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826426A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826426AC: 38AAE35C  addi r5, r10, -0x1ca4
	ctx.r[5].s64 = ctx.r[10].s64 + -7332;
	// 826426B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826426B4: 390BA6A8  addi r8, r11, -0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + -22872;
	// 826426B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826426BC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826426C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826426C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826426C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826426CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826426D0: 386AE3BC  addi r3, r10, -0x1c44
	ctx.r[3].s64 = ctx.r[10].s64 + -7236;
	// 826426D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826426D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826426DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826426E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826426E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826426E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826426EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826426F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826426F4: 4BE2472D  bl 0x82466e20
	ctx.lr = 0x826426F8;
	sub_82466E20(ctx, base);
	// 826426F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826426FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642708 size=100
    let mut pc: u32 = 0x82642708;
    'dispatch: loop {
        match pc {
            0x82642708 => {
    //   block [0x82642708..0x8264276C)
	// 82642708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642714: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264271C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642728: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8264272C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264273C: 386AE3EC  addi r3, r10, -0x1c14
	ctx.r[3].s64 = ctx.r[10].s64 + -7188;
	// 82642740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264274C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642758: 4BE246C9  bl 0x82466e20
	ctx.lr = 0x8264275C;
	sub_82466E20(ctx, base);
	// 8264275C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642770 size=112
    let mut pc: u32 = 0x82642770;
    'dispatch: loop {
        match pc {
            0x82642770 => {
    //   block [0x82642770..0x826427E0)
	// 82642770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264277C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642780: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642784: 38AAE3EC  addi r5, r10, -0x1c14
	ctx.r[5].s64 = ctx.r[10].s64 + -7188;
	// 82642788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264278C: 390BA6C0  addi r8, r11, -0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + -22848;
	// 82642790: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82642794: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82642798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264279C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826427A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826427A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826427A8: 386AE41C  addi r3, r10, -0x1be4
	ctx.r[3].s64 = ctx.r[10].s64 + -7140;
	// 826427AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826427B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826427B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826427B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826427BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826427C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826427C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826427C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826427CC: 4BE24655  bl 0x82466e20
	ctx.lr = 0x826427D0;
	sub_82466E20(ctx, base);
	// 826427D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826427D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826427D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826427DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826427E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826427E0 size=108
    let mut pc: u32 = 0x826427E0;
    'dispatch: loop {
        match pc {
            0x826427E0 => {
    //   block [0x826427E0..0x8264284C)
	// 826427E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826427E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826427E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826427EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826427F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826427F4: 38EBA768  addi r7, r11, -0x5898
	ctx.r[7].s64 = ctx.r[11].s64 + -22680;
	// 826427F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826427FC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82642800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264280C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642810: 386AE44C  addi r3, r10, -0x1bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7092;
	// 82642814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264281C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264282C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642838: 4BE245E9  bl 0x82466e20
	ctx.lr = 0x8264283C;
	sub_82466E20(ctx, base);
	// 8264283C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642850 size=112
    let mut pc: u32 = 0x82642850;
    'dispatch: loop {
        match pc {
            0x82642850 => {
    //   block [0x82642850..0x826428C0)
	// 82642850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264285C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642860: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642864: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264286C: 390BA798  addi r8, r11, -0x5868
	ctx.r[8].s64 = ctx.r[11].s64 + -22632;
	// 82642870: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82642874: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82642878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264287C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642888: 386AE47C  addi r3, r10, -0x1b84
	ctx.r[3].s64 = ctx.r[10].s64 + -7044;
	// 8264288C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264289C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826428A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826428A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826428A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826428AC: 4BE24575  bl 0x82466e20
	ctx.lr = 0x826428B0;
	sub_82466E20(ctx, base);
	// 826428B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826428B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826428B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826428BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826428C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826428C0 size=112
    let mut pc: u32 = 0x826428C0;
    'dispatch: loop {
        match pc {
            0x826428C0 => {
    //   block [0x826428C0..0x82642930)
	// 826428C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826428C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826428C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826428CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826428D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826428D4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826428D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826428DC: 390BA7E0  addi r8, r11, -0x5820
	ctx.r[8].s64 = ctx.r[11].s64 + -22560;
	// 826428E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826428E4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826428E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826428EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826428F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826428F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826428F8: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 826428FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264291C: 4BE24505  bl 0x82466e20
	ctx.lr = 0x82642920;
	sub_82466E20(ctx, base);
	// 82642920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642930 size=100
    let mut pc: u32 = 0x82642930;
    'dispatch: loop {
        match pc {
            0x82642930 => {
    //   block [0x82642930..0x82642994)
	// 82642930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264293C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642944: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264294C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642950: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82642954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264295C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642964: 386AE4DC  addi r3, r10, -0x1b24
	ctx.r[3].s64 = ctx.r[10].s64 + -6948;
	// 82642968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264296C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82642974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264297C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642980: 4BE244A1  bl 0x82466e20
	ctx.lr = 0x82642984;
	sub_82466E20(ctx, base);
	// 82642984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264298C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642998 size=112
    let mut pc: u32 = 0x82642998;
    'dispatch: loop {
        match pc {
            0x82642998 => {
    //   block [0x82642998..0x82642A08)
	// 82642998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264299C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826429A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826429A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826429A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826429AC: 38AAE4DC  addi r5, r10, -0x1b24
	ctx.r[5].s64 = ctx.r[10].s64 + -6948;
	// 826429B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826429B4: 390BA828  addi r8, r11, -0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + -22488;
	// 826429B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826429BC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826429C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826429C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826429C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826429CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826429D0: 386AE50C  addi r3, r10, -0x1af4
	ctx.r[3].s64 = ctx.r[10].s64 + -6900;
	// 826429D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826429D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826429DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826429E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826429E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826429E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826429EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826429F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826429F4: 4BE2442D  bl 0x82466e20
	ctx.lr = 0x826429F8;
	sub_82466E20(ctx, base);
	// 826429F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826429FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642A08 size=112
    let mut pc: u32 = 0x82642A08;
    'dispatch: loop {
        match pc {
            0x82642A08 => {
    //   block [0x82642A08..0x82642A78)
	// 82642A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642A1C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642A24: 390BA870  addi r8, r11, -0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + -22416;
	// 82642A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642A2C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82642A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642A40: 386AE53C  addi r3, r10, -0x1ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -6852;
	// 82642A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642A64: 4BE243BD  bl 0x82466e20
	ctx.lr = 0x82642A68;
	sub_82466E20(ctx, base);
	// 82642A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642A78 size=112
    let mut pc: u32 = 0x82642A78;
    'dispatch: loop {
        match pc {
            0x82642A78 => {
    //   block [0x82642A78..0x82642AE8)
	// 82642A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642A8C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642A94: 390BA888  addi r8, r11, -0x5778
	ctx.r[8].s64 = ctx.r[11].s64 + -22392;
	// 82642A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642A9C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82642AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642AB0: 386AE56C  addi r3, r10, -0x1a94
	ctx.r[3].s64 = ctx.r[10].s64 + -6804;
	// 82642AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642AC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82642AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642AD4: 4BE2434D  bl 0x82466e20
	ctx.lr = 0x82642AD8;
	sub_82466E20(ctx, base);
	// 82642AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642AE8 size=112
    let mut pc: u32 = 0x82642AE8;
    'dispatch: loop {
        match pc {
            0x82642AE8 => {
    //   block [0x82642AE8..0x82642B58)
	// 82642AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642AFC: 38AAE53C  addi r5, r10, -0x1ac4
	ctx.r[5].s64 = ctx.r[10].s64 + -6852;
	// 82642B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642B04: 390BA8A0  addi r8, r11, -0x5760
	ctx.r[8].s64 = ctx.r[11].s64 + -22368;
	// 82642B08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82642B0C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82642B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642B20: 386AE59C  addi r3, r10, -0x1a64
	ctx.r[3].s64 = ctx.r[10].s64 + -6756;
	// 82642B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642B44: 4BE242DD  bl 0x82466e20
	ctx.lr = 0x82642B48;
	sub_82466E20(ctx, base);
	// 82642B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642B58 size=72
    let mut pc: u32 = 0x82642B58;
    'dispatch: loop {
        match pc {
            0x82642B58 => {
    //   block [0x82642B58..0x82642BA0)
	// 82642B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642B64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82642B68: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82642B6C: 38CB5D10  addi r6, r11, 0x5d10
	ctx.r[6].s64 = ctx.r[11].s64 + 23824;
	// 82642B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82642B74: 388B7428  addi r4, r11, 0x7428
	ctx.r[4].s64 = ctx.r[11].s64 + 29736;
	// 82642B78: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82642B7C: 386BE5CC  addi r3, r11, -0x1a34
	ctx.r[3].s64 = ctx.r[11].s64 + -6708;
	// 82642B80: 4BE38F09  bl 0x8247ba88
	ctx.lr = 0x82642B84;
	sub_8247BA88(ctx, base);
	// 82642B84: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82642B88: 386BCD98  addi r3, r11, -0x3268
	ctx.r[3].s64 = ctx.r[11].s64 + -12904;
	// 82642B8C: 4BEEFFAD  bl 0x82532b38
	ctx.lr = 0x82642B90;
	sub_82532B38(ctx, base);
	// 82642B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82642B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642BA0 size=108
    let mut pc: u32 = 0x82642BA0;
    'dispatch: loop {
        match pc {
            0x82642BA0 => {
    //   block [0x82642BA0..0x82642C0C)
	// 82642BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642BAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642BB4: 38EBC460  addi r7, r11, -0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + -15264;
	// 82642BB8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82642BBC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82642BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642BC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642BD0: 386AE5E4  addi r3, r10, -0x1a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6684;
	// 82642BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642BF8: 4BE24229  bl 0x82466e20
	ctx.lr = 0x82642BFC;
	sub_82466E20(ctx, base);
	// 82642BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642C10 size=108
    let mut pc: u32 = 0x82642C10;
    'dispatch: loop {
        match pc {
            0x82642C10 => {
    //   block [0x82642C10..0x82642C7C)
	// 82642C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642C1C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642C24: 38EBC4D8  addi r7, r11, -0x3b28
	ctx.r[7].s64 = ctx.r[11].s64 + -15144;
	// 82642C28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642C2C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82642C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642C34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642C40: 386AE614  addi r3, r10, -0x19ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6636;
	// 82642C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642C68: 4BE241B9  bl 0x82466e20
	ctx.lr = 0x82642C6C;
	sub_82466E20(ctx, base);
	// 82642C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642C80 size=108
    let mut pc: u32 = 0x82642C80;
    'dispatch: loop {
        match pc {
            0x82642C80 => {
    //   block [0x82642C80..0x82642CEC)
	// 82642C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642C8C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642C94: 38EBC508  addi r7, r11, -0x3af8
	ctx.r[7].s64 = ctx.r[11].s64 + -15096;
	// 82642C98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642C9C: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82642CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642CA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642CB0: 386AE644  addi r3, r10, -0x19bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6588;
	// 82642CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642CD8: 4BE24149  bl 0x82466e20
	ctx.lr = 0x82642CDC;
	sub_82466E20(ctx, base);
	// 82642CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642CF0 size=108
    let mut pc: u32 = 0x82642CF0;
    'dispatch: loop {
        match pc {
            0x82642CF0 => {
    //   block [0x82642CF0..0x82642D5C)
	// 82642CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642CFC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642D04: 38EBC538  addi r7, r11, -0x3ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -15048;
	// 82642D08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642D0C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82642D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642D14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642D20: 386AE674  addi r3, r10, -0x198c
	ctx.r[3].s64 = ctx.r[10].s64 + -6540;
	// 82642D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642D48: 4BE240D9  bl 0x82466e20
	ctx.lr = 0x82642D4C;
	sub_82466E20(ctx, base);
	// 82642D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642D60 size=112
    let mut pc: u32 = 0x82642D60;
    'dispatch: loop {
        match pc {
            0x82642D60 => {
    //   block [0x82642D60..0x82642DD0)
	// 82642D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642D74: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82642D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642D7C: 390BC568  addi r8, r11, -0x3a98
	ctx.r[8].s64 = ctx.r[11].s64 + -15000;
	// 82642D80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82642D84: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82642D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642D98: 386AE6A4  addi r3, r10, -0x195c
	ctx.r[3].s64 = ctx.r[10].s64 + -6492;
	// 82642D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642DBC: 4BE24065  bl 0x82466e20
	ctx.lr = 0x82642DC0;
	sub_82466E20(ctx, base);
	// 82642DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642DD0 size=108
    let mut pc: u32 = 0x82642DD0;
    'dispatch: loop {
        match pc {
            0x82642DD0 => {
    //   block [0x82642DD0..0x82642E3C)
	// 82642DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642DDC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642DE4: 38EBC5C8  addi r7, r11, -0x3a38
	ctx.r[7].s64 = ctx.r[11].s64 + -14904;
	// 82642DE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82642DEC: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82642DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642E00: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 82642E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642E28: 4BE23FF9  bl 0x82466e20
	ctx.lr = 0x82642E2C;
	sub_82466E20(ctx, base);
	// 82642E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642E40 size=112
    let mut pc: u32 = 0x82642E40;
    'dispatch: loop {
        match pc {
            0x82642E40 => {
    //   block [0x82642E40..0x82642EB0)
	// 82642E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642E4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642E50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642E54: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642E5C: 390BC628  addi r8, r11, -0x39d8
	ctx.r[8].s64 = ctx.r[11].s64 + -14808;
	// 82642E60: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82642E64: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82642E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642E78: 386AE704  addi r3, r10, -0x18fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6396;
	// 82642E7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642E9C: 4BE23F85  bl 0x82466e20
	ctx.lr = 0x82642EA0;
	sub_82466E20(ctx, base);
	// 82642EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642EB0 size=112
    let mut pc: u32 = 0x82642EB0;
    'dispatch: loop {
        match pc {
            0x82642EB0 => {
    //   block [0x82642EB0..0x82642F20)
	// 82642EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642EC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642EC4: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642ECC: 390BC6B8  addi r8, r11, -0x3948
	ctx.r[8].s64 = ctx.r[11].s64 + -14664;
	// 82642ED0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642ED4: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82642ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642EE8: 386AE734  addi r3, r10, -0x18cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6348;
	// 82642EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642F0C: 4BE23F15  bl 0x82466e20
	ctx.lr = 0x82642F10;
	sub_82466E20(ctx, base);
	// 82642F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642F20 size=108
    let mut pc: u32 = 0x82642F20;
    'dispatch: loop {
        match pc {
            0x82642F20 => {
    //   block [0x82642F20..0x82642F8C)
	// 82642F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642F2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642F34: 38EBC6D0  addi r7, r11, -0x3930
	ctx.r[7].s64 = ctx.r[11].s64 + -14640;
	// 82642F38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82642F3C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82642F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642F50: 386AE764  addi r3, r10, -0x189c
	ctx.r[3].s64 = ctx.r[10].s64 + -6300;
	// 82642F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642F78: 4BE23EA9  bl 0x82466e20
	ctx.lr = 0x82642F7C;
	sub_82466E20(ctx, base);
	// 82642F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642F90 size=112
    let mut pc: u32 = 0x82642F90;
    'dispatch: loop {
        match pc {
            0x82642F90 => {
    //   block [0x82642F90..0x82643000)
	// 82642F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642F9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642FA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642FA4: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642FAC: 390BC730  addi r8, r11, -0x38d0
	ctx.r[8].s64 = ctx.r[11].s64 + -14544;
	// 82642FB0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82642FB4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82642FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642FBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642FC8: 386AE794  addi r3, r10, -0x186c
	ctx.r[3].s64 = ctx.r[10].s64 + -6252;
	// 82642FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642FEC: 4BE23E35  bl 0x82466e20
	ctx.lr = 0x82642FF0;
	sub_82466E20(ctx, base);
	// 82642FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643000 size=108
    let mut pc: u32 = 0x82643000;
    'dispatch: loop {
        match pc {
            0x82643000 => {
    //   block [0x82643000..0x8264306C)
	// 82643000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264300C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643014: 38EBC7D8  addi r7, r11, -0x3828
	ctx.r[7].s64 = ctx.r[11].s64 + -14376;
	// 82643018: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264301C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82643020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264302C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643030: 386AE7C4  addi r3, r10, -0x183c
	ctx.r[3].s64 = ctx.r[10].s64 + -6204;
	// 82643034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264303C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264304C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643058: 4BE23DC9  bl 0x82466e20
	ctx.lr = 0x8264305C;
	sub_82466E20(ctx, base);
	// 8264305C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643070 size=108
    let mut pc: u32 = 0x82643070;
    'dispatch: loop {
        match pc {
            0x82643070 => {
    //   block [0x82643070..0x826430DC)
	// 82643070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264307C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643084: 38EBC7F0  addi r7, r11, -0x3810
	ctx.r[7].s64 = ctx.r[11].s64 + -14352;
	// 82643088: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264308C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82643090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264309C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826430A0: 386AE7F4  addi r3, r10, -0x180c
	ctx.r[3].s64 = ctx.r[10].s64 + -6156;
	// 826430A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826430A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826430AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826430B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826430B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826430B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826430BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826430C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826430C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826430C8: 4BE23D59  bl 0x82466e20
	ctx.lr = 0x826430CC;
	sub_82466E20(ctx, base);
	// 826430CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826430D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826430D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826430D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826430E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826430E0 size=112
    let mut pc: u32 = 0x826430E0;
    'dispatch: loop {
        match pc {
            0x826430E0 => {
    //   block [0x826430E0..0x82643150)
	// 826430E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826430E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826430E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826430EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826430F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826430F4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826430F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826430FC: 390BC850  addi r8, r11, -0x37b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14256;
	// 82643100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82643104: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82643108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264310C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82643114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643118: 386AE824  addi r3, r10, -0x17dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6108;
	// 8264311C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82643120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264312C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264313C: 4BE23CE5  bl 0x82466e20
	ctx.lr = 0x82643140;
	sub_82466E20(ctx, base);
	// 82643140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643150 size=108
    let mut pc: u32 = 0x82643150;
    'dispatch: loop {
        match pc {
            0x82643150 => {
    //   block [0x82643150..0x826431BC)
	// 82643150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264315C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643164: 38EBC868  addi r7, r11, -0x3798
	ctx.r[7].s64 = ctx.r[11].s64 + -14232;
	// 82643168: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264316C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82643170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264317C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643180: 386AE854  addi r3, r10, -0x17ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6060;
	// 82643184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264318C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264319C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826431A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826431A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826431A8: 4BE23C79  bl 0x82466e20
	ctx.lr = 0x826431AC;
	sub_82466E20(ctx, base);
	// 826431AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826431B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826431B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826431B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826431C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826431C0 size=108
    let mut pc: u32 = 0x826431C0;
    'dispatch: loop {
        match pc {
            0x826431C0 => {
    //   block [0x826431C0..0x8264322C)
	// 826431C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826431C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826431C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826431CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826431D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826431D4: 38EBC8B0  addi r7, r11, -0x3750
	ctx.r[7].s64 = ctx.r[11].s64 + -14160;
	// 826431D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826431DC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826431E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826431E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826431E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826431EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826431F0: 386AE884  addi r3, r10, -0x177c
	ctx.r[3].s64 = ctx.r[10].s64 + -6012;
	// 826431F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826431F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826431FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264320C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643218: 4BE23C09  bl 0x82466e20
	ctx.lr = 0x8264321C;
	sub_82466E20(ctx, base);
	// 8264321C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643230 size=108
    let mut pc: u32 = 0x82643230;
    'dispatch: loop {
        match pc {
            0x82643230 => {
    //   block [0x82643230..0x8264329C)
	// 82643230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264323C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643244: 38EBC940  addi r7, r11, -0x36c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14016;
	// 82643248: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264324C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82643250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264325C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643260: 386AE8B4  addi r3, r10, -0x174c
	ctx.r[3].s64 = ctx.r[10].s64 + -5964;
	// 82643264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264326C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264327C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643288: 4BE23B99  bl 0x82466e20
	ctx.lr = 0x8264328C;
	sub_82466E20(ctx, base);
	// 8264328C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826432A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826432A0 size=100
    let mut pc: u32 = 0x826432A0;
    'dispatch: loop {
        match pc {
            0x826432A0 => {
    //   block [0x826432A0..0x82643304)
	// 826432A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826432A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826432A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826432AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826432B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826432B4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826432B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826432BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826432C0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826432C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826432C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826432CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826432D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826432D4: 386AE8E4  addi r3, r10, -0x171c
	ctx.r[3].s64 = ctx.r[10].s64 + -5916;
	// 826432D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826432DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826432E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826432E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826432E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826432EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826432F0: 4BE23B31  bl 0x82466e20
	ctx.lr = 0x826432F4;
	sub_82466E20(ctx, base);
	// 826432F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826432F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826432FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643308 size=112
    let mut pc: u32 = 0x82643308;
    'dispatch: loop {
        match pc {
            0x82643308 => {
    //   block [0x82643308..0x82643378)
	// 82643308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264330C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643314: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643318: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264331C: 38AAE8E4  addi r5, r10, -0x171c
	ctx.r[5].s64 = ctx.r[10].s64 + -5916;
	// 82643320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643324: 390BC9D0  addi r8, r11, -0x3630
	ctx.r[8].s64 = ctx.r[11].s64 + -13872;
	// 82643328: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264332C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82643330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643334: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264333C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643340: 386AE914  addi r3, r10, -0x16ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5868;
	// 82643344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82643348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264334C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264335C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643364: 4BE23ABD  bl 0x82466e20
	ctx.lr = 0x82643368;
	sub_82466E20(ctx, base);
	// 82643368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264336C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643378 size=108
    let mut pc: u32 = 0x82643378;
    'dispatch: loop {
        match pc {
            0x82643378 => {
    //   block [0x82643378..0x826433E4)
	// 82643378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643384: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264338C: 38EBCA30  addi r7, r11, -0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + -13776;
	// 82643390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643394: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82643398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264339C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826433A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826433A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826433A8: 386AE944  addi r3, r10, -0x16bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5820;
	// 826433AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826433B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826433B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826433B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826433BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826433C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826433C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826433C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826433CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826433D0: 4BE23A51  bl 0x82466e20
	ctx.lr = 0x826433D4;
	sub_82466E20(ctx, base);
	// 826433D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826433D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826433DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826433E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826433E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826433E8 size=108
    let mut pc: u32 = 0x826433E8;
    'dispatch: loop {
        match pc {
            0x826433E8 => {
    //   block [0x826433E8..0x82643454)
	// 826433E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826433EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826433F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826433F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826433F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826433FC: 38EBCA60  addi r7, r11, -0x35a0
	ctx.r[7].s64 = ctx.r[11].s64 + -13728;
	// 82643400: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82643404: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82643408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264340C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643418: 386AE974  addi r3, r10, -0x168c
	ctx.r[3].s64 = ctx.r[10].s64 + -5772;
	// 8264341C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264342C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264343C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643440: 4BE239E1  bl 0x82466e20
	ctx.lr = 0x82643444;
	sub_82466E20(ctx, base);
	// 82643444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


