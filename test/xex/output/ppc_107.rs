pub fn sub_826B9C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9C80 size=112
    let mut pc: u32 = 0x826B9C80;
    'dispatch: loop {
        match pc {
            0x826B9C80 => {
    //   block [0x826B9C80..0x826B9CF0)
	// 826B9C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9C8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9C94: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9C98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B9C9C: 390BD530  addi r8, r11, -0x2ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -10960;
	// 826B9CA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9CA4: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826B9CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9CAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9CB8: 386A08FC  addi r3, r10, 0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2300;
	// 826B9CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9CDC: 4BDAD145  bl 0x82466e20
	ctx.lr = 0x826B9CE0;
	sub_82466E20(ctx, base);
	// 826B9CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9CF0 size=24
    let mut pc: u32 = 0x826B9CF0;
    'dispatch: loop {
        match pc {
            0x826B9CF0 => {
    //   block [0x826B9CF0..0x826B9D08)
	// 826B9CF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9CF4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9CF8: 394A2A50  addi r10, r10, 0x2a50
	ctx.r[10].s64 = ctx.r[10].s64 + 10832;
	// 826B9CFC: 816BD0C4  lwz r11, -0x2f3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12092 as u32) ) } as u64;
	// 826B9D00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B9D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9D08 size=116
    let mut pc: u32 = 0x826B9D08;
    'dispatch: loop {
        match pc {
            0x826B9D08 => {
    //   block [0x826B9D08..0x826B9D7C)
	// 826B9D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9D14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9D18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D1C: 392B04FC  addi r9, r11, 0x4fc
	ctx.r[9].s64 = ctx.r[11].s64 + 1276;
	// 826B9D20: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9D24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B9D28: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9D2C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826B9D30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9D34: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826B9D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9D3C: 396B2A50  addi r11, r11, 0x2a50
	ctx.r[11].s64 = ctx.r[11].s64 + 10832;
	// 826B9D40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B9D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9D50: 386A092C  addi r3, r10, 0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + 2348;
	// 826B9D54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9D58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9D60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B9D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9D68: 4BDAD0B9  bl 0x82466e20
	ctx.lr = 0x826B9D6C;
	sub_82466E20(ctx, base);
	// 826B9D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9D80 size=116
    let mut pc: u32 = 0x826B9D80;
    'dispatch: loop {
        match pc {
            0x826B9D80 => {
    //   block [0x826B9D80..0x826B9DF4)
	// 826B9D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9D8C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9D90: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B9D94: 390AD560  addi r8, r10, -0x2aa0
	ctx.r[8].s64 = ctx.r[10].s64 + -10912;
	// 826B9D98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9DA0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B9DA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9DA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9DB4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826B9DB8: 396B056C  addi r11, r11, 0x56c
	ctx.r[11].s64 = ctx.r[11].s64 + 1388;
	// 826B9DBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9DC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9DC4: 386A095C  addi r3, r10, 0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + 2396;
	// 826B9DC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9DD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9DE0: 4BDAD041  bl 0x82466e20
	ctx.lr = 0x826B9DE4;
	sub_82466E20(ctx, base);
	// 826B9DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9DF8 size=108
    let mut pc: u32 = 0x826B9DF8;
    'dispatch: loop {
        match pc {
            0x826B9DF8 => {
    //   block [0x826B9DF8..0x826B9E64)
	// 826B9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9E04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9E0C: 38EBD590  addi r7, r11, -0x2a70
	ctx.r[7].s64 = ctx.r[11].s64 + -10864;
	// 826B9E10: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B9E14: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826B9E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9E28: 386A098C  addi r3, r10, 0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + 2444;
	// 826B9E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9E50: 4BDACFD1  bl 0x82466e20
	ctx.lr = 0x826B9E54;
	sub_82466E20(ctx, base);
	// 826B9E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9E68 size=24
    let mut pc: u32 = 0x826B9E68;
    'dispatch: loop {
        match pc {
            0x826B9E68 => {
    //   block [0x826B9E68..0x826B9E80)
	// 826B9E68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9E6C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9E70: 394A2BE8  addi r10, r10, 0x2be8
	ctx.r[10].s64 = ctx.r[10].s64 + 11240;
	// 826B9E74: 816BD620  lwz r11, -0x29e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10720 as u32) ) } as u64;
	// 826B9E78: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B9E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9E80 size=116
    let mut pc: u32 = 0x826B9E80;
    'dispatch: loop {
        match pc {
            0x826B9E80 => {
    //   block [0x826B9E80..0x826B9EF4)
	// 826B9E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9E8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9E90: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9E94: 392B0590  addi r9, r11, 0x590
	ctx.r[9].s64 = ctx.r[11].s64 + 1424;
	// 826B9E98: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9E9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9EA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9EA4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826B9EA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9EAC: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826B9EB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9EB4: 396B2BE8  addi r11, r11, 0x2be8
	ctx.r[11].s64 = ctx.r[11].s64 + 11240;
	// 826B9EB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B9EBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9EC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9EC8: 386A09BC  addi r3, r10, 0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + 2492;
	// 826B9ECC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9ED0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9ED8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B9EDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9EE0: 4BDACF41  bl 0x82466e20
	ctx.lr = 0x826B9EE4;
	sub_82466E20(ctx, base);
	// 826B9EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9EF8 size=112
    let mut pc: u32 = 0x826B9EF8;
    'dispatch: loop {
        match pc {
            0x826B9EF8 => {
    //   block [0x826B9EF8..0x826B9F68)
	// 826B9EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9F0C: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9F14: 390BD624  addi r8, r11, -0x29dc
	ctx.r[8].s64 = ctx.r[11].s64 + -10716;
	// 826B9F18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9F1C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826B9F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9F24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9F30: 386A09EC  addi r3, r10, 0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + 2540;
	// 826B9F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9F54: 4BDACECD  bl 0x82466e20
	ctx.lr = 0x826B9F58;
	sub_82466E20(ctx, base);
	// 826B9F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9F68 size=116
    let mut pc: u32 = 0x826B9F68;
    'dispatch: loop {
        match pc {
            0x826B9F68 => {
    //   block [0x826B9F68..0x826B9FDC)
	// 826B9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9F74: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9F78: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B9F7C: 390AD658  addi r8, r10, -0x29a8
	ctx.r[8].s64 = ctx.r[10].s64 + -10664;
	// 826B9F80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F84: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9F88: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9F8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9F90: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9F9C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826B9FA0: 396B05D8  addi r11, r11, 0x5d8
	ctx.r[11].s64 = ctx.r[11].s64 + 1496;
	// 826B9FA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9FA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9FAC: 386A0A1C  addi r3, r10, 0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + 2588;
	// 826B9FB0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9FB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9FB8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9FC8: 4BDACE59  bl 0x82466e20
	ctx.lr = 0x826B9FCC;
	sub_82466E20(ctx, base);
	// 826B9FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9FE0 size=108
    let mut pc: u32 = 0x826B9FE0;
    'dispatch: loop {
        match pc {
            0x826B9FE0 => {
    //   block [0x826B9FE0..0x826BA04C)
	// 826B9FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9FEC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9FF4: 38EBD6E8  addi r7, r11, -0x2918
	ctx.r[7].s64 = ctx.r[11].s64 + -10520;
	// 826B9FF8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826B9FFC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826BA000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA010: 386A0A4C  addi r3, r10, 0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + 2636;
	// 826BA014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA01C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA038: 4BDACDE9  bl 0x82466e20
	ctx.lr = 0x826BA03C;
	sub_82466E20(ctx, base);
	// 826BA03C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA050 size=116
    let mut pc: u32 = 0x826BA050;
    'dispatch: loop {
        match pc {
            0x826BA050 => {
    //   block [0x826BA050..0x826BA0C4)
	// 826BA050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA05C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA060: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BA064: 390AD838  addi r8, r10, -0x27c8
	ctx.r[8].s64 = ctx.r[10].s64 + -10184;
	// 826BA068: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA06C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA070: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA074: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA078: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA084: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826BA088: 396B05FC  addi r11, r11, 0x5fc
	ctx.r[11].s64 = ctx.r[11].s64 + 1532;
	// 826BA08C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA090: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA094: 386A0A7C  addi r3, r10, 0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + 2684;
	// 826BA098: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA0A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA0B0: 4BDACD71  bl 0x82466e20
	ctx.lr = 0x826BA0B4;
	sub_82466E20(ctx, base);
	// 826BA0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA0C8 size=112
    let mut pc: u32 = 0x826BA0C8;
    'dispatch: loop {
        match pc {
            0x826BA0C8 => {
    //   block [0x826BA0C8..0x826BA138)
	// 826BA0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA0D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA0DC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA0E4: 390BD8F8  addi r8, r11, -0x2708
	ctx.r[8].s64 = ctx.r[11].s64 + -9992;
	// 826BA0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA0EC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826BA0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA0F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA100: 386A0AAC  addi r3, r10, 0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + 2732;
	// 826BA104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA124: 4BDACCFD  bl 0x82466e20
	ctx.lr = 0x826BA128;
	sub_82466E20(ctx, base);
	// 826BA128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA138 size=112
    let mut pc: u32 = 0x826BA138;
    'dispatch: loop {
        match pc {
            0x826BA138 => {
    //   block [0x826BA138..0x826BA1A8)
	// 826BA138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA148: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA14C: 38AA089C  addi r5, r10, 0x89c
	ctx.r[5].s64 = ctx.r[10].s64 + 2204;
	// 826BA150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA154: 390BD910  addi r8, r11, -0x26f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9968;
	// 826BA158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BA15C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826BA160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA170: 386A0ADC  addi r3, r10, 0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + 2780;
	// 826BA174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA194: 4BDACC8D  bl 0x82466e20
	ctx.lr = 0x826BA198;
	sub_82466E20(ctx, base);
	// 826BA198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA1A8 size=112
    let mut pc: u32 = 0x826BA1A8;
    'dispatch: loop {
        match pc {
            0x826BA1A8 => {
    //   block [0x826BA1A8..0x826BA218)
	// 826BA1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA1B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA1B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA1BC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA1C4: 390BD9A0  addi r8, r11, -0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + -9824;
	// 826BA1C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA1CC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826BA1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA1D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA1E0: 386A0B0C  addi r3, r10, 0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + 2828;
	// 826BA1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA204: 4BDACC1D  bl 0x82466e20
	ctx.lr = 0x826BA208;
	sub_82466E20(ctx, base);
	// 826BA208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA218 size=116
    let mut pc: u32 = 0x826BA218;
    'dispatch: loop {
        match pc {
            0x826BA218 => {
    //   block [0x826BA218..0x826BA28C)
	// 826BA218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA224: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA228: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA22C: 390BD9B8  addi r8, r11, -0x2648
	ctx.r[8].s64 = ctx.r[11].s64 + -9800;
	// 826BA230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA234: 392A0644  addi r9, r10, 0x644
	ctx.r[9].s64 = ctx.r[10].s64 + 1604;
	// 826BA238: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA23C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BA240: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA244: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA24C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA25C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BA260: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826BA264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA268: 386B0B3C  addi r3, r11, 0xb3c
	ctx.r[3].s64 = ctx.r[11].s64 + 2876;
	// 826BA26C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA278: 4BDACBA9  bl 0x82466e20
	ctx.lr = 0x826BA27C;
	sub_82466E20(ctx, base);
	// 826BA27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA290 size=100
    let mut pc: u32 = 0x826BA290;
    'dispatch: loop {
        match pc {
            0x826BA290 => {
    //   block [0x826BA290..0x826BA2F4)
	// 826BA290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA29C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA2A4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA2B0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826BA2B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA2BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA2C4: 386A0B6C  addi r3, r10, 0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + 2924;
	// 826BA2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA2CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA2D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA2D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA2E0: 4BDACB41  bl 0x82466e20
	ctx.lr = 0x826BA2E4;
	sub_82466E20(ctx, base);
	// 826BA2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA2F8 size=112
    let mut pc: u32 = 0x826BA2F8;
    'dispatch: loop {
        match pc {
            0x826BA2F8 => {
    //   block [0x826BA2F8..0x826BA368)
	// 826BA2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA308: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA30C: 38AA0B6C  addi r5, r10, 0xb6c
	ctx.r[5].s64 = ctx.r[10].s64 + 2924;
	// 826BA310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA314: 390BD9D0  addi r8, r11, -0x2630
	ctx.r[8].s64 = ctx.r[11].s64 + -9776;
	// 826BA318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA31C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826BA320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA330: 386A0B9C  addi r3, r10, 0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + 2972;
	// 826BA334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA354: 4BDACACD  bl 0x82466e20
	ctx.lr = 0x826BA358;
	sub_82466E20(ctx, base);
	// 826BA358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA368 size=108
    let mut pc: u32 = 0x826BA368;
    'dispatch: loop {
        match pc {
            0x826BA368 => {
    //   block [0x826BA368..0x826BA3D4)
	// 826BA368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA374: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826BA37C: 38EBD9E8  addi r7, r11, -0x2618
	ctx.r[7].s64 = ctx.r[11].s64 + -9752;
	// 826BA380: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BA384: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 826BA388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA38C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA398: 386A0BCC  addi r3, r10, 0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + 3020;
	// 826BA39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA3C0: 4BDACA61  bl 0x82466e20
	ctx.lr = 0x826BA3C4;
	sub_82466E20(ctx, base);
	// 826BA3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA3D8 size=112
    let mut pc: u32 = 0x826BA3D8;
    'dispatch: loop {
        match pc {
            0x826BA3D8 => {
    //   block [0x826BA3D8..0x826BA448)
	// 826BA3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA3E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA3EC: 392B06B0  addi r9, r11, 0x6b0
	ctx.r[9].s64 = ctx.r[11].s64 + 1712;
	// 826BA3F0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826BA3F4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826BA3F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA3FC: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826BA400: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA404: 396BDA50  addi r11, r11, -0x25b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9648;
	// 826BA408: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BA40C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA410: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BA414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA418: 386A0BFC  addi r3, r10, 0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3068;
	// 826BA41C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA420: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BA424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA428: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BA42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA434: 4BDAC9ED  bl 0x82466e20
	ctx.lr = 0x826BA438;
	sub_82466E20(ctx, base);
	// 826BA438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA448 size=108
    let mut pc: u32 = 0x826BA448;
    'dispatch: loop {
        match pc {
            0x826BA448 => {
    //   block [0x826BA448..0x826BA4B4)
	// 826BA448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA454: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA45C: 38EBDB10  addi r7, r11, -0x24f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9456;
	// 826BA460: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BA464: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826BA468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA46C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA478: 386A0C2C  addi r3, r10, 0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3116;
	// 826BA47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA4A0: 4BDAC981  bl 0x82466e20
	ctx.lr = 0x826BA4A4;
	sub_82466E20(ctx, base);
	// 826BA4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA4B8 size=116
    let mut pc: u32 = 0x826BA4B8;
    'dispatch: loop {
        match pc {
            0x826BA4B8 => {
    //   block [0x826BA4B8..0x826BA52C)
	// 826BA4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA4C4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA4C8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826BA4CC: 390ADB58  addi r8, r10, -0x24a8
	ctx.r[8].s64 = ctx.r[10].s64 + -9384;
	// 826BA4D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA4D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA4D8: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826BA4DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA4E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA4EC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826BA4F0: 396B06E8  addi r11, r11, 0x6e8
	ctx.r[11].s64 = ctx.r[11].s64 + 1768;
	// 826BA4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA4F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA4FC: 386A0C5C  addi r3, r10, 0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3164;
	// 826BA500: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA504: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA508: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA518: 4BDAC909  bl 0x82466e20
	ctx.lr = 0x826BA51C;
	sub_82466E20(ctx, base);
	// 826BA51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA530 size=100
    let mut pc: u32 = 0x826BA530;
    'dispatch: loop {
        match pc {
            0x826BA530 => {
    //   block [0x826BA530..0x826BA594)
	// 826BA530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA53C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA544: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826BA548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA550: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826BA554: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA564: 386A0C8C  addi r3, r10, 0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3212;
	// 826BA568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA56C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA580: 4BDAC8A1  bl 0x82466e20
	ctx.lr = 0x826BA584;
	sub_82466E20(ctx, base);
	// 826BA584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BA598 size=24
    let mut pc: u32 = 0x826BA598;
    'dispatch: loop {
        match pc {
            0x826BA598 => {
    //   block [0x826BA598..0x826BA5B0)
	// 826BA598: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA59C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA5A0: 394A2CF0  addi r10, r10, 0x2cf0
	ctx.r[10].s64 = ctx.r[10].s64 + 11504;
	// 826BA5A4: 816BDCD8  lwz r11, -0x2328(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9000 as u32) ) } as u64;
	// 826BA5A8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BA5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA5B0 size=116
    let mut pc: u32 = 0x826BA5B0;
    'dispatch: loop {
        match pc {
            0x826BA5B0 => {
    //   block [0x826BA5B0..0x826BA624)
	// 826BA5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA5BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA5C0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA5C4: 392B0768  addi r9, r11, 0x768
	ctx.r[9].s64 = ctx.r[11].s64 + 1896;
	// 826BA5C8: 38AA0C8C  addi r5, r10, 0xc8c
	ctx.r[5].s64 = ctx.r[10].s64 + 3212;
	// 826BA5CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA5D0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826BA5D4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 826BA5D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA5DC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826BA5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA5E4: 396B2CF0  addi r11, r11, 0x2cf0
	ctx.r[11].s64 = ctx.r[11].s64 + 11504;
	// 826BA5E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BA5EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA5F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BA5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA5F8: 386A0CBC  addi r3, r10, 0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + 3260;
	// 826BA5FC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BA600: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BA604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA608: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BA60C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA610: 4BDAC811  bl 0x82466e20
	ctx.lr = 0x826BA614;
	sub_82466E20(ctx, base);
	// 826BA614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA628 size=112
    let mut pc: u32 = 0x826BA628;
    'dispatch: loop {
        match pc {
            0x826BA628 => {
    //   block [0x826BA628..0x826BA698)
	// 826BA628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA634: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA638: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BA63C: 38EADCE0  addi r7, r10, -0x2320
	ctx.r[7].s64 = ctx.r[10].s64 + -8992;
	// 826BA640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA644: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA648: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826BA64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA650: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA654: 396B07F4  addi r11, r11, 0x7f4
	ctx.r[11].s64 = ctx.r[11].s64 + 2036;
	// 826BA658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA65C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA660: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA664: 386A0CEC  addi r3, r10, 0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + 3308;
	// 826BA668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA66C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA670: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA674: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA678: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA67C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA684: 4BDAC79D  bl 0x82466e20
	ctx.lr = 0x826BA688;
	sub_82466E20(ctx, base);
	// 826BA688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA698 size=108
    let mut pc: u32 = 0x826BA698;
    'dispatch: loop {
        match pc {
            0x826BA698 => {
    //   block [0x826BA698..0x826BA704)
	// 826BA698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA6A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA6AC: 38EBDD40  addi r7, r11, -0x22c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8896;
	// 826BA6B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA6B4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826BA6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA6BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA6C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA6C8: 386A0D1C  addi r3, r10, 0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3356;
	// 826BA6CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA6EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA6F0: 4BDAC731  bl 0x82466e20
	ctx.lr = 0x826BA6F4;
	sub_82466E20(ctx, base);
	// 826BA6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA708 size=116
    let mut pc: u32 = 0x826BA708;
    'dispatch: loop {
        match pc {
            0x826BA708 => {
    //   block [0x826BA708..0x826BA77C)
	// 826BA708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA718: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA71C: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 826BA720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA724: 392A07E0  addi r9, r10, 0x7e0
	ctx.r[9].s64 = ctx.r[10].s64 + 2016;
	// 826BA728: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA72C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BA730: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA734: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA73C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA74C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BA750: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826BA754: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA758: 386B0D4C  addi r3, r11, 0xd4c
	ctx.r[3].s64 = ctx.r[11].s64 + 3404;
	// 826BA75C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA760: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA768: 4BDAC6B9  bl 0x82466e20
	ctx.lr = 0x826BA76C;
	sub_82466E20(ctx, base);
	// 826BA76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA780 size=96
    let mut pc: u32 = 0x826BA780;
    'dispatch: loop {
        match pc {
            0x826BA780 => {
    //   block [0x826BA780..0x826BA7E0)
	// 826BA780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA78C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA794: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826BA798: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA7A0: 386A0D7C  addi r3, r10, 0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3452;
	// 826BA7A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA7AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BA7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA7C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA7C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA7CC: 4BDAC655  bl 0x82466e20
	ctx.lr = 0x826BA7D0;
	sub_82466E20(ctx, base);
	// 826BA7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA7E0 size=112
    let mut pc: u32 = 0x826BA7E0;
    'dispatch: loop {
        match pc {
            0x826BA7E0 => {
    //   block [0x826BA7E0..0x826BA850)
	// 826BA7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA7EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA7F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA7F4: 38AA0D7C  addi r5, r10, 0xd7c
	ctx.r[5].s64 = ctx.r[10].s64 + 3452;
	// 826BA7F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA7FC: 390BDD88  addi r8, r11, -0x2278
	ctx.r[8].s64 = ctx.r[11].s64 + -8824;
	// 826BA800: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BA804: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826BA808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA80C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA818: 386A0DAC  addi r3, r10, 0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + 3500;
	// 826BA81C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA83C: 4BDAC5E5  bl 0x82466e20
	ctx.lr = 0x826BA840;
	sub_82466E20(ctx, base);
	// 826BA840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA850 size=112
    let mut pc: u32 = 0x826BA850;
    'dispatch: loop {
        match pc {
            0x826BA850 => {
    //   block [0x826BA850..0x826BA8C0)
	// 826BA850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA85C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA860: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA864: 392A0810  addi r9, r10, 0x810
	ctx.r[9].s64 = ctx.r[10].s64 + 2064;
	// 826BA868: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA86C: 390BDDC0  addi r8, r11, -0x2240
	ctx.r[8].s64 = ctx.r[11].s64 + -8768;
	// 826BA870: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826BA874: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826BA878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA87C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA888: 386A0DDC  addi r3, r10, 0xddc
	ctx.r[3].s64 = ctx.r[10].s64 + 3548;
	// 826BA88C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA890: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA8AC: 4BDAC575  bl 0x82466e20
	ctx.lr = 0x826BA8B0;
	sub_82466E20(ctx, base);
	// 826BA8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA8C0 size=108
    let mut pc: u32 = 0x826BA8C0;
    'dispatch: loop {
        match pc {
            0x826BA8C0 => {
    //   block [0x826BA8C0..0x826BA92C)
	// 826BA8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA8CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA8D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA8D4: 38EBDE68  addi r7, r11, -0x2198
	ctx.r[7].s64 = ctx.r[11].s64 + -8600;
	// 826BA8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA8DC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826BA8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA8F0: 386A0E0C  addi r3, r10, 0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + 3596;
	// 826BA8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA918: 4BDAC509  bl 0x82466e20
	ctx.lr = 0x826BA91C;
	sub_82466E20(ctx, base);
	// 826BA91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA930 size=108
    let mut pc: u32 = 0x826BA930;
    'dispatch: loop {
        match pc {
            0x826BA930 => {
    //   block [0x826BA930..0x826BA99C)
	// 826BA930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA93C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA940: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA944: 38EBDE98  addi r7, r11, -0x2168
	ctx.r[7].s64 = ctx.r[11].s64 + -8552;
	// 826BA948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA94C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826BA950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA960: 386A0E3C  addi r3, r10, 0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + 3644;
	// 826BA964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA988: 4BDAC499  bl 0x82466e20
	ctx.lr = 0x826BA98C;
	sub_82466E20(ctx, base);
	// 826BA98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BA9A0 size=28
    let mut pc: u32 = 0x826BA9A0;
    'dispatch: loop {
        match pc {
            0x826BA9A0 => {
    //   block [0x826BA9A0..0x826BA9BC)
	// 826BA9A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA9A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA9A8: 394A2E10  addi r10, r10, 0x2e10
	ctx.r[10].s64 = ctx.r[10].s64 + 11792;
	// 826BA9AC: 816BDDBC  lwz r11, -0x2244(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8772 as u32) ) } as u64;
	// 826BA9B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BA9B4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826BA9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA9C0 size=112
    let mut pc: u32 = 0x826BA9C0;
    'dispatch: loop {
        match pc {
            0x826BA9C0 => {
    //   block [0x826BA9C0..0x826BAA30)
	// 826BA9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA9CC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA9D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA9D4: 392A0988  addi r9, r10, 0x988
	ctx.r[9].s64 = ctx.r[10].s64 + 2440;
	// 826BA9D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA9DC: 390B2E10  addi r8, r11, 0x2e10
	ctx.r[8].s64 = ctx.r[11].s64 + 11792;
	// 826BA9E0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BA9E4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826BA9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA9F8: 386A0E6C  addi r3, r10, 0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3692;
	// 826BA9FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAA00: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826BAA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAA14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAA1C: 4BDAC405  bl 0x82466e20
	ctx.lr = 0x826BAA20;
	sub_82466E20(ctx, base);
	// 826BAA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAA30 size=108
    let mut pc: u32 = 0x826BAA30;
    'dispatch: loop {
        match pc {
            0x826BAA30 => {
    //   block [0x826BAA30..0x826BAA9C)
	// 826BAA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAA3C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAA40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAA44: 38EBDED0  addi r7, r11, -0x2130
	ctx.r[7].s64 = ctx.r[11].s64 + -8496;
	// 826BAA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAA4C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826BAA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAA54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAA60: 386A0E9C  addi r3, r10, 0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3740;
	// 826BAA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAA88: 4BDAC399  bl 0x82466e20
	ctx.lr = 0x826BAA8C;
	sub_82466E20(ctx, base);
	// 826BAA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAAA0 size=108
    let mut pc: u32 = 0x826BAAA0;
    'dispatch: loop {
        match pc {
            0x826BAAA0 => {
    //   block [0x826BAAA0..0x826BAB0C)
	// 826BAAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAAAC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAAB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAAB4: 38EBDF00  addi r7, r11, -0x2100
	ctx.r[7].s64 = ctx.r[11].s64 + -8448;
	// 826BAAB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BAABC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826BAAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAAC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAAC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAAD0: 386A0ECC  addi r3, r10, 0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + 3788;
	// 826BAAD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAAF8: 4BDAC329  bl 0x82466e20
	ctx.lr = 0x826BAAFC;
	sub_82466E20(ctx, base);
	// 826BAAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAB10 size=24
    let mut pc: u32 = 0x826BAB10;
    'dispatch: loop {
        match pc {
            0x826BAB10 => {
    //   block [0x826BAB10..0x826BAB28)
	// 826BAB10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAB14: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAB18: 394A2ED0  addi r10, r10, 0x2ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 11984;
	// 826BAB1C: 816BDF18  lwz r11, -0x20e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8424 as u32) ) } as u64;
	// 826BAB20: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BAB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAB28 size=112
    let mut pc: u32 = 0x826BAB28;
    'dispatch: loop {
        match pc {
            0x826BAB28 => {
    //   block [0x826BAB28..0x826BAB98)
	// 826BAB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAB34: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BAB38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAB3C: 392A09DC  addi r9, r10, 0x9dc
	ctx.r[9].s64 = ctx.r[10].s64 + 2524;
	// 826BAB40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAB44: 390B2ED0  addi r8, r11, 0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 11984;
	// 826BAB48: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826BAB4C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826BAB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BAB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAB60: 386A0EFC  addi r3, r10, 0xefc
	ctx.r[3].s64 = ctx.r[10].s64 + 3836;
	// 826BAB64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAB68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BAB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAB84: 4BDAC29D  bl 0x82466e20
	ctx.lr = 0x826BAB88;
	sub_82466E20(ctx, base);
	// 826BAB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAB98 size=112
    let mut pc: u32 = 0x826BAB98;
    'dispatch: loop {
        match pc {
            0x826BAB98 => {
    //   block [0x826BAB98..0x826BAC08)
	// 826BAB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BABA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BABA4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BABA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BABAC: 392A0A18  addi r9, r10, 0xa18
	ctx.r[9].s64 = ctx.r[10].s64 + 2584;
	// 826BABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BABB4: 390BDF28  addi r8, r11, -0x20d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8408;
	// 826BABB8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BABBC: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826BABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BABC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BABC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BABCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BABD0: 386A0F2C  addi r3, r10, 0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3884;
	// 826BABD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BABD8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BABDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BABEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BABF4: 4BDAC22D  bl 0x82466e20
	ctx.lr = 0x826BABF8;
	sub_82466E20(ctx, base);
	// 826BABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAC08 size=108
    let mut pc: u32 = 0x826BAC08;
    'dispatch: loop {
        match pc {
            0x826BAC08 => {
    //   block [0x826BAC08..0x826BAC74)
	// 826BAC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAC14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAC18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAC1C: 38EBDF70  addi r7, r11, -0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + -8336;
	// 826BAC20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAC24: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826BAC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAC38: 386A0F5C  addi r3, r10, 0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3932;
	// 826BAC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAC60: 4BDAC1C1  bl 0x82466e20
	ctx.lr = 0x826BAC64;
	sub_82466E20(ctx, base);
	// 826BAC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAC78 size=108
    let mut pc: u32 = 0x826BAC78;
    'dispatch: loop {
        match pc {
            0x826BAC78 => {
    //   block [0x826BAC78..0x826BACE4)
	// 826BAC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAC84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAC88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAC8C: 38EBDFA0  addi r7, r11, -0x2060
	ctx.r[7].s64 = ctx.r[11].s64 + -8288;
	// 826BAC90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAC94: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826BAC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAC9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BACA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BACA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BACA8: 386A0F8C  addi r3, r10, 0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3980;
	// 826BACAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BACB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BACB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BACBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BACC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BACC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BACC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BACCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BACD0: 4BDAC151  bl 0x82466e20
	ctx.lr = 0x826BACD4;
	sub_82466E20(ctx, base);
	// 826BACD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BACD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BACDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BACE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BACE8 size=112
    let mut pc: u32 = 0x826BACE8;
    'dispatch: loop {
        match pc {
            0x826BACE8 => {
    //   block [0x826BACE8..0x826BAD58)
	// 826BACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BACF4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BACF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BACFC: 392A0A50  addi r9, r10, 0xa50
	ctx.r[9].s64 = ctx.r[10].s64 + 2640;
	// 826BAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAD04: 390BDFD0  addi r8, r11, -0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + -8240;
	// 826BAD08: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826BAD0C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826BAD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAD14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BAD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAD20: 386A0FBC  addi r3, r10, 0xfbc
	ctx.r[3].s64 = ctx.r[10].s64 + 4028;
	// 826BAD24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAD28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BAD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAD44: 4BDAC0DD  bl 0x82466e20
	ctx.lr = 0x826BAD48;
	sub_82466E20(ctx, base);
	// 826BAD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAD58 size=108
    let mut pc: u32 = 0x826BAD58;
    'dispatch: loop {
        match pc {
            0x826BAD58 => {
    //   block [0x826BAD58..0x826BADC4)
	// 826BAD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAD64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAD68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAD6C: 38EBE030  addi r7, r11, -0x1fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -8144;
	// 826BAD70: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826BAD74: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826BAD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAD7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAD80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAD88: 386A0FEC  addi r3, r10, 0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + 4076;
	// 826BAD8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BADA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BADA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BADA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BADAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BADB0: 4BDAC071  bl 0x82466e20
	ctx.lr = 0x826BADB4;
	sub_82466E20(ctx, base);
	// 826BADB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BADB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BADBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BADC8 size=108
    let mut pc: u32 = 0x826BADC8;
    'dispatch: loop {
        match pc {
            0x826BADC8 => {
    //   block [0x826BADC8..0x826BAE34)
	// 826BADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BADD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BADD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BADDC: 38EBE120  addi r7, r11, -0x1ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -7904;
	// 826BADE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BADE4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826BADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BADEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BADF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BADF8: 386A101C  addi r3, r10, 0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + 4124;
	// 826BADFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAE20: 4BDAC001  bl 0x82466e20
	ctx.lr = 0x826BAE24;
	sub_82466E20(ctx, base);
	// 826BAE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAE38 size=108
    let mut pc: u32 = 0x826BAE38;
    'dispatch: loop {
        match pc {
            0x826BAE38 => {
    //   block [0x826BAE38..0x826BAEA4)
	// 826BAE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAE44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAE4C: 38EBE138  addi r7, r11, -0x1ec8
	ctx.r[7].s64 = ctx.r[11].s64 + -7880;
	// 826BAE50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826BAE54: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826BAE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAE5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAE68: 386A104C  addi r3, r10, 0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + 4172;
	// 826BAE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAE90: 4BDABF91  bl 0x82466e20
	ctx.lr = 0x826BAE94;
	sub_82466E20(ctx, base);
	// 826BAE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAEA8 size=24
    let mut pc: u32 = 0x826BAEA8;
    'dispatch: loop {
        match pc {
            0x826BAEA8 => {
    //   block [0x826BAEA8..0x826BAEC0)
	// 826BAEA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAEAC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAEB0: 394A2F60  addi r10, r10, 0x2f60
	ctx.r[10].s64 = ctx.r[10].s64 + 12128;
	// 826BAEB4: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BAEB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BAEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAEC0 size=108
    let mut pc: u32 = 0x826BAEC0;
    'dispatch: loop {
        match pc {
            0x826BAEC0 => {
    //   block [0x826BAEC0..0x826BAF2C)
	// 826BAEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAECC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAED4: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 826BAED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAEDC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826BAEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAEE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAEF0: 386A107C  addi r3, r10, 0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + 4220;
	// 826BAEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAF18: 4BDABF09  bl 0x82466e20
	ctx.lr = 0x826BAF1C;
	sub_82466E20(ctx, base);
	// 826BAF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAF30 size=24
    let mut pc: u32 = 0x826BAF30;
    'dispatch: loop {
        match pc {
            0x826BAF30 => {
    //   block [0x826BAF30..0x826BAF48)
	// 826BAF30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAF34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAF38: 394A2F90  addi r10, r10, 0x2f90
	ctx.r[10].s64 = ctx.r[10].s64 + 12176;
	// 826BAF3C: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BAF40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BAF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAF48 size=108
    let mut pc: u32 = 0x826BAF48;
    'dispatch: loop {
        match pc {
            0x826BAF48 => {
    //   block [0x826BAF48..0x826BAFB4)
	// 826BAF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAF54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAF5C: 38EB2F90  addi r7, r11, 0x2f90
	ctx.r[7].s64 = ctx.r[11].s64 + 12176;
	// 826BAF60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAF64: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826BAF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAF6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAF70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAF78: 386A10AC  addi r3, r10, 0x10ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4268;
	// 826BAF7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAFA0: 4BDABE81  bl 0x82466e20
	ctx.lr = 0x826BAFA4;
	sub_82466E20(ctx, base);
	// 826BAFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAFB8 size=108
    let mut pc: u32 = 0x826BAFB8;
    'dispatch: loop {
        match pc {
            0x826BAFB8 => {
    //   block [0x826BAFB8..0x826BB024)
	// 826BAFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAFC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAFCC: 38EBE1B0  addi r7, r11, -0x1e50
	ctx.r[7].s64 = ctx.r[11].s64 + -7760;
	// 826BAFD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BAFD4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826BAFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAFDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAFE8: 386A10DC  addi r3, r10, 0x10dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4316;
	// 826BAFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB010: 4BDABE11  bl 0x82466e20
	ctx.lr = 0x826BB014;
	sub_82466E20(ctx, base);
	// 826BB014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB028 size=24
    let mut pc: u32 = 0x826BB028;
    'dispatch: loop {
        match pc {
            0x826BB028 => {
    //   block [0x826BB028..0x826BB040)
	// 826BB028: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB02C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB030: 394A2FC0  addi r10, r10, 0x2fc0
	ctx.r[10].s64 = ctx.r[10].s64 + 12224;
	// 826BB034: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BB038: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BB03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB040 size=108
    let mut pc: u32 = 0x826BB040;
    'dispatch: loop {
        match pc {
            0x826BB040 => {
    //   block [0x826BB040..0x826BB0AC)
	// 826BB040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB04C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB054: 38EB2FC0  addi r7, r11, 0x2fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 12224;
	// 826BB058: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB05C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826BB060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB064: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB070: 386A110C  addi r3, r10, 0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + 4364;
	// 826BB074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB098: 4BDABD89  bl 0x82466e20
	ctx.lr = 0x826BB09C;
	sub_82466E20(ctx, base);
	// 826BB09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB0B0 size=112
    let mut pc: u32 = 0x826BB0B0;
    'dispatch: loop {
        match pc {
            0x826BB0B0 => {
    //   block [0x826BB0B0..0x826BB120)
	// 826BB0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB0BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB0C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB0C4: 392A0A94  addi r9, r10, 0xa94
	ctx.r[9].s64 = ctx.r[10].s64 + 2708;
	// 826BB0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB0CC: 390BE1CC  addi r8, r11, -0x1e34
	ctx.r[8].s64 = ctx.r[11].s64 + -7732;
	// 826BB0D0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826BB0D4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826BB0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB0E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB0E8: 386A113C  addi r3, r10, 0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + 4412;
	// 826BB0EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB0F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB10C: 4BDABD15  bl 0x82466e20
	ctx.lr = 0x826BB110;
	sub_82466E20(ctx, base);
	// 826BB110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB120 size=108
    let mut pc: u32 = 0x826BB120;
    'dispatch: loop {
        match pc {
            0x826BB120 => {
    //   block [0x826BB120..0x826BB18C)
	// 826BB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB12C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB134: 38EBE1FC  addi r7, r11, -0x1e04
	ctx.r[7].s64 = ctx.r[11].s64 + -7684;
	// 826BB138: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB13C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826BB140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB150: 386A116C  addi r3, r10, 0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + 4460;
	// 826BB154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB178: 4BDABCA9  bl 0x82466e20
	ctx.lr = 0x826BB17C;
	sub_82466E20(ctx, base);
	// 826BB17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB190 size=108
    let mut pc: u32 = 0x826BB190;
    'dispatch: loop {
        match pc {
            0x826BB190 => {
    //   block [0x826BB190..0x826BB1FC)
	// 826BB190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB19C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB1A4: 38EBE22C  addi r7, r11, -0x1dd4
	ctx.r[7].s64 = ctx.r[11].s64 + -7636;
	// 826BB1A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB1AC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826BB1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB1B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB1C0: 386A119C  addi r3, r10, 0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + 4508;
	// 826BB1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB1E8: 4BDABC39  bl 0x82466e20
	ctx.lr = 0x826BB1EC;
	sub_82466E20(ctx, base);
	// 826BB1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB200 size=112
    let mut pc: u32 = 0x826BB200;
    'dispatch: loop {
        match pc {
            0x826BB200 => {
    //   block [0x826BB200..0x826BB270)
	// 826BB200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB210: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB214: 38AA11FC  addi r5, r10, 0x11fc
	ctx.r[5].s64 = ctx.r[10].s64 + 4604;
	// 826BB218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB21C: 390BE25C  addi r8, r11, -0x1da4
	ctx.r[8].s64 = ctx.r[11].s64 + -7588;
	// 826BB220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BB224: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826BB228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB22C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB238: 386A11CC  addi r3, r10, 0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4556;
	// 826BB23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BB240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB25C: 4BDABBC5  bl 0x82466e20
	ctx.lr = 0x826BB260;
	sub_82466E20(ctx, base);
	// 826BB260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB270 size=108
    let mut pc: u32 = 0x826BB270;
    'dispatch: loop {
        match pc {
            0x826BB270 => {
    //   block [0x826BB270..0x826BB2DC)
	// 826BB270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB27C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB284: 38EBE274  addi r7, r11, -0x1d8c
	ctx.r[7].s64 = ctx.r[11].s64 + -7564;
	// 826BB288: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB28C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826BB290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB298: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB2A0: 386A11FC  addi r3, r10, 0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4604;
	// 826BB2A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB2C8: 4BDABB59  bl 0x82466e20
	ctx.lr = 0x826BB2CC;
	sub_82466E20(ctx, base);
	// 826BB2CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB2E0 size=108
    let mut pc: u32 = 0x826BB2E0;
    'dispatch: loop {
        match pc {
            0x826BB2E0 => {
    //   block [0x826BB2E0..0x826BB34C)
	// 826BB2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB2EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB2F4: 38EBE2A4  addi r7, r11, -0x1d5c
	ctx.r[7].s64 = ctx.r[11].s64 + -7516;
	// 826BB2F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB2FC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826BB300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB310: 386A122C  addi r3, r10, 0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + 4652;
	// 826BB314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB338: 4BDABAE9  bl 0x82466e20
	ctx.lr = 0x826BB33C;
	sub_82466E20(ctx, base);
	// 826BB33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB350 size=108
    let mut pc: u32 = 0x826BB350;
    'dispatch: loop {
        match pc {
            0x826BB350 => {
    //   block [0x826BB350..0x826BB3BC)
	// 826BB350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB35C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB364: 38EBE2BC  addi r7, r11, -0x1d44
	ctx.r[7].s64 = ctx.r[11].s64 + -7492;
	// 826BB368: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB36C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826BB370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB378: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB380: 386A125C  addi r3, r10, 0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + 4700;
	// 826BB384: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB3A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB3A8: 4BDABA79  bl 0x82466e20
	ctx.lr = 0x826BB3AC;
	sub_82466E20(ctx, base);
	// 826BB3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB3C0 size=108
    let mut pc: u32 = 0x826BB3C0;
    'dispatch: loop {
        match pc {
            0x826BB3C0 => {
    //   block [0x826BB3C0..0x826BB42C)
	// 826BB3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB3CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB3D4: 38EBE2F0  addi r7, r11, -0x1d10
	ctx.r[7].s64 = ctx.r[11].s64 + -7440;
	// 826BB3D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826BB3DC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826BB3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB3E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB3E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB3F0: 386A128C  addi r3, r10, 0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + 4748;
	// 826BB3F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB418: 4BDABA09  bl 0x82466e20
	ctx.lr = 0x826BB41C;
	sub_82466E20(ctx, base);
	// 826BB41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB430 size=108
    let mut pc: u32 = 0x826BB430;
    'dispatch: loop {
        match pc {
            0x826BB430 => {
    //   block [0x826BB430..0x826BB49C)
	// 826BB430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB43C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB444: 38EBE398  addi r7, r11, -0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + -7272;
	// 826BB448: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB44C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826BB450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB454: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB460: 386A12BC  addi r3, r10, 0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4796;
	// 826BB464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB488: 4BDAB999  bl 0x82466e20
	ctx.lr = 0x826BB48C;
	sub_82466E20(ctx, base);
	// 826BB48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB4A0 size=108
    let mut pc: u32 = 0x826BB4A0;
    'dispatch: loop {
        match pc {
            0x826BB4A0 => {
    //   block [0x826BB4A0..0x826BB50C)
	// 826BB4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB4AC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB4B4: 38EBE3C8  addi r7, r11, -0x1c38
	ctx.r[7].s64 = ctx.r[11].s64 + -7224;
	// 826BB4B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB4BC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826BB4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB4C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB4D0: 386A12EC  addi r3, r10, 0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4844;
	// 826BB4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB4F8: 4BDAB929  bl 0x82466e20
	ctx.lr = 0x826BB4FC;
	sub_82466E20(ctx, base);
	// 826BB4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB510 size=108
    let mut pc: u32 = 0x826BB510;
    'dispatch: loop {
        match pc {
            0x826BB510 => {
    //   block [0x826BB510..0x826BB57C)
	// 826BB510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB51C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB524: 38EBE3E0  addi r7, r11, -0x1c20
	ctx.r[7].s64 = ctx.r[11].s64 + -7200;
	// 826BB528: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB52C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826BB530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB540: 386A131C  addi r3, r10, 0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + 4892;
	// 826BB544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB568: 4BDAB8B9  bl 0x82466e20
	ctx.lr = 0x826BB56C;
	sub_82466E20(ctx, base);
	// 826BB56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB580 size=108
    let mut pc: u32 = 0x826BB580;
    'dispatch: loop {
        match pc {
            0x826BB580 => {
    //   block [0x826BB580..0x826BB5EC)
	// 826BB580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB58C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB594: 38EBE410  addi r7, r11, -0x1bf0
	ctx.r[7].s64 = ctx.r[11].s64 + -7152;
	// 826BB598: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BB59C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826BB5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB5A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB5B0: 386A134C  addi r3, r10, 0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + 4940;
	// 826BB5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB5D8: 4BDAB849  bl 0x82466e20
	ctx.lr = 0x826BB5DC;
	sub_82466E20(ctx, base);
	// 826BB5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB5F0 size=24
    let mut pc: u32 = 0x826BB5F0;
    'dispatch: loop {
        match pc {
            0x826BB5F0 => {
    //   block [0x826BB5F0..0x826BB608)
	// 826BB5F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB5F4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB5F8: 394A2FF0  addi r10, r10, 0x2ff0
	ctx.r[10].s64 = ctx.r[10].s64 + 12272;
	// 826BB5FC: 816BE2EC  lwz r11, -0x1d14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7444 as u32) ) } as u64;
	// 826BB600: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BB604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB608 size=112
    let mut pc: u32 = 0x826BB608;
    'dispatch: loop {
        match pc {
            0x826BB608 => {
    //   block [0x826BB608..0x826BB678)
	// 826BB608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB614: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB618: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB61C: 392A0AC0  addi r9, r10, 0xac0
	ctx.r[9].s64 = ctx.r[10].s64 + 2752;
	// 826BB620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB624: 390B2FF0  addi r8, r11, 0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 12272;
	// 826BB628: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BB62C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826BB630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB640: 386A137C  addi r3, r10, 0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + 4988;
	// 826BB644: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB648: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB664: 4BDAB7BD  bl 0x82466e20
	ctx.lr = 0x826BB668;
	sub_82466E20(ctx, base);
	// 826BB668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB678 size=108
    let mut pc: u32 = 0x826BB678;
    'dispatch: loop {
        match pc {
            0x826BB678 => {
    //   block [0x826BB678..0x826BB6E4)
	// 826BB678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB684: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB68C: 38EBE4D4  addi r7, r11, -0x1b2c
	ctx.r[7].s64 = ctx.r[11].s64 + -6956;
	// 826BB690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB694: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826BB698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB69C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB6A8: 386A13AC  addi r3, r10, 0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5036;
	// 826BB6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB6D0: 4BDAB751  bl 0x82466e20
	ctx.lr = 0x826BB6D4;
	sub_82466E20(ctx, base);
	// 826BB6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB6E8 size=112
    let mut pc: u32 = 0x826BB6E8;
    'dispatch: loop {
        match pc {
            0x826BB6E8 => {
    //   block [0x826BB6E8..0x826BB758)
	// 826BB6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB6F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB6F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB6FC: 392A0B04  addi r9, r10, 0xb04
	ctx.r[9].s64 = ctx.r[10].s64 + 2820;
	// 826BB700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB704: 390BE508  addi r8, r11, -0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + -6904;
	// 826BB708: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BB70C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826BB710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB720: 386A13DC  addi r3, r10, 0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5084;
	// 826BB724: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB728: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB744: 4BDAB6DD  bl 0x82466e20
	ctx.lr = 0x826BB748;
	sub_82466E20(ctx, base);
	// 826BB748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB758 size=24
    let mut pc: u32 = 0x826BB758;
    'dispatch: loop {
        match pc {
            0x826BB758 => {
    //   block [0x826BB758..0x826BB770)
	// 826BB758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB75C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB760: 394A3068  addi r10, r10, 0x3068
	ctx.r[10].s64 = ctx.r[10].s64 + 12392;
	// 826BB764: 816BE504  lwz r11, -0x1afc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6908 as u32) ) } as u64;
	// 826BB768: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BB76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB770 size=112
    let mut pc: u32 = 0x826BB770;
    'dispatch: loop {
        match pc {
            0x826BB770 => {
    //   block [0x826BB770..0x826BB7E0)
	// 826BB770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB77C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB780: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB784: 392A0B40  addi r9, r10, 0xb40
	ctx.r[9].s64 = ctx.r[10].s64 + 2880;
	// 826BB788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB78C: 390B3068  addi r8, r11, 0x3068
	ctx.r[8].s64 = ctx.r[11].s64 + 12392;
	// 826BB790: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BB794: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826BB798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB79C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB7A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB7A8: 386A140C  addi r3, r10, 0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + 5132;
	// 826BB7AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB7B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB7CC: 4BDAB655  bl 0x82466e20
	ctx.lr = 0x826BB7D0;
	sub_82466E20(ctx, base);
	// 826BB7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB7E0 size=108
    let mut pc: u32 = 0x826BB7E0;
    'dispatch: loop {
        match pc {
            0x826BB7E0 => {
    //   block [0x826BB7E0..0x826BB84C)
	// 826BB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB7EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB7F4: 38EBE5C8  addi r7, r11, -0x1a38
	ctx.r[7].s64 = ctx.r[11].s64 + -6712;
	// 826BB7F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB7FC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826BB800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB810: 386A143C  addi r3, r10, 0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + 5180;
	// 826BB814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB838: 4BDAB5E9  bl 0x82466e20
	ctx.lr = 0x826BB83C;
	sub_82466E20(ctx, base);
	// 826BB83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB850 size=108
    let mut pc: u32 = 0x826BB850;
    'dispatch: loop {
        match pc {
            0x826BB850 => {
    //   block [0x826BB850..0x826BB8BC)
	// 826BB850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB85C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB864: 38EBE5E0  addi r7, r11, -0x1a20
	ctx.r[7].s64 = ctx.r[11].s64 + -6688;
	// 826BB868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB86C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826BB870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB880: 386A146C  addi r3, r10, 0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + 5228;
	// 826BB884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB8A8: 4BDAB579  bl 0x82466e20
	ctx.lr = 0x826BB8AC;
	sub_82466E20(ctx, base);
	// 826BB8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB8C0 size=24
    let mut pc: u32 = 0x826BB8C0;
    'dispatch: loop {
        match pc {
            0x826BB8C0 => {
    //   block [0x826BB8C0..0x826BB8D8)
	// 826BB8C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB8C4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB8C8: 394A30B0  addi r10, r10, 0x30b0
	ctx.r[10].s64 = ctx.r[10].s64 + 12464;
	// 826BB8CC: 816BE610  lwz r11, -0x19f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6640 as u32) ) } as u64;
	// 826BB8D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BB8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB8D8 size=112
    let mut pc: u32 = 0x826BB8D8;
    'dispatch: loop {
        match pc {
            0x826BB8D8 => {
    //   block [0x826BB8D8..0x826BB948)
	// 826BB8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB8E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB8E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB8EC: 392A0B7C  addi r9, r10, 0xb7c
	ctx.r[9].s64 = ctx.r[10].s64 + 2940;
	// 826BB8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB8F4: 390B30B0  addi r8, r11, 0x30b0
	ctx.r[8].s64 = ctx.r[11].s64 + 12464;
	// 826BB8F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BB8FC: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826BB900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB910: 386A149C  addi r3, r10, 0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + 5276;
	// 826BB914: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB918: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB934: 4BDAB4ED  bl 0x82466e20
	ctx.lr = 0x826BB938;
	sub_82466E20(ctx, base);
	// 826BB938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB948 size=108
    let mut pc: u32 = 0x826BB948;
    'dispatch: loop {
        match pc {
            0x826BB948 => {
    //   block [0x826BB948..0x826BB9B4)
	// 826BB948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB954: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB95C: 38EBE614  addi r7, r11, -0x19ec
	ctx.r[7].s64 = ctx.r[11].s64 + -6636;
	// 826BB960: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB964: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826BB968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB96C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB978: 386A14CC  addi r3, r10, 0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5324;
	// 826BB97C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB99C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB9A0: 4BDAB481  bl 0x82466e20
	ctx.lr = 0x826BB9A4;
	sub_82466E20(ctx, base);
	// 826BB9A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB9B8 size=108
    let mut pc: u32 = 0x826BB9B8;
    'dispatch: loop {
        match pc {
            0x826BB9B8 => {
    //   block [0x826BB9B8..0x826BBA24)
	// 826BB9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB9C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB9CC: 38EBE630  addi r7, r11, -0x19d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6608;
	// 826BB9D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BB9D4: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826BB9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB9DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB9E8: 386A14FC  addi r3, r10, 0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5372;
	// 826BB9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBA10: 4BDAB411  bl 0x82466e20
	ctx.lr = 0x826BBA14;
	sub_82466E20(ctx, base);
	// 826BBA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBA28 size=108
    let mut pc: u32 = 0x826BBA28;
    'dispatch: loop {
        match pc {
            0x826BBA28 => {
    //   block [0x826BBA28..0x826BBA94)
	// 826BBA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBA34: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBA3C: 38EBE678  addi r7, r11, -0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + -6536;
	// 826BBA40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BBA44: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826BBA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBA4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBA50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBA58: 386A152C  addi r3, r10, 0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + 5420;
	// 826BBA5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBA80: 4BDAB3A1  bl 0x82466e20
	ctx.lr = 0x826BBA84;
	sub_82466E20(ctx, base);
	// 826BBA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBA98 size=108
    let mut pc: u32 = 0x826BBA98;
    'dispatch: loop {
        match pc {
            0x826BBA98 => {
    //   block [0x826BBA98..0x826BBB04)
	// 826BBA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBAA4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBAAC: 38EBE6A8  addi r7, r11, -0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + -6488;
	// 826BBAB0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826BBAB4: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826BBAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBABC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBAC8: 386A155C  addi r3, r10, 0x155c
	ctx.r[3].s64 = ctx.r[10].s64 + 5468;
	// 826BBACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBAF0: 4BDAB331  bl 0x82466e20
	ctx.lr = 0x826BBAF4;
	sub_82466E20(ctx, base);
	// 826BBAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBB08 size=108
    let mut pc: u32 = 0x826BBB08;
    'dispatch: loop {
        match pc {
            0x826BBB08 => {
    //   block [0x826BBB08..0x826BBB74)
	// 826BBB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBB14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBB1C: 38EBE7C8  addi r7, r11, -0x1838
	ctx.r[7].s64 = ctx.r[11].s64 + -6200;
	// 826BBB20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826BBB24: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826BBB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBB2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBB38: 386A158C  addi r3, r10, 0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + 5516;
	// 826BBB3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBB60: 4BDAB2C1  bl 0x82466e20
	ctx.lr = 0x826BBB64;
	sub_82466E20(ctx, base);
	// 826BBB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBB78 size=108
    let mut pc: u32 = 0x826BBB78;
    'dispatch: loop {
        match pc {
            0x826BBB78 => {
    //   block [0x826BBB78..0x826BBBE4)
	// 826BBB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBB84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBB8C: 38EBE858  addi r7, r11, -0x17a8
	ctx.r[7].s64 = ctx.r[11].s64 + -6056;
	// 826BBB90: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BBB94: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826BBB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBB9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBBA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBBA8: 386A15BC  addi r3, r10, 0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5564;
	// 826BBBAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBBCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBBD0: 4BDAB251  bl 0x82466e20
	ctx.lr = 0x826BBBD4;
	sub_82466E20(ctx, base);
	// 826BBBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBBE8 size=108
    let mut pc: u32 = 0x826BBBE8;
    'dispatch: loop {
        match pc {
            0x826BBBE8 => {
    //   block [0x826BBBE8..0x826BBC54)
	// 826BBBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBBF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBBFC: 38EBE918  addi r7, r11, -0x16e8
	ctx.r[7].s64 = ctx.r[11].s64 + -5864;
	// 826BBC00: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826BBC04: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826BBC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBC0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBC10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBC18: 386A15EC  addi r3, r10, 0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + 5612;
	// 826BBC1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBC3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBC40: 4BDAB1E1  bl 0x82466e20
	ctx.lr = 0x826BBC44;
	sub_82466E20(ctx, base);
	// 826BBC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBC58 size=108
    let mut pc: u32 = 0x826BBC58;
    'dispatch: loop {
        match pc {
            0x826BBC58 => {
    //   block [0x826BBC58..0x826BBCC4)
	// 826BBC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBC64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBC6C: 38EBE9F0  addi r7, r11, -0x1610
	ctx.r[7].s64 = ctx.r[11].s64 + -5648;
	// 826BBC70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BBC74: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826BBC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBC7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBC80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBC88: 386A161C  addi r3, r10, 0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + 5660;
	// 826BBC8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBCA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBCAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBCB0: 4BDAB171  bl 0x82466e20
	ctx.lr = 0x826BBCB4;
	sub_82466E20(ctx, base);
	// 826BBCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBCC8 size=108
    let mut pc: u32 = 0x826BBCC8;
    'dispatch: loop {
        match pc {
            0x826BBCC8 => {
    //   block [0x826BBCC8..0x826BBD34)
	// 826BBCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBCD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBCDC: 38EBEAB0  addi r7, r11, -0x1550
	ctx.r[7].s64 = ctx.r[11].s64 + -5456;
	// 826BBCE0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826BBCE4: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826BBCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBCEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBCF8: 386A164C  addi r3, r10, 0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + 5708;
	// 826BBCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBD20: 4BDAB101  bl 0x82466e20
	ctx.lr = 0x826BBD24;
	sub_82466E20(ctx, base);
	// 826BBD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBD38 size=112
    let mut pc: u32 = 0x826BBD38;
    'dispatch: loop {
        match pc {
            0x826BBD38 => {
    //   block [0x826BBD38..0x826BBDA8)
	// 826BBD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBD44: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BBD48: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826BBD4C: 38EAEB58  addi r7, r10, -0x14a8
	ctx.r[7].s64 = ctx.r[10].s64 + -5288;
	// 826BBD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBD54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BBD58: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826BBD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBD60: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBD64: 396B0B90  addi r11, r11, 0xb90
	ctx.r[11].s64 = ctx.r[11].s64 + 2960;
	// 826BBD68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBD6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBD70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBD74: 386A167C  addi r3, r10, 0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + 5756;
	// 826BBD78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBD7C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BBD80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBD84: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BBD88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBD8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBD90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBD94: 4BDAB08D  bl 0x82466e20
	ctx.lr = 0x826BBD98;
	sub_82466E20(ctx, base);
	// 826BBD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBDA8 size=108
    let mut pc: u32 = 0x826BBDA8;
    'dispatch: loop {
        match pc {
            0x826BBDA8 => {
    //   block [0x826BBDA8..0x826BBE14)
	// 826BBDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBDB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBDB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBDBC: 38EBEC78  addi r7, r11, -0x1388
	ctx.r[7].s64 = ctx.r[11].s64 + -5000;
	// 826BBDC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BBDC4: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826BBDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBDCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBDD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBDD8: 386A16AC  addi r3, r10, 0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5804;
	// 826BBDDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBDE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBDE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBDF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBDF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBDFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBE00: 4BDAB021  bl 0x82466e20
	ctx.lr = 0x826BBE04;
	sub_82466E20(ctx, base);
	// 826BBE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBE18 size=108
    let mut pc: u32 = 0x826BBE18;
    'dispatch: loop {
        match pc {
            0x826BBE18 => {
    //   block [0x826BBE18..0x826BBE84)
	// 826BBE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBE24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBE2C: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 826BBE30: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826BBE34: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826BBE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBE48: 386A16DC  addi r3, r10, 0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5852;
	// 826BBE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBE70: 4BDAAFB1  bl 0x82466e20
	ctx.lr = 0x826BBE74;
	sub_82466E20(ctx, base);
	// 826BBE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBE88 size=108
    let mut pc: u32 = 0x826BBE88;
    'dispatch: loop {
        match pc {
            0x826BBE88 => {
    //   block [0x826BBE88..0x826BBEF4)
	// 826BBE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBE94: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBE98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBE9C: 38EBEDE0  addi r7, r11, -0x1220
	ctx.r[7].s64 = ctx.r[11].s64 + -4640;
	// 826BBEA0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826BBEA4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826BBEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBEAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBEB8: 386A170C  addi r3, r10, 0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + 5900;
	// 826BBEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBEE0: 4BDAAF41  bl 0x82466e20
	ctx.lr = 0x826BBEE4;
	sub_82466E20(ctx, base);
	// 826BBEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBEF8 size=108
    let mut pc: u32 = 0x826BBEF8;
    'dispatch: loop {
        match pc {
            0x826BBEF8 => {
    //   block [0x826BBEF8..0x826BBF64)
	// 826BBEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBF04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBF0C: 38EBEEB8  addi r7, r11, -0x1148
	ctx.r[7].s64 = ctx.r[11].s64 + -4424;
	// 826BBF10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BBF14: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826BBF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBF1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBF28: 386A173C  addi r3, r10, 0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + 5948;
	// 826BBF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBF50: 4BDAAED1  bl 0x82466e20
	ctx.lr = 0x826BBF54;
	sub_82466E20(ctx, base);
	// 826BBF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBF68 size=108
    let mut pc: u32 = 0x826BBF68;
    'dispatch: loop {
        match pc {
            0x826BBF68 => {
    //   block [0x826BBF68..0x826BBFD4)
	// 826BBF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBF74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBF7C: 38EBEF00  addi r7, r11, -0x1100
	ctx.r[7].s64 = ctx.r[11].s64 + -4352;
	// 826BBF80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BBF84: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826BBF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBF8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBF90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBF98: 386A176C  addi r3, r10, 0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + 5996;
	// 826BBF9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBFA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBFC0: 4BDAAE61  bl 0x82466e20
	ctx.lr = 0x826BBFC4;
	sub_82466E20(ctx, base);
	// 826BBFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBFD8 size=108
    let mut pc: u32 = 0x826BBFD8;
    'dispatch: loop {
        match pc {
            0x826BBFD8 => {
    //   block [0x826BBFD8..0x826BC044)
	// 826BBFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBFE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBFEC: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 826BBFF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BBFF4: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826BBFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBFFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC008: 386A179C  addi r3, r10, 0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + 6044;
	// 826BC00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC030: 4BDAADF1  bl 0x82466e20
	ctx.lr = 0x826BC034;
	sub_82466E20(ctx, base);
	// 826BC034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC048 size=108
    let mut pc: u32 = 0x826BC048;
    'dispatch: loop {
        match pc {
            0x826BC048 => {
    //   block [0x826BC048..0x826BC0B4)
	// 826BC048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC054: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC05C: 38EBEF60  addi r7, r11, -0x10a0
	ctx.r[7].s64 = ctx.r[11].s64 + -4256;
	// 826BC060: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC064: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826BC068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC078: 386A17CC  addi r3, r10, 0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6092;
	// 826BC07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC0A0: 4BDAAD81  bl 0x82466e20
	ctx.lr = 0x826BC0A4;
	sub_82466E20(ctx, base);
	// 826BC0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC0B8 size=112
    let mut pc: u32 = 0x826BC0B8;
    'dispatch: loop {
        match pc {
            0x826BC0B8 => {
    //   block [0x826BC0B8..0x826BC128)
	// 826BC0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC0C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC0CC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC0D4: 390BEF78  addi r8, r11, -0x1088
	ctx.r[8].s64 = ctx.r[11].s64 + -4232;
	// 826BC0D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BC0DC: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826BC0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC0E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC0F0: 386A17FC  addi r3, r10, 0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6140;
	// 826BC0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC114: 4BDAAD0D  bl 0x82466e20
	ctx.lr = 0x826BC118;
	sub_82466E20(ctx, base);
	// 826BC118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC128 size=108
    let mut pc: u32 = 0x826BC128;
    'dispatch: loop {
        match pc {
            0x826BC128 => {
    //   block [0x826BC128..0x826BC194)
	// 826BC128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC134: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC13C: 38EBEFC0  addi r7, r11, -0x1040
	ctx.r[7].s64 = ctx.r[11].s64 + -4160;
	// 826BC140: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC144: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826BC148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC158: 386A182C  addi r3, r10, 0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + 6188;
	// 826BC15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC180: 4BDAACA1  bl 0x82466e20
	ctx.lr = 0x826BC184;
	sub_82466E20(ctx, base);
	// 826BC184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC198 size=112
    let mut pc: u32 = 0x826BC198;
    'dispatch: loop {
        match pc {
            0x826BC198 => {
    //   block [0x826BC198..0x826BC208)
	// 826BC198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC1A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC1AC: 38AA182C  addi r5, r10, 0x182c
	ctx.r[5].s64 = ctx.r[10].s64 + 6188;
	// 826BC1B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC1B4: 390BF020  addi r8, r11, -0xfe0
	ctx.r[8].s64 = ctx.r[11].s64 + -4064;
	// 826BC1B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BC1BC: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826BC1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC1D0: 386A185C  addi r3, r10, 0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + 6236;
	// 826BC1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC1F4: 4BDAAC2D  bl 0x82466e20
	ctx.lr = 0x826BC1F8;
	sub_82466E20(ctx, base);
	// 826BC1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC208 size=96
    let mut pc: u32 = 0x826BC208;
    'dispatch: loop {
        match pc {
            0x826BC208 => {
    //   block [0x826BC208..0x826BC268)
	// 826BC208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC21C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826BC220: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC228: 386A188C  addi r3, r10, 0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + 6284;
	// 826BC22C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC234: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC254: 4BDAABCD  bl 0x82466e20
	ctx.lr = 0x826BC258;
	sub_82466E20(ctx, base);
	// 826BC258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC268 size=112
    let mut pc: u32 = 0x826BC268;
    'dispatch: loop {
        match pc {
            0x826BC268 => {
    //   block [0x826BC268..0x826BC2D8)
	// 826BC268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC278: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC27C: 38AA356C  addi r5, r10, 0x356c
	ctx.r[5].s64 = ctx.r[10].s64 + 13676;
	// 826BC280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC284: 390BF068  addi r8, r11, -0xf98
	ctx.r[8].s64 = ctx.r[11].s64 + -3992;
	// 826BC288: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BC28C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826BC290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC2A0: 386A18BC  addi r3, r10, 0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6332;
	// 826BC2A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC2C4: 4BDAAB5D  bl 0x82466e20
	ctx.lr = 0x826BC2C8;
	sub_82466E20(ctx, base);
	// 826BC2C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC2D8 size=96
    let mut pc: u32 = 0x826BC2D8;
    'dispatch: loop {
        match pc {
            0x826BC2D8 => {
    //   block [0x826BC2D8..0x826BC338)
	// 826BC2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC2E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC2EC: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826BC2F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC2F8: 386A18EC  addi r3, r10, 0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6380;
	// 826BC2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC304: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC31C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC320: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC324: 4BDAAAFD  bl 0x82466e20
	ctx.lr = 0x826BC328;
	sub_82466E20(ctx, base);
	// 826BC328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC338 size=100
    let mut pc: u32 = 0x826BC338;
    'dispatch: loop {
        match pc {
            0x826BC338 => {
    //   block [0x826BC338..0x826BC39C)
	// 826BC338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC34C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC358: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826BC35C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC364: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC36C: 386A191C  addi r3, r10, 0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + 6428;
	// 826BC370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC388: 4BDAAA99  bl 0x82466e20
	ctx.lr = 0x826BC38C;
	sub_82466E20(ctx, base);
	// 826BC38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC3A0 size=96
    let mut pc: u32 = 0x826BC3A0;
    'dispatch: loop {
        match pc {
            0x826BC3A0 => {
    //   block [0x826BC3A0..0x826BC400)
	// 826BC3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC3AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC3B4: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826BC3B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC3C0: 386A194C  addi r3, r10, 0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + 6476;
	// 826BC3C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC3CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC3E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC3E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC3EC: 4BDAAA35  bl 0x82466e20
	ctx.lr = 0x826BC3F0;
	sub_82466E20(ctx, base);
	// 826BC3F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC400 size=112
    let mut pc: u32 = 0x826BC400;
    'dispatch: loop {
        match pc {
            0x826BC400 => {
    //   block [0x826BC400..0x826BC470)
	// 826BC400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC40C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC410: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC414: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC41C: 390BF0C8  addi r8, r11, -0xf38
	ctx.r[8].s64 = ctx.r[11].s64 + -3896;
	// 826BC420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BC424: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826BC428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC42C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC438: 386A197C  addi r3, r10, 0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + 6524;
	// 826BC43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC45C: 4BDAA9C5  bl 0x82466e20
	ctx.lr = 0x826BC460;
	sub_82466E20(ctx, base);
	// 826BC460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC470 size=112
    let mut pc: u32 = 0x826BC470;
    'dispatch: loop {
        match pc {
            0x826BC470 => {
    //   block [0x826BC470..0x826BC4E0)
	// 826BC470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC47C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC480: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC484: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC48C: 390BF0F8  addi r8, r11, -0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + -3848;
	// 826BC490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC494: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826BC498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC4A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC4A8: 386A19AC  addi r3, r10, 0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6572;
	// 826BC4AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC4CC: 4BDAA955  bl 0x82466e20
	ctx.lr = 0x826BC4D0;
	sub_82466E20(ctx, base);
	// 826BC4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC4E0 size=100
    let mut pc: u32 = 0x826BC4E0;
    'dispatch: loop {
        match pc {
            0x826BC4E0 => {
    //   block [0x826BC4E0..0x826BC544)
	// 826BC4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC4EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC4F4: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC500: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826BC504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC514: 386A19DC  addi r3, r10, 0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6620;
	// 826BC518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC51C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC520: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC530: 4BDAA8F1  bl 0x82466e20
	ctx.lr = 0x826BC534;
	sub_82466E20(ctx, base);
	// 826BC534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC548 size=96
    let mut pc: u32 = 0x826BC548;
    'dispatch: loop {
        match pc {
            0x826BC548 => {
    //   block [0x826BC548..0x826BC5A8)
	// 826BC548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC55C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826BC560: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC568: 386A1A0C  addi r3, r10, 0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 6668;
	// 826BC56C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC574: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC594: 4BDAA88D  bl 0x82466e20
	ctx.lr = 0x826BC598;
	sub_82466E20(ctx, base);
	// 826BC598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC5A8 size=112
    let mut pc: u32 = 0x826BC5A8;
    'dispatch: loop {
        match pc {
            0x826BC5A8 => {
    //   block [0x826BC5A8..0x826BC618)
	// 826BC5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC5B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC5BC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC5C4: 390BF110  addi r8, r11, -0xef0
	ctx.r[8].s64 = ctx.r[11].s64 + -3824;
	// 826BC5C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC5CC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826BC5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC5E0: 386A1A3C  addi r3, r10, 0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 6716;
	// 826BC5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC604: 4BDAA81D  bl 0x82466e20
	ctx.lr = 0x826BC608;
	sub_82466E20(ctx, base);
	// 826BC608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC618 size=96
    let mut pc: u32 = 0x826BC618;
    'dispatch: loop {
        match pc {
            0x826BC618 => {
    //   block [0x826BC618..0x826BC678)
	// 826BC618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC62C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826BC630: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC638: 386A1A6C  addi r3, r10, 0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 6764;
	// 826BC63C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC644: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC664: 4BDAA7BD  bl 0x82466e20
	ctx.lr = 0x826BC668;
	sub_82466E20(ctx, base);
	// 826BC668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC678 size=112
    let mut pc: u32 = 0x826BC678;
    'dispatch: loop {
        match pc {
            0x826BC678 => {
    //   block [0x826BC678..0x826BC6E8)
	// 826BC678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC688: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC68C: 38AA1A6C  addi r5, r10, 0x1a6c
	ctx.r[5].s64 = ctx.r[10].s64 + 6764;
	// 826BC690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC694: 390BF128  addi r8, r11, -0xed8
	ctx.r[8].s64 = ctx.r[11].s64 + -3800;
	// 826BC698: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC69C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826BC6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC6A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC6B0: 386A1A9C  addi r3, r10, 0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 6812;
	// 826BC6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC6D4: 4BDAA74D  bl 0x82466e20
	ctx.lr = 0x826BC6D8;
	sub_82466E20(ctx, base);
	// 826BC6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC6E8 size=108
    let mut pc: u32 = 0x826BC6E8;
    'dispatch: loop {
        match pc {
            0x826BC6E8 => {
    //   block [0x826BC6E8..0x826BC754)
	// 826BC6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC6F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC6FC: 38EBF140  addi r7, r11, -0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + -3776;
	// 826BC700: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC704: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826BC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC70C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC718: 386A1ACC  addi r3, r10, 0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + 6860;
	// 826BC71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC740: 4BDAA6E1  bl 0x82466e20
	ctx.lr = 0x826BC744;
	sub_82466E20(ctx, base);
	// 826BC744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC758 size=112
    let mut pc: u32 = 0x826BC758;
    'dispatch: loop {
        match pc {
            0x826BC758 => {
    //   block [0x826BC758..0x826BC7C8)
	// 826BC758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC768: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC76C: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BC770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC774: 390BF1A0  addi r8, r11, -0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + -3680;
	// 826BC778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC77C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826BC780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC790: 386A1AFC  addi r3, r10, 0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + 6908;
	// 826BC794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC7B4: 4BDAA66D  bl 0x82466e20
	ctx.lr = 0x826BC7B8;
	sub_82466E20(ctx, base);
	// 826BC7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC7C8 size=112
    let mut pc: u32 = 0x826BC7C8;
    'dispatch: loop {
        match pc {
            0x826BC7C8 => {
    //   block [0x826BC7C8..0x826BC838)
	// 826BC7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC7D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC7D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC7DC: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC7E4: 390BF1B8  addi r8, r11, -0xe48
	ctx.r[8].s64 = ctx.r[11].s64 + -3656;
	// 826BC7E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BC7EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826BC7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC7F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC800: 386A1B2C  addi r3, r10, 0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6956;
	// 826BC804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC824: 4BDAA5FD  bl 0x82466e20
	ctx.lr = 0x826BC828;
	sub_82466E20(ctx, base);
	// 826BC828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC838 size=112
    let mut pc: u32 = 0x826BC838;
    'dispatch: loop {
        match pc {
            0x826BC838 => {
    //   block [0x826BC838..0x826BC8A8)
	// 826BC838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC848: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC84C: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC854: 390BF1E8  addi r8, r11, -0xe18
	ctx.r[8].s64 = ctx.r[11].s64 + -3608;
	// 826BC858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC85C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826BC860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC870: 386A1B5C  addi r3, r10, 0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7004;
	// 826BC874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC894: 4BDAA58D  bl 0x82466e20
	ctx.lr = 0x826BC898;
	sub_82466E20(ctx, base);
	// 826BC898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC8A8 size=116
    let mut pc: u32 = 0x826BC8A8;
    'dispatch: loop {
        match pc {
            0x826BC8A8 => {
    //   block [0x826BC8A8..0x826BC91C)
	// 826BC8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC8B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC8B8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BC8BC: 390BF200  addi r8, r11, -0xe00
	ctx.r[8].s64 = ctx.r[11].s64 + -3584;
	// 826BC8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC8C4: 392A0C08  addi r9, r10, 0xc08
	ctx.r[9].s64 = ctx.r[10].s64 + 3080;
	// 826BC8C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC8CC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826BC8D0: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BC8D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC8DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC8EC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BC8F0: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826BC8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BC8F8: 386B1B8C  addi r3, r11, 0x1b8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7052;
	// 826BC8FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BC900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC908: 4BDAA519  bl 0x82466e20
	ctx.lr = 0x826BC90C;
	sub_82466E20(ctx, base);
	// 826BC90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC920 size=112
    let mut pc: u32 = 0x826BC920;
    'dispatch: loop {
        match pc {
            0x826BC920 => {
    //   block [0x826BC920..0x826BC990)
	// 826BC920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC92C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC930: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC934: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC93C: 390BF230  addi r8, r11, -0xdd0
	ctx.r[8].s64 = ctx.r[11].s64 + -3536;
	// 826BC940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC944: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826BC948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC94C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC958: 386A1BBC  addi r3, r10, 0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7100;
	// 826BC95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC96C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC97C: 4BDAA4A5  bl 0x82466e20
	ctx.lr = 0x826BC980;
	sub_82466E20(ctx, base);
	// 826BC980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC990 size=112
    let mut pc: u32 = 0x826BC990;
    'dispatch: loop {
        match pc {
            0x826BC990 => {
    //   block [0x826BC990..0x826BCA00)
	// 826BC990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC99C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC9A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC9A4: 38AA209C  addi r5, r10, 0x209c
	ctx.r[5].s64 = ctx.r[10].s64 + 8348;
	// 826BC9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC9AC: 390BF248  addi r8, r11, -0xdb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3512;
	// 826BC9B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC9B4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826BC9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC9BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC9C8: 386A1BEC  addi r3, r10, 0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + 7148;
	// 826BC9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC9EC: 4BDAA435  bl 0x82466e20
	ctx.lr = 0x826BC9F0;
	sub_82466E20(ctx, base);
	// 826BC9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCA00 size=112
    let mut pc: u32 = 0x826BCA00;
    'dispatch: loop {
        match pc {
            0x826BCA00 => {
    //   block [0x826BCA00..0x826BCA70)
	// 826BCA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCA0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCA14: 38AA1DFC  addi r5, r10, 0x1dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 7676;
	// 826BCA18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCA1C: 390BF260  addi r8, r11, -0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + -3488;
	// 826BCA20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BCA24: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826BCA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCA2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCA38: 386A1C1C  addi r3, r10, 0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7196;
	// 826BCA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCA5C: 4BDAA3C5  bl 0x82466e20
	ctx.lr = 0x826BCA60;
	sub_82466E20(ctx, base);
	// 826BCA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCA70 size=112
    let mut pc: u32 = 0x826BCA70;
    'dispatch: loop {
        match pc {
            0x826BCA70 => {
    //   block [0x826BCA70..0x826BCAE0)
	// 826BCA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCA7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCA84: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCA8C: 390BF278  addi r8, r11, -0xd88
	ctx.r[8].s64 = ctx.r[11].s64 + -3464;
	// 826BCA90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BCA94: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826BCA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCA9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCAA8: 386A1C4C  addi r3, r10, 0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7244;
	// 826BCAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCACC: 4BDAA355  bl 0x82466e20
	ctx.lr = 0x826BCAD0;
	sub_82466E20(ctx, base);
	// 826BCAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCAE0 size=112
    let mut pc: u32 = 0x826BCAE0;
    'dispatch: loop {
        match pc {
            0x826BCAE0 => {
    //   block [0x826BCAE0..0x826BCB50)
	// 826BCAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCAEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCAF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCAF4: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCAFC: 390BF2C0  addi r8, r11, -0xd40
	ctx.r[8].s64 = ctx.r[11].s64 + -3392;
	// 826BCB00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BCB04: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826BCB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCB18: 386A1C7C  addi r3, r10, 0x1c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7292;
	// 826BCB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCB3C: 4BDAA2E5  bl 0x82466e20
	ctx.lr = 0x826BCB40;
	sub_82466E20(ctx, base);
	// 826BCB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCB50 size=112
    let mut pc: u32 = 0x826BCB50;
    'dispatch: loop {
        match pc {
            0x826BCB50 => {
    //   block [0x826BCB50..0x826BCBC0)
	// 826BCB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCB5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCB64: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCB6C: 390BF2F0  addi r8, r11, -0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + -3344;
	// 826BCB70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BCB74: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826BCB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCB88: 386A1CAC  addi r3, r10, 0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + 7340;
	// 826BCB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCBAC: 4BDAA275  bl 0x82466e20
	ctx.lr = 0x826BCBB0;
	sub_82466E20(ctx, base);
	// 826BCBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCBC0 size=108
    let mut pc: u32 = 0x826BCBC0;
    'dispatch: loop {
        match pc {
            0x826BCBC0 => {
    //   block [0x826BCBC0..0x826BCC2C)
	// 826BCBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCBCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCBD4: 38EBF320  addi r7, r11, -0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + -3296;
	// 826BCBD8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BCBDC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826BCBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCBE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCBE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCBF0: 386A1CDC  addi r3, r10, 0x1cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 7388;
	// 826BCBF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCC18: 4BDAA209  bl 0x82466e20
	ctx.lr = 0x826BCC1C;
	sub_82466E20(ctx, base);
	// 826BCC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCC30 size=112
    let mut pc: u32 = 0x826BCC30;
    'dispatch: loop {
        match pc {
            0x826BCC30 => {
    //   block [0x826BCC30..0x826BCCA0)
	// 826BCC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCC3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCC40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCC44: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCC4C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 826BCC50: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826BCC54: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826BCC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCC5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCC68: 386A1D0C  addi r3, r10, 0x1d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7436;
	// 826BCC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCC8C: 4BDAA195  bl 0x82466e20
	ctx.lr = 0x826BCC90;
	sub_82466E20(ctx, base);
	// 826BCC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCCA0 size=116
    let mut pc: u32 = 0x826BCCA0;
    'dispatch: loop {
        match pc {
            0x826BCCA0 => {
    //   block [0x826BCCA0..0x826BCD14)
	// 826BCCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCCAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BCCB0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCCB4: 392B0C44  addi r9, r11, 0xc44
	ctx.r[9].s64 = ctx.r[11].s64 + 3140;
	// 826BCCB8: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCCBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCCC0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826BCCC4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826BCCC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCCCC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826BCCD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCCD4: 396BF3E8  addi r11, r11, -0xc18
	ctx.r[11].s64 = ctx.r[11].s64 + -3096;
	// 826BCCD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BCCDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCCE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BCCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCCE8: 386A1D3C  addi r3, r10, 0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7484;
	// 826BCCEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BCCF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BCCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCCF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BCCFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCD00: 4BDAA121  bl 0x82466e20
	ctx.lr = 0x826BCD04;
	sub_82466E20(ctx, base);
	// 826BCD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCD18 size=100
    let mut pc: u32 = 0x826BCD18;
    'dispatch: loop {
        match pc {
            0x826BCD18 => {
    //   block [0x826BCD18..0x826BCD7C)
	// 826BCD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCD2C: 38AA1E8C  addi r5, r10, 0x1e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7820;
	// 826BCD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCD38: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826BCD3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCD4C: 386A1D6C  addi r3, r10, 0x1d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7532;
	// 826BCD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCD54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCD58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BCD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCD60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCD68: 4BDAA0B9  bl 0x82466e20
	ctx.lr = 0x826BCD6C;
	sub_82466E20(ctx, base);
	// 826BCD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCD80 size=100
    let mut pc: u32 = 0x826BCD80;
    'dispatch: loop {
        match pc {
            0x826BCD80 => {
    //   block [0x826BCD80..0x826BCDE4)
	// 826BCD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCD8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCD94: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BCD98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCDA0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826BCDA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCDB4: 386A1D9C  addi r3, r10, 0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7580;
	// 826BCDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCDBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCDC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BCDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCDC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCDD0: 4BDAA051  bl 0x82466e20
	ctx.lr = 0x826BCDD4;
	sub_82466E20(ctx, base);
	// 826BCDD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCDE8 size=108
    let mut pc: u32 = 0x826BCDE8;
    'dispatch: loop {
        match pc {
            0x826BCDE8 => {
    //   block [0x826BCDE8..0x826BCE54)
	// 826BCDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCDF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCDFC: 38EBF478  addi r7, r11, -0xb88
	ctx.r[7].s64 = ctx.r[11].s64 + -2952;
	// 826BCE00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BCE04: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826BCE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCE0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCE18: 386A1DCC  addi r3, r10, 0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 7628;
	// 826BCE1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCE3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCE40: 4BDA9FE1  bl 0x82466e20
	ctx.lr = 0x826BCE44;
	sub_82466E20(ctx, base);
	// 826BCE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCE58 size=112
    let mut pc: u32 = 0x826BCE58;
    'dispatch: loop {
        match pc {
            0x826BCE58 => {
    //   block [0x826BCE58..0x826BCEC8)
	// 826BCE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCE64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCE6C: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCE74: 390BF4A8  addi r8, r11, -0xb58
	ctx.r[8].s64 = ctx.r[11].s64 + -2904;
	// 826BCE78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BCE7C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826BCE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCE84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCE90: 386A1DFC  addi r3, r10, 0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7676;
	// 826BCE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCEB4: 4BDA9F6D  bl 0x82466e20
	ctx.lr = 0x826BCEB8;
	sub_82466E20(ctx, base);
	// 826BCEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCEC8 size=108
    let mut pc: u32 = 0x826BCEC8;
    'dispatch: loop {
        match pc {
            0x826BCEC8 => {
    //   block [0x826BCEC8..0x826BCF34)
	// 826BCEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCED4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCEDC: 38EBF4C0  addi r7, r11, -0xb40
	ctx.r[7].s64 = ctx.r[11].s64 + -2880;
	// 826BCEE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BCEE4: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826BCEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCEEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCEF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCEF8: 386A1E2C  addi r3, r10, 0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7724;
	// 826BCEFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCF1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCF20: 4BDA9F01  bl 0x82466e20
	ctx.lr = 0x826BCF24;
	sub_82466E20(ctx, base);
	// 826BCF24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BCF38 size=28
    let mut pc: u32 = 0x826BCF38;
    'dispatch: loop {
        match pc {
            0x826BCF38 => {
    //   block [0x826BCF38..0x826BCF54)
	// 826BCF38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCF3C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BCF40: 394A3128  addi r10, r10, 0x3128
	ctx.r[10].s64 = ctx.r[10].s64 + 12584;
	// 826BCF44: 816BF3E4  lwz r11, -0xc1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3100 as u32) ) } as u64;
	// 826BCF48: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826BCF4C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826BCF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCF58 size=108
    let mut pc: u32 = 0x826BCF58;
    'dispatch: loop {
        match pc {
            0x826BCF58 => {
    //   block [0x826BCF58..0x826BCFC4)
	// 826BCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCF64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCF6C: 38EB3128  addi r7, r11, 0x3128
	ctx.r[7].s64 = ctx.r[11].s64 + 12584;
	// 826BCF70: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826BCF74: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826BCF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCF7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCF88: 386A1E5C  addi r3, r10, 0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7772;
	// 826BCF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCFB0: 4BDA9E71  bl 0x82466e20
	ctx.lr = 0x826BCFB4;
	sub_82466E20(ctx, base);
	// 826BCFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCFC8 size=116
    let mut pc: u32 = 0x826BCFC8;
    'dispatch: loop {
        match pc {
            0x826BCFC8 => {
    //   block [0x826BCFC8..0x826BD03C)
	// 826BCFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCFD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCFD8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BCFDC: 390BF4E0  addi r8, r11, -0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + -2848;
	// 826BCFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCFE4: 392A0CB4  addi r9, r10, 0xcb4
	ctx.r[9].s64 = ctx.r[10].s64 + 3252;
	// 826BCFE8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCFEC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BCFF0: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCFF4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCFFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD00C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BD010: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826BD014: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BD018: 386B1E8C  addi r3, r11, 0x1e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7820;
	// 826BD01C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BD020: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD028: 4BDA9DF9  bl 0x82466e20
	ctx.lr = 0x826BD02C;
	sub_82466E20(ctx, base);
	// 826BD02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD040 size=112
    let mut pc: u32 = 0x826BD040;
    'dispatch: loop {
        match pc {
            0x826BD040 => {
    //   block [0x826BD040..0x826BD0B0)
	// 826BD040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD04C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD050: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD054: 38AA1B5C  addi r5, r10, 0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + 7004;
	// 826BD058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD05C: 390BF558  addi r8, r11, -0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + -2728;
	// 826BD060: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD064: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826BD068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD078: 386A1EBC  addi r3, r10, 0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 7868;
	// 826BD07C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD09C: 4BDA9D85  bl 0x82466e20
	ctx.lr = 0x826BD0A0;
	sub_82466E20(ctx, base);
	// 826BD0A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD0B0 size=108
    let mut pc: u32 = 0x826BD0B0;
    'dispatch: loop {
        match pc {
            0x826BD0B0 => {
    //   block [0x826BD0B0..0x826BD11C)
	// 826BD0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD0BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD0C4: 38EBF570  addi r7, r11, -0xa90
	ctx.r[7].s64 = ctx.r[11].s64 + -2704;
	// 826BD0C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BD0CC: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826BD0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD0D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD0E0: 386A1EEC  addi r3, r10, 0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + 7916;
	// 826BD0E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD108: 4BDA9D19  bl 0x82466e20
	ctx.lr = 0x826BD10C;
	sub_82466E20(ctx, base);
	// 826BD10C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD120 size=112
    let mut pc: u32 = 0x826BD120;
    'dispatch: loop {
        match pc {
            0x826BD120 => {
    //   block [0x826BD120..0x826BD190)
	// 826BD120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD130: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD134: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD13C: 390BF5A0  addi r8, r11, -0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + -2656;
	// 826BD140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD144: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826BD148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD158: 386A1F1C  addi r3, r10, 0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7964;
	// 826BD15C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD17C: 4BDA9CA5  bl 0x82466e20
	ctx.lr = 0x826BD180;
	sub_82466E20(ctx, base);
	// 826BD180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD190 size=112
    let mut pc: u32 = 0x826BD190;
    'dispatch: loop {
        match pc {
            0x826BD190 => {
    //   block [0x826BD190..0x826BD200)
	// 826BD190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD1A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD1A4: 38AA209C  addi r5, r10, 0x209c
	ctx.r[5].s64 = ctx.r[10].s64 + 8348;
	// 826BD1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD1AC: 390BF5D0  addi r8, r11, -0xa30
	ctx.r[8].s64 = ctx.r[11].s64 + -2608;
	// 826BD1B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD1B4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826BD1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD1BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD1C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD1C8: 386A1F4C  addi r3, r10, 0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 8012;
	// 826BD1CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD1EC: 4BDA9C35  bl 0x82466e20
	ctx.lr = 0x826BD1F0;
	sub_82466E20(ctx, base);
	// 826BD1F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD200 size=100
    let mut pc: u32 = 0x826BD200;
    'dispatch: loop {
        match pc {
            0x826BD200 => {
    //   block [0x826BD200..0x826BD264)
	// 826BD200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD214: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD220: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826BD224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD234: 386A1F7C  addi r3, r10, 0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 8060;
	// 826BD238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD23C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD240: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD248: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD250: 4BDA9BD1  bl 0x82466e20
	ctx.lr = 0x826BD254;
	sub_82466E20(ctx, base);
	// 826BD254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD268 size=112
    let mut pc: u32 = 0x826BD268;
    'dispatch: loop {
        match pc {
            0x826BD268 => {
    //   block [0x826BD268..0x826BD2D8)
	// 826BD268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD278: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD27C: 38AA1D9C  addi r5, r10, 0x1d9c
	ctx.r[5].s64 = ctx.r[10].s64 + 7580;
	// 826BD280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD284: 390BF600  addi r8, r11, -0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + -2560;
	// 826BD288: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD28C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826BD290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD2A0: 386A1FAC  addi r3, r10, 0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + 8108;
	// 826BD2A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD2C4: 4BDA9B5D  bl 0x82466e20
	ctx.lr = 0x826BD2C8;
	sub_82466E20(ctx, base);
	// 826BD2C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD2D8 size=112
    let mut pc: u32 = 0x826BD2D8;
    'dispatch: loop {
        match pc {
            0x826BD2D8 => {
    //   block [0x826BD2D8..0x826BD348)
	// 826BD2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD2E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD2E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD2EC: 38AA1D9C  addi r5, r10, 0x1d9c
	ctx.r[5].s64 = ctx.r[10].s64 + 7580;
	// 826BD2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD2F4: 390BF648  addi r8, r11, -0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2488;
	// 826BD2F8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826BD2FC: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826BD300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD310: 386A1FDC  addi r3, r10, 0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 8156;
	// 826BD314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD334: 4BDA9AED  bl 0x82466e20
	ctx.lr = 0x826BD338;
	sub_82466E20(ctx, base);
	// 826BD338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD348 size=108
    let mut pc: u32 = 0x826BD348;
    'dispatch: loop {
        match pc {
            0x826BD348 => {
    //   block [0x826BD348..0x826BD3B4)
	// 826BD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD354: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD35C: 38EBF6F0  addi r7, r11, -0x910
	ctx.r[7].s64 = ctx.r[11].s64 + -2320;
	// 826BD360: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BD364: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826BD368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD36C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD378: 386A200C  addi r3, r10, 0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + 8204;
	// 826BD37C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD39C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD3A0: 4BDA9A81  bl 0x82466e20
	ctx.lr = 0x826BD3A4;
	sub_82466E20(ctx, base);
	// 826BD3A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD3A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD3AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD3B8 size=112
    let mut pc: u32 = 0x826BD3B8;
    'dispatch: loop {
        match pc {
            0x826BD3B8 => {
    //   block [0x826BD3B8..0x826BD428)
	// 826BD3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD3C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD3C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD3CC: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BD3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD3D4: 390BF738  addi r8, r11, -0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + -2248;
	// 826BD3D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BD3DC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826BD3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD3E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD3F0: 386A203C  addi r3, r10, 0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + 8252;
	// 826BD3F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD414: 4BDA9A0D  bl 0x82466e20
	ctx.lr = 0x826BD418;
	sub_82466E20(ctx, base);
	// 826BD418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD428 size=100
    let mut pc: u32 = 0x826BD428;
    'dispatch: loop {
        match pc {
            0x826BD428 => {
    //   block [0x826BD428..0x826BD48C)
	// 826BD428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD434: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD43C: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BD440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD448: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826BD44C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD45C: 386A206C  addi r3, r10, 0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + 8300;
	// 826BD460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD468: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD470: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD478: 4BDA99A9  bl 0x82466e20
	ctx.lr = 0x826BD47C;
	sub_82466E20(ctx, base);
	// 826BD47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD490 size=100
    let mut pc: u32 = 0x826BD490;
    'dispatch: loop {
        match pc {
            0x826BD490 => {
    //   block [0x826BD490..0x826BD4F4)
	// 826BD490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD4A4: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD4B0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826BD4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD4C4: 386A209C  addi r3, r10, 0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + 8348;
	// 826BD4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD4D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD4D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD4E0: 4BDA9941  bl 0x82466e20
	ctx.lr = 0x826BD4E4;
	sub_82466E20(ctx, base);
	// 826BD4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD4F8 size=112
    let mut pc: u32 = 0x826BD4F8;
    'dispatch: loop {
        match pc {
            0x826BD4F8 => {
    //   block [0x826BD4F8..0x826BD568)
	// 826BD4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD50C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BD510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD514: 390BF798  addi r8, r11, -0x868
	ctx.r[8].s64 = ctx.r[11].s64 + -2152;
	// 826BD518: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BD51C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826BD520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD530: 386A20CC  addi r3, r10, 0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8396;
	// 826BD534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD554: 4BDA98CD  bl 0x82466e20
	ctx.lr = 0x826BD558;
	sub_82466E20(ctx, base);
	// 826BD558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD568 size=112
    let mut pc: u32 = 0x826BD568;
    'dispatch: loop {
        match pc {
            0x826BD568 => {
    //   block [0x826BD568..0x826BD5D8)
	// 826BD568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD578: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD57C: 38AA1E8C  addi r5, r10, 0x1e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7820;
	// 826BD580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD584: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 826BD588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD58C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826BD590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD5A0: 386A20FC  addi r3, r10, 0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8444;
	// 826BD5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD5C4: 4BDA985D  bl 0x82466e20
	ctx.lr = 0x826BD5C8;
	sub_82466E20(ctx, base);
	// 826BD5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD5D8 size=112
    let mut pc: u32 = 0x826BD5D8;
    'dispatch: loop {
        match pc {
            0x826BD5D8 => {
    //   block [0x826BD5D8..0x826BD648)
	// 826BD5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD5E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD5E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD5EC: 38AA1FDC  addi r5, r10, 0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + 8156;
	// 826BD5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD5F4: 390BF840  addi r8, r11, -0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + -1984;
	// 826BD5F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD5FC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826BD600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD610: 386A212C  addi r3, r10, 0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + 8492;
	// 826BD614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD634: 4BDA97ED  bl 0x82466e20
	ctx.lr = 0x826BD638;
	sub_82466E20(ctx, base);
	// 826BD638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD648 size=112
    let mut pc: u32 = 0x826BD648;
    'dispatch: loop {
        match pc {
            0x826BD648 => {
    //   block [0x826BD648..0x826BD6B8)
	// 826BD648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD65C: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD664: 390BF870  addi r8, r11, -0x790
	ctx.r[8].s64 = ctx.r[11].s64 + -1936;
	// 826BD668: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD66C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826BD670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD680: 386A215C  addi r3, r10, 0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + 8540;
	// 826BD684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD6A4: 4BDA977D  bl 0x82466e20
	ctx.lr = 0x826BD6A8;
	sub_82466E20(ctx, base);
	// 826BD6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD6B8 size=112
    let mut pc: u32 = 0x826BD6B8;
    'dispatch: loop {
        match pc {
            0x826BD6B8 => {
    //   block [0x826BD6B8..0x826BD728)
	// 826BD6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD6C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD6C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD6CC: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BD6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD6D4: 390BF8B8  addi r8, r11, -0x748
	ctx.r[8].s64 = ctx.r[11].s64 + -1864;
	// 826BD6D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD6DC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826BD6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD6E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD6E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD6F0: 386A218C  addi r3, r10, 0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + 8588;
	// 826BD6F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD714: 4BDA970D  bl 0x82466e20
	ctx.lr = 0x826BD718;
	sub_82466E20(ctx, base);
	// 826BD718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD728 size=112
    let mut pc: u32 = 0x826BD728;
    'dispatch: loop {
        match pc {
            0x826BD728 => {
    //   block [0x826BD728..0x826BD798)
	// 826BD728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD738: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD73C: 38AA1B5C  addi r5, r10, 0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + 7004;
	// 826BD740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD744: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 826BD748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD74C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826BD750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD760: 386A21BC  addi r3, r10, 0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8636;
	// 826BD764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD784: 4BDA969D  bl 0x82466e20
	ctx.lr = 0x826BD788;
	sub_82466E20(ctx, base);
	// 826BD788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD798 size=112
    let mut pc: u32 = 0x826BD798;
    'dispatch: loop {
        match pc {
            0x826BD798 => {
    //   block [0x826BD798..0x826BD808)
	// 826BD798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD7A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD7A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD7AC: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BD7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD7B4: 390BF918  addi r8, r11, -0x6e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1768;
	// 826BD7B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD7BC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826BD7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD7C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD7D0: 386A21EC  addi r3, r10, 0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8684;
	// 826BD7D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD7F4: 4BDA962D  bl 0x82466e20
	ctx.lr = 0x826BD7F8;
	sub_82466E20(ctx, base);
	// 826BD7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BD808 size=24
    let mut pc: u32 = 0x826BD808;
    'dispatch: loop {
        match pc {
            0x826BD808 => {
    //   block [0x826BD808..0x826BD820)
	// 826BD808: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD80C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BD810: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 826BD814: 816BF4DC  lwz r11, -0xb24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2852 as u32) ) } as u64;
	// 826BD818: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BD81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD820 size=112
    let mut pc: u32 = 0x826BD820;
    'dispatch: loop {
        match pc {
            0x826BD820 => {
    //   block [0x826BD820..0x826BD890)
	// 826BD820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD82C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BD830: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD834: 392A0DD8  addi r9, r10, 0xdd8
	ctx.r[9].s64 = ctx.r[10].s64 + 3544;
	// 826BD838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD83C: 390B3260  addi r8, r11, 0x3260
	ctx.r[8].s64 = ctx.r[11].s64 + 12896;
	// 826BD840: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BD844: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826BD848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD84C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD858: 386A221C  addi r3, r10, 0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + 8732;
	// 826BD85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BD860: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BD864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD87C: 4BDA95A5  bl 0x82466e20
	ctx.lr = 0x826BD880;
	sub_82466E20(ctx, base);
	// 826BD880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD890 size=112
    let mut pc: u32 = 0x826BD890;
    'dispatch: loop {
        match pc {
            0x826BD890 => {
    //   block [0x826BD890..0x826BD900)
	// 826BD890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD89C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD8A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD8A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BD8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD8AC: 390BF94C  addi r8, r11, -0x6b4
	ctx.r[8].s64 = ctx.r[11].s64 + -1716;
	// 826BD8B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD8B4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826BD8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD8BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD8C8: 386A224C  addi r3, r10, 0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + 8780;
	// 826BD8CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD8EC: 4BDA9535  bl 0x82466e20
	ctx.lr = 0x826BD8F0;
	sub_82466E20(ctx, base);
	// 826BD8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD900 size=108
    let mut pc: u32 = 0x826BD900;
    'dispatch: loop {
        match pc {
            0x826BD900 => {
    //   block [0x826BD900..0x826BD96C)
	// 826BD900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD90C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD914: 38EBF97C  addi r7, r11, -0x684
	ctx.r[7].s64 = ctx.r[11].s64 + -1668;
	// 826BD918: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BD91C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826BD920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD930: 386A227C  addi r3, r10, 0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + 8828;
	// 826BD934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD958: 4BDA94C9  bl 0x82466e20
	ctx.lr = 0x826BD95C;
	sub_82466E20(ctx, base);
	// 826BD95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD970 size=100
    let mut pc: u32 = 0x826BD970;
    'dispatch: loop {
        match pc {
            0x826BD970 => {
    //   block [0x826BD970..0x826BD9D4)
	// 826BD970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD97C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD984: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BD988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD990: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826BD994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD9A4: 386A22AC  addi r3, r10, 0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8876;
	// 826BD9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD9B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD9B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD9C0: 4BDA9461  bl 0x82466e20
	ctx.lr = 0x826BD9C4;
	sub_82466E20(ctx, base);
	// 826BD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


