pub fn sub_826D9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9E80 size=112
    let mut pc: u32 = 0x826D9E80;
    'dispatch: loop {
        match pc {
            0x826D9E80 => {
    //   block [0x826D9E80..0x826D9EF0)
	// 826D9E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9E8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9E94: 38AAE024  addi r5, r10, -0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -8156;
	// 826D9E98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9E9C: 390B5098  addi r8, r11, 0x5098
	ctx.r[8].s64 = ctx.r[11].s64 + 20632;
	// 826D9EA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9EA4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 826D9EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9EB8: 386AE174  addi r3, r10, -0x1e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7820;
	// 826D9EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9EDC: 4BD8CF45  bl 0x82466e20
	ctx.lr = 0x826D9EE0;
	sub_82466E20(ctx, base);
	// 826D9EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9EF0 size=100
    let mut pc: u32 = 0x826D9EF0;
    'dispatch: loop {
        match pc {
            0x826D9EF0 => {
    //   block [0x826D9EF0..0x826D9F54)
	// 826D9EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9EFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9F04: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826D9F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9F10: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 826D9F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9F24: 386AE1A4  addi r3, r10, -0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7772;
	// 826D9F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9F2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9F30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9F38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9F40: 4BD8CEE1  bl 0x82466e20
	ctx.lr = 0x826D9F44;
	sub_82466E20(ctx, base);
	// 826D9F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9F58 size=112
    let mut pc: u32 = 0x826D9F58;
    'dispatch: loop {
        match pc {
            0x826D9F58 => {
    //   block [0x826D9F58..0x826D9FC8)
	// 826D9F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9F64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9F6C: 38AAE1A4  addi r5, r10, -0x1e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7772;
	// 826D9F70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9F74: 390B50B0  addi r8, r11, 0x50b0
	ctx.r[8].s64 = ctx.r[11].s64 + 20656;
	// 826D9F78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D9F7C: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 826D9F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9F90: 386AE1D4  addi r3, r10, -0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7724;
	// 826D9F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9FB4: 4BD8CE6D  bl 0x82466e20
	ctx.lr = 0x826D9FB8;
	sub_82466E20(ctx, base);
	// 826D9FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9FC8 size=112
    let mut pc: u32 = 0x826D9FC8;
    'dispatch: loop {
        match pc {
            0x826D9FC8 => {
    //   block [0x826D9FC8..0x826DA038)
	// 826D9FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9FD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9FD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9FDC: 38AAE1D4  addi r5, r10, -0x1e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -7724;
	// 826D9FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9FE4: 390B5110  addi r8, r11, 0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + 20752;
	// 826D9FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D9FEC: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 826D9FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9FF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA000: 386AE204  addi r3, r10, -0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7676;
	// 826DA004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA024: 4BD8CDFD  bl 0x82466e20
	ctx.lr = 0x826DA028;
	sub_82466E20(ctx, base);
	// 826DA028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DA038 size=24
    let mut pc: u32 = 0x826DA038;
    'dispatch: loop {
        match pc {
            0x826DA038 => {
    //   block [0x826DA038..0x826DA050)
	// 826DA038: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA03C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA040: 394AB588  addi r10, r10, -0x4a78
	ctx.r[10].s64 = ctx.r[10].s64 + -19064;
	// 826DA044: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826DA048: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826DA04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA050 size=116
    let mut pc: u32 = 0x826DA050;
    'dispatch: loop {
        match pc {
            0x826DA050 => {
    //   block [0x826DA050..0x826DA0C4)
	// 826DA050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA05C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA060: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DA064: 390AB588  addi r8, r10, -0x4a78
	ctx.r[8].s64 = ctx.r[10].s64 + -19064;
	// 826DA068: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA06C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA070: 38AAE1D4  addi r5, r10, -0x1e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -7724;
	// 826DA074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA078: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA084: 388AB78C  addi r4, r10, -0x4874
	ctx.r[4].s64 = ctx.r[10].s64 + -18548;
	// 826DA088: 396B53A8  addi r11, r11, 0x53a8
	ctx.r[11].s64 = ctx.r[11].s64 + 21416;
	// 826DA08C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DA094: 386AE234  addi r3, r10, -0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -7628;
	// 826DA098: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DA09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA0A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DA0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA0B0: 4BD8CD71  bl 0x82466e20
	ctx.lr = 0x826DA0B4;
	sub_82466E20(ctx, base);
	// 826DA0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA0C8 size=112
    let mut pc: u32 = 0x826DA0C8;
    'dispatch: loop {
        match pc {
            0x826DA0C8 => {
    //   block [0x826DA0C8..0x826DA138)
	// 826DA0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA0D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA0DC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DA0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA0E4: 390B5140  addi r8, r11, 0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + 20800;
	// 826DA0E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA0EC: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 826DA0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA100: 386AE264  addi r3, r10, -0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7580;
	// 826DA104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DA118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA124: 4BD8CCFD  bl 0x82466e20
	ctx.lr = 0x826DA128;
	sub_82466E20(ctx, base);
	// 826DA128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA138 size=116
    let mut pc: u32 = 0x826DA138;
    'dispatch: loop {
        match pc {
            0x826DA138 => {
    //   block [0x826DA138..0x826DA1AC)
	// 826DA138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA144: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA148: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DA14C: 390B5174  addi r8, r11, 0x5174
	ctx.r[8].s64 = ctx.r[11].s64 + 20852;
	// 826DA150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA154: 392A53E8  addi r9, r10, 0x53e8
	ctx.r[9].s64 = ctx.r[10].s64 + 21480;
	// 826DA158: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA15C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DA160: 38AAE534  addi r5, r10, -0x1acc
	ctx.r[5].s64 = ctx.r[10].s64 + -6860;
	// 826DA164: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA17C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DA180: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 826DA184: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA188: 386BE294  addi r3, r11, -0x1d6c
	ctx.r[3].s64 = ctx.r[11].s64 + -7532;
	// 826DA18C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA190: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA198: 4BD8CC89  bl 0x82466e20
	ctx.lr = 0x826DA19C;
	sub_82466E20(ctx, base);
	// 826DA19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA1B0 size=112
    let mut pc: u32 = 0x826DA1B0;
    'dispatch: loop {
        match pc {
            0x826DA1B0 => {
    //   block [0x826DA1B0..0x826DA220)
	// 826DA1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA1BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA1C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA1C4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA1C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA1CC: 390B518C  addi r8, r11, 0x518c
	ctx.r[8].s64 = ctx.r[11].s64 + 20876;
	// 826DA1D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DA1D4: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 826DA1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA1DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA1E8: 386AE2C4  addi r3, r10, -0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7484;
	// 826DA1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA20C: 4BD8CC15  bl 0x82466e20
	ctx.lr = 0x826DA210;
	sub_82466E20(ctx, base);
	// 826DA210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA220 size=116
    let mut pc: u32 = 0x826DA220;
    'dispatch: loop {
        match pc {
            0x826DA220 => {
    //   block [0x826DA220..0x826DA294)
	// 826DA220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA22C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA230: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DA234: 390B51A8  addi r8, r11, 0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + 20904;
	// 826DA238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA23C: 392A5414  addi r9, r10, 0x5414
	ctx.r[9].s64 = ctx.r[10].s64 + 21524;
	// 826DA240: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA244: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826DA248: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA24C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA254: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA264: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DA268: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 826DA26C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA270: 386BE2F4  addi r3, r11, -0x1d0c
	ctx.r[3].s64 = ctx.r[11].s64 + -7436;
	// 826DA274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA278: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA280: 4BD8CBA1  bl 0x82466e20
	ctx.lr = 0x826DA284;
	sub_82466E20(ctx, base);
	// 826DA284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA298 size=112
    let mut pc: u32 = 0x826DA298;
    'dispatch: loop {
        match pc {
            0x826DA298 => {
    //   block [0x826DA298..0x826DA308)
	// 826DA298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA2A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA2A8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA2AC: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA2B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA2B4: 390B51D8  addi r8, r11, 0x51d8
	ctx.r[8].s64 = ctx.r[11].s64 + 20952;
	// 826DA2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA2BC: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 826DA2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA2C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA2D0: 386AE324  addi r3, r10, -0x1cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7388;
	// 826DA2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA2F4: 4BD8CB2D  bl 0x82466e20
	ctx.lr = 0x826DA2F8;
	sub_82466E20(ctx, base);
	// 826DA2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA308 size=112
    let mut pc: u32 = 0x826DA308;
    'dispatch: loop {
        match pc {
            0x826DA308 => {
    //   block [0x826DA308..0x826DA378)
	// 826DA308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA318: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA31C: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA324: 390B5220  addi r8, r11, 0x5220
	ctx.r[8].s64 = ctx.r[11].s64 + 21024;
	// 826DA328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA32C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 826DA330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA340: 386AE354  addi r3, r10, -0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + -7340;
	// 826DA344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA364: 4BD8CABD  bl 0x82466e20
	ctx.lr = 0x826DA368;
	sub_82466E20(ctx, base);
	// 826DA368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA378 size=108
    let mut pc: u32 = 0x826DA378;
    'dispatch: loop {
        match pc {
            0x826DA378 => {
    //   block [0x826DA378..0x826DA3E4)
	// 826DA378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA384: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA388: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA38C: 38EB5268  addi r7, r11, 0x5268
	ctx.r[7].s64 = ctx.r[11].s64 + 21096;
	// 826DA390: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DA394: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 826DA398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA3A8: 386AE384  addi r3, r10, -0x1c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7292;
	// 826DA3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA3D0: 4BD8CA51  bl 0x82466e20
	ctx.lr = 0x826DA3D4;
	sub_82466E20(ctx, base);
	// 826DA3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA3E8 size=112
    let mut pc: u32 = 0x826DA3E8;
    'dispatch: loop {
        match pc {
            0x826DA3E8 => {
    //   block [0x826DA3E8..0x826DA458)
	// 826DA3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA3F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA3F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA3FC: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA404: 390B52B0  addi r8, r11, 0x52b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21168;
	// 826DA408: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DA40C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 826DA410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA420: 386AE3B4  addi r3, r10, -0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7244;
	// 826DA424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA444: 4BD8C9DD  bl 0x82466e20
	ctx.lr = 0x826DA448;
	sub_82466E20(ctx, base);
	// 826DA448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA458 size=116
    let mut pc: u32 = 0x826DA458;
    'dispatch: loop {
        match pc {
            0x826DA458 => {
    //   block [0x826DA458..0x826DA4CC)
	// 826DA458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA464: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA468: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA46C: 392B5450  addi r9, r11, 0x5450
	ctx.r[9].s64 = ctx.r[11].s64 + 21584;
	// 826DA470: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA474: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA478: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826DA47C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826DA480: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA484: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 826DA488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA48C: 396B5330  addi r11, r11, 0x5330
	ctx.r[11].s64 = ctx.r[11].s64 + 21296;
	// 826DA490: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DA494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA498: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DA49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA4A0: 386AE3E4  addi r3, r10, -0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7196;
	// 826DA4A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA4A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DA4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA4B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DA4B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA4B8: 4BD8C969  bl 0x82466e20
	ctx.lr = 0x826DA4BC;
	sub_82466E20(ctx, base);
	// 826DA4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA4D0 size=108
    let mut pc: u32 = 0x826DA4D0;
    'dispatch: loop {
        match pc {
            0x826DA4D0 => {
    //   block [0x826DA4D0..0x826DA53C)
	// 826DA4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA4DC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA4E4: 38EB53C0  addi r7, r11, 0x53c0
	ctx.r[7].s64 = ctx.r[11].s64 + 21440;
	// 826DA4E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DA4EC: 388AB870  addi r4, r10, -0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + -18320;
	// 826DA4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA4F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA500: 386AE414  addi r3, r10, -0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + -7148;
	// 826DA504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA528: 4BD8C8F9  bl 0x82466e20
	ctx.lr = 0x826DA52C;
	sub_82466E20(ctx, base);
	// 826DA52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA540 size=108
    let mut pc: u32 = 0x826DA540;
    'dispatch: loop {
        match pc {
            0x826DA540 => {
    //   block [0x826DA540..0x826DA5AC)
	// 826DA540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA54C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA550: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA554: 38EB5408  addi r7, r11, 0x5408
	ctx.r[7].s64 = ctx.r[11].s64 + 21512;
	// 826DA558: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DA55C: 388AB898  addi r4, r10, -0x4768
	ctx.r[4].s64 = ctx.r[10].s64 + -18280;
	// 826DA560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA570: 386AE444  addi r3, r10, -0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7100;
	// 826DA574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA598: 4BD8C889  bl 0x82466e20
	ctx.lr = 0x826DA59C;
	sub_82466E20(ctx, base);
	// 826DA59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA5B0 size=112
    let mut pc: u32 = 0x826DA5B0;
    'dispatch: loop {
        match pc {
            0x826DA5B0 => {
    //   block [0x826DA5B0..0x826DA620)
	// 826DA5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA5BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA5C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA5C4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA5CC: 390B5468  addi r8, r11, 0x5468
	ctx.r[8].s64 = ctx.r[11].s64 + 21608;
	// 826DA5D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DA5D4: 388AB8C0  addi r4, r10, -0x4740
	ctx.r[4].s64 = ctx.r[10].s64 + -18240;
	// 826DA5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA5E8: 386AE474  addi r3, r10, -0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7052;
	// 826DA5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA60C: 4BD8C815  bl 0x82466e20
	ctx.lr = 0x826DA610;
	sub_82466E20(ctx, base);
	// 826DA610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA620 size=112
    let mut pc: u32 = 0x826DA620;
    'dispatch: loop {
        match pc {
            0x826DA620 => {
    //   block [0x826DA620..0x826DA690)
	// 826DA620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA62C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA630: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA634: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA63C: 390B54C8  addi r8, r11, 0x54c8
	ctx.r[8].s64 = ctx.r[11].s64 + 21704;
	// 826DA640: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DA644: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 826DA648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA64C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA658: 386AE4A4  addi r3, r10, -0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7004;
	// 826DA65C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA67C: 4BD8C7A5  bl 0x82466e20
	ctx.lr = 0x826DA680;
	sub_82466E20(ctx, base);
	// 826DA680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DA690 size=24
    let mut pc: u32 = 0x826DA690;
    'dispatch: loop {
        match pc {
            0x826DA690 => {
    //   block [0x826DA690..0x826DA6A8)
	// 826DA690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA694: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA698: 394AB660  addi r10, r10, -0x49a0
	ctx.r[10].s64 = ctx.r[10].s64 + -18848;
	// 826DA69C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826DA6A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826DA6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA6A8 size=116
    let mut pc: u32 = 0x826DA6A8;
    'dispatch: loop {
        match pc {
            0x826DA6A8 => {
    //   block [0x826DA6A8..0x826DA71C)
	// 826DA6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA6B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA6B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DA6BC: 390AB660  addi r8, r10, -0x49a0
	ctx.r[8].s64 = ctx.r[10].s64 + -18848;
	// 826DA6C0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA6C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA6C8: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA6CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA6D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA6DC: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 826DA6E0: 396B5480  addi r11, r11, 0x5480
	ctx.r[11].s64 = ctx.r[11].s64 + 21632;
	// 826DA6E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA6E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA6EC: 386AE4D4  addi r3, r10, -0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6956;
	// 826DA6F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DA6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA6F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA708: 4BD8C719  bl 0x82466e20
	ctx.lr = 0x826DA70C;
	sub_82466E20(ctx, base);
	// 826DA70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA720 size=100
    let mut pc: u32 = 0x826DA720;
    'dispatch: loop {
        match pc {
            0x826DA720 => {
    //   block [0x826DA720..0x826DA784)
	// 826DA720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA72C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA734: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DA738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA740: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 826DA744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA754: 386AE504  addi r3, r10, -0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + -6908;
	// 826DA758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DA764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA770: 4BD8C6B1  bl 0x82466e20
	ctx.lr = 0x826DA774;
	sub_82466E20(ctx, base);
	// 826DA774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA788 size=100
    let mut pc: u32 = 0x826DA788;
    'dispatch: loop {
        match pc {
            0x826DA788 => {
    //   block [0x826DA788..0x826DA7EC)
	// 826DA788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA79C: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DA7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA7A8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 826DA7AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA7BC: 386AE534  addi r3, r10, -0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + -6860;
	// 826DA7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA7C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DA7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA7D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA7D8: 4BD8C649  bl 0x82466e20
	ctx.lr = 0x826DA7DC;
	sub_82466E20(ctx, base);
	// 826DA7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA7F0 size=112
    let mut pc: u32 = 0x826DA7F0;
    'dispatch: loop {
        match pc {
            0x826DA7F0 => {
    //   block [0x826DA7F0..0x826DA860)
	// 826DA7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA800: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA804: 38AAE504  addi r5, r10, -0x1afc
	ctx.r[5].s64 = ctx.r[10].s64 + -6908;
	// 826DA808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA80C: 390B54E0  addi r8, r11, 0x54e0
	ctx.r[8].s64 = ctx.r[11].s64 + 21728;
	// 826DA810: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA814: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 826DA818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA81C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA828: 386AE564  addi r3, r10, -0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -6812;
	// 826DA82C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA84C: 4BD8C5D5  bl 0x82466e20
	ctx.lr = 0x826DA850;
	sub_82466E20(ctx, base);
	// 826DA850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA860 size=112
    let mut pc: u32 = 0x826DA860;
    'dispatch: loop {
        match pc {
            0x826DA860 => {
    //   block [0x826DA860..0x826DA8D0)
	// 826DA860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA870: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA874: 38AAE504  addi r5, r10, -0x1afc
	ctx.r[5].s64 = ctx.r[10].s64 + -6908;
	// 826DA878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA87C: 390B5528  addi r8, r11, 0x5528
	ctx.r[8].s64 = ctx.r[11].s64 + 21800;
	// 826DA880: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DA884: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 826DA888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA88C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA898: 386AE594  addi r3, r10, -0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -6764;
	// 826DA89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA8BC: 4BD8C565  bl 0x82466e20
	ctx.lr = 0x826DA8C0;
	sub_82466E20(ctx, base);
	// 826DA8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA8D0 size=112
    let mut pc: u32 = 0x826DA8D0;
    'dispatch: loop {
        match pc {
            0x826DA8D0 => {
    //   block [0x826DA8D0..0x826DA940)
	// 826DA8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA8E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA8E4: 38AAE594  addi r5, r10, -0x1a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -6764;
	// 826DA8E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA8EC: 390B55D0  addi r8, r11, 0x55d0
	ctx.r[8].s64 = ctx.r[11].s64 + 21968;
	// 826DA8F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA8F4: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 826DA8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA8FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA908: 386AE5C4  addi r3, r10, -0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -6716;
	// 826DA90C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA92C: 4BD8C4F5  bl 0x82466e20
	ctx.lr = 0x826DA930;
	sub_82466E20(ctx, base);
	// 826DA930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA940 size=112
    let mut pc: u32 = 0x826DA940;
    'dispatch: loop {
        match pc {
            0x826DA940 => {
    //   block [0x826DA940..0x826DA9B0)
	// 826DA940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA950: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA954: 38AAE1A4  addi r5, r10, -0x1e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7772;
	// 826DA958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA95C: 390B5600  addi r8, r11, 0x5600
	ctx.r[8].s64 = ctx.r[11].s64 + 22016;
	// 826DA960: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA964: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 826DA968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA96C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA978: 386AE5F4  addi r3, r10, -0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -6668;
	// 826DA97C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA99C: 4BD8C485  bl 0x82466e20
	ctx.lr = 0x826DA9A0;
	sub_82466E20(ctx, base);
	// 826DA9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA9B0 size=112
    let mut pc: u32 = 0x826DA9B0;
    'dispatch: loop {
        match pc {
            0x826DA9B0 => {
    //   block [0x826DA9B0..0x826DAA20)
	// 826DA9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA9C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA9C4: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826DA9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA9CC: 390B5630  addi r8, r11, 0x5630
	ctx.r[8].s64 = ctx.r[11].s64 + 22064;
	// 826DA9D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA9D4: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 826DA9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA9DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA9E8: 386AE624  addi r3, r10, -0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6620;
	// 826DA9EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAA0C: 4BD8C415  bl 0x82466e20
	ctx.lr = 0x826DAA10;
	sub_82466E20(ctx, base);
	// 826DAA10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAA20 size=112
    let mut pc: u32 = 0x826DAA20;
    'dispatch: loop {
        match pc {
            0x826DAA20 => {
    //   block [0x826DAA20..0x826DAA90)
	// 826DAA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAA30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAA34: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAA38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAA3C: 390B5660  addi r8, r11, 0x5660
	ctx.r[8].s64 = ctx.r[11].s64 + 22112;
	// 826DAA40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAA44: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 826DAA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAA58: 386AE654  addi r3, r10, -0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6572;
	// 826DAA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAA7C: 4BD8C3A5  bl 0x82466e20
	ctx.lr = 0x826DAA80;
	sub_82466E20(ctx, base);
	// 826DAA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAA90 size=112
    let mut pc: u32 = 0x826DAA90;
    'dispatch: loop {
        match pc {
            0x826DAA90 => {
    //   block [0x826DAA90..0x826DAB00)
	// 826DAA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAA9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAAA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAAA4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DAAA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAAAC: 390B5690  addi r8, r11, 0x5690
	ctx.r[8].s64 = ctx.r[11].s64 + 22160;
	// 826DAAB0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DAAB4: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 826DAAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAAC8: 386AE684  addi r3, r10, -0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + -6524;
	// 826DAACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAADC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAAEC: 4BD8C335  bl 0x82466e20
	ctx.lr = 0x826DAAF0;
	sub_82466E20(ctx, base);
	// 826DAAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAB00 size=108
    let mut pc: u32 = 0x826DAB00;
    'dispatch: loop {
        match pc {
            0x826DAB00 => {
    //   block [0x826DAB00..0x826DAB6C)
	// 826DAB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAB0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAB10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAB14: 38EB5708  addi r7, r11, 0x5708
	ctx.r[7].s64 = ctx.r[11].s64 + 22280;
	// 826DAB18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DAB1C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 826DAB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAB24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DAB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAB30: 386AE6B4  addi r3, r10, -0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + -6476;
	// 826DAB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DAB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAB58: 4BD8C2C9  bl 0x82466e20
	ctx.lr = 0x826DAB5C;
	sub_82466E20(ctx, base);
	// 826DAB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAB70 size=112
    let mut pc: u32 = 0x826DAB70;
    'dispatch: loop {
        match pc {
            0x826DAB70 => {
    //   block [0x826DAB70..0x826DABE0)
	// 826DAB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAB7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAB80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAB84: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAB88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAB8C: 390B5738  addi r8, r11, 0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + 22328;
	// 826DAB90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAB94: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 826DAB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAB9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DABA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DABA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DABA8: 386AE6E4  addi r3, r10, -0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + -6428;
	// 826DABAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DABB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DABB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DABB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DABBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DABC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DABC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DABC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DABCC: 4BD8C255  bl 0x82466e20
	ctx.lr = 0x826DABD0;
	sub_82466E20(ctx, base);
	// 826DABD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DABD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DABD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DABDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DABE0 size=112
    let mut pc: u32 = 0x826DABE0;
    'dispatch: loop {
        match pc {
            0x826DABE0 => {
    //   block [0x826DABE0..0x826DAC50)
	// 826DABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DABE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DABE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DABEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DABF0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DABF4: 38AAE534  addi r5, r10, -0x1acc
	ctx.r[5].s64 = ctx.r[10].s64 + -6860;
	// 826DABF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DABFC: 390B5768  addi r8, r11, 0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + 22376;
	// 826DAC00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAC04: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 826DAC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAC0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAC18: 386AE714  addi r3, r10, -0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6380;
	// 826DAC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAC3C: 4BD8C1E5  bl 0x82466e20
	ctx.lr = 0x826DAC40;
	sub_82466E20(ctx, base);
	// 826DAC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAC50 size=100
    let mut pc: u32 = 0x826DAC50;
    'dispatch: loop {
        match pc {
            0x826DAC50 => {
    //   block [0x826DAC50..0x826DACB4)
	// 826DAC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAC5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAC64: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAC68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAC70: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 826DAC74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAC84: 386AE744  addi r3, r10, -0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6332;
	// 826DAC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DACA0: 4BD8C181  bl 0x82466e20
	ctx.lr = 0x826DACA4;
	sub_82466E20(ctx, base);
	// 826DACA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DACA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DACAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DACB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DACB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DACB8 size=112
    let mut pc: u32 = 0x826DACB8;
    'dispatch: loop {
        match pc {
            0x826DACB8 => {
    //   block [0x826DACB8..0x826DAD28)
	// 826DACB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DACBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DACC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DACC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DACC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DACCC: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DACD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DACD4: 390B5798  addi r8, r11, 0x5798
	ctx.r[8].s64 = ctx.r[11].s64 + 22424;
	// 826DACD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DACDC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 826DACE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DACE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DACE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DACEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DACF0: 386AE774  addi r3, r10, -0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + -6284;
	// 826DACF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DACF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DACFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAD14: 4BD8C10D  bl 0x82466e20
	ctx.lr = 0x826DAD18;
	sub_82466E20(ctx, base);
	// 826DAD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAD28 size=96
    let mut pc: u32 = 0x826DAD28;
    'dispatch: loop {
        match pc {
            0x826DAD28 => {
    //   block [0x826DAD28..0x826DAD88)
	// 826DAD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAD34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAD3C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 826DAD40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAD48: 386AE7A4  addi r3, r10, -0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + -6236;
	// 826DAD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAD54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAD74: 4BD8C0AD  bl 0x82466e20
	ctx.lr = 0x826DAD78;
	sub_82466E20(ctx, base);
	// 826DAD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAD88 size=108
    let mut pc: u32 = 0x826DAD88;
    'dispatch: loop {
        match pc {
            0x826DAD88 => {
    //   block [0x826DAD88..0x826DADF4)
	// 826DAD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAD94: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAD98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAD9C: 38EB57E0  addi r7, r11, 0x57e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22496;
	// 826DADA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DADA4: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 826DADA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DADAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DADB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DADB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DADB8: 386AE7D4  addi r3, r10, -0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + -6188;
	// 826DADBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DADC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DADC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DADC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DADCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DADD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DADD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DADD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DADDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DADE0: 4BD8C041  bl 0x82466e20
	ctx.lr = 0x826DADE4;
	sub_82466E20(ctx, base);
	// 826DADE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DADE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DADEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DADF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DADF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DADF8 size=100
    let mut pc: u32 = 0x826DADF8;
    'dispatch: loop {
        match pc {
            0x826DADF8 => {
    //   block [0x826DADF8..0x826DAE5C)
	// 826DADF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DADFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAE04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DAE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAE0C: 392A54F0  addi r9, r10, 0x54f0
	ctx.r[9].s64 = ctx.r[10].s64 + 21744;
	// 826DAE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAE18: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 826DAE1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAE2C: 386AE804  addi r3, r10, -0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6140;
	// 826DAE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAE34: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826DAE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAE40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAE48: 4BD8BFD9  bl 0x82466e20
	ctx.lr = 0x826DAE4C;
	sub_82466E20(ctx, base);
	// 826DAE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DAE60 size=24
    let mut pc: u32 = 0x826DAE60;
    'dispatch: loop {
        match pc {
            0x826DAE60 => {
    //   block [0x826DAE60..0x826DAE78)
	// 826DAE60: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAE64: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DAE68: 394AB708  addi r10, r10, -0x48f8
	ctx.r[10].s64 = ctx.r[10].s64 + -18680;
	// 826DAE6C: 816B5848  lwz r11, 0x5848(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22600 as u32) ) } as u64;
	// 826DAE70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DAE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAE78 size=112
    let mut pc: u32 = 0x826DAE78;
    'dispatch: loop {
        match pc {
            0x826DAE78 => {
    //   block [0x826DAE78..0x826DAEE8)
	// 826DAE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAE84: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DAE88: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DAE8C: 392A5630  addi r9, r10, 0x5630
	ctx.r[9].s64 = ctx.r[10].s64 + 22064;
	// 826DAE90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAE94: 390BB708  addi r8, r11, -0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + -18680;
	// 826DAE98: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DAE9C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 826DAEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAEA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAEB0: 386AE834  addi r3, r10, -0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6092;
	// 826DAEB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DAEB8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826DAEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAED4: 4BD8BF4D  bl 0x82466e20
	ctx.lr = 0x826DAED8;
	sub_82466E20(ctx, base);
	// 826DAED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAEE8 size=112
    let mut pc: u32 = 0x826DAEE8;
    'dispatch: loop {
        match pc {
            0x826DAEE8 => {
    //   block [0x826DAEE8..0x826DAF58)
	// 826DAEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAEF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAEF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAEFC: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DAF00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAF04: 390B5850  addi r8, r11, 0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + 22608;
	// 826DAF08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAF0C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 826DAF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAF20: 386AE864  addi r3, r10, -0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + -6044;
	// 826DAF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAF44: 4BD8BEDD  bl 0x82466e20
	ctx.lr = 0x826DAF48;
	sub_82466E20(ctx, base);
	// 826DAF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAF58 size=108
    let mut pc: u32 = 0x826DAF58;
    'dispatch: loop {
        match pc {
            0x826DAF58 => {
    //   block [0x826DAF58..0x826DAFC4)
	// 826DAF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAF64: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAF68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAF6C: 38EB5880  addi r7, r11, 0x5880
	ctx.r[7].s64 = ctx.r[11].s64 + 22656;
	// 826DAF70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAF74: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 826DAF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAF7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DAF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAF88: 386AE894  addi r3, r10, -0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + -5996;
	// 826DAF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DAF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAFB0: 4BD8BE71  bl 0x82466e20
	ctx.lr = 0x826DAFB4;
	sub_82466E20(ctx, base);
	// 826DAFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAFC8 size=112
    let mut pc: u32 = 0x826DAFC8;
    'dispatch: loop {
        match pc {
            0x826DAFC8 => {
    //   block [0x826DAFC8..0x826DB038)
	// 826DAFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAFD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAFD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAFDC: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DAFE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAFE4: 390B5898  addi r8, r11, 0x5898
	ctx.r[8].s64 = ctx.r[11].s64 + 22680;
	// 826DAFE8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DAFEC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 826DAFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAFF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB000: 386AE8C4  addi r3, r10, -0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + -5948;
	// 826DB004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB024: 4BD8BDFD  bl 0x82466e20
	ctx.lr = 0x826DB028;
	sub_82466E20(ctx, base);
	// 826DB028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB038 size=100
    let mut pc: u32 = 0x826DB038;
    'dispatch: loop {
        match pc {
            0x826DB038 => {
    //   block [0x826DB038..0x826DB09C)
	// 826DB038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB044: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB04C: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB058: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 826DB05C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB06C: 386AE8F4  addi r3, r10, -0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + -5900;
	// 826DB070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB078: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DB07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB080: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DB084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB088: 4BD8BD99  bl 0x82466e20
	ctx.lr = 0x826DB08C;
	sub_82466E20(ctx, base);
	// 826DB08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB0A0 size=112
    let mut pc: u32 = 0x826DB0A0;
    'dispatch: loop {
        match pc {
            0x826DB0A0 => {
    //   block [0x826DB0A0..0x826DB110)
	// 826DB0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB0AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB0B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB0B4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB0BC: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 826DB0C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB0C4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 826DB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB0CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB0D8: 386AE924  addi r3, r10, -0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5852;
	// 826DB0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB0FC: 4BD8BD25  bl 0x82466e20
	ctx.lr = 0x826DB100;
	sub_82466E20(ctx, base);
	// 826DB100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB110 size=112
    let mut pc: u32 = 0x826DB110;
    'dispatch: loop {
        match pc {
            0x826DB110 => {
    //   block [0x826DB110..0x826DB180)
	// 826DB110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB120: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB124: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB12C: 390B5958  addi r8, r11, 0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + 22872;
	// 826DB130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB134: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 826DB138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB13C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB148: 386AE954  addi r3, r10, -0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5804;
	// 826DB14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB16C: 4BD8BCB5  bl 0x82466e20
	ctx.lr = 0x826DB170;
	sub_82466E20(ctx, base);
	// 826DB170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB180 size=112
    let mut pc: u32 = 0x826DB180;
    'dispatch: loop {
        match pc {
            0x826DB180 => {
    //   block [0x826DB180..0x826DB1F0)
	// 826DB180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB194: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB19C: 390B5988  addi r8, r11, 0x5988
	ctx.r[8].s64 = ctx.r[11].s64 + 22920;
	// 826DB1A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB1A4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826DB1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB1AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB1B8: 386AE984  addi r3, r10, -0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + -5756;
	// 826DB1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB1DC: 4BD8BC45  bl 0x82466e20
	ctx.lr = 0x826DB1E0;
	sub_82466E20(ctx, base);
	// 826DB1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB1F0 size=112
    let mut pc: u32 = 0x826DB1F0;
    'dispatch: loop {
        match pc {
            0x826DB1F0 => {
    //   block [0x826DB1F0..0x826DB260)
	// 826DB1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB1FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB204: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB20C: 390B59B8  addi r8, r11, 0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + 22968;
	// 826DB210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB214: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 826DB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB21C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB228: 386AE9B4  addi r3, r10, -0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + -5708;
	// 826DB22C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB24C: 4BD8BBD5  bl 0x82466e20
	ctx.lr = 0x826DB250;
	sub_82466E20(ctx, base);
	// 826DB250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB260 size=112
    let mut pc: u32 = 0x826DB260;
    'dispatch: loop {
        match pc {
            0x826DB260 => {
    //   block [0x826DB260..0x826DB2D0)
	// 826DB260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB26C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB270: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB274: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB27C: 390B59E8  addi r8, r11, 0x59e8
	ctx.r[8].s64 = ctx.r[11].s64 + 23016;
	// 826DB280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB284: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826DB288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB28C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB298: 386AE9E4  addi r3, r10, -0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + -5660;
	// 826DB29C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB2BC: 4BD8BB65  bl 0x82466e20
	ctx.lr = 0x826DB2C0;
	sub_82466E20(ctx, base);
	// 826DB2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB2D0 size=112
    let mut pc: u32 = 0x826DB2D0;
    'dispatch: loop {
        match pc {
            0x826DB2D0 => {
    //   block [0x826DB2D0..0x826DB340)
	// 826DB2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB2E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB2E4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB2E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB2EC: 390B5A00  addi r8, r11, 0x5a00
	ctx.r[8].s64 = ctx.r[11].s64 + 23040;
	// 826DB2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB2F4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 826DB2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB2FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB308: 386AEA14  addi r3, r10, -0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5612;
	// 826DB30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB32C: 4BD8BAF5  bl 0x82466e20
	ctx.lr = 0x826DB330;
	sub_82466E20(ctx, base);
	// 826DB330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB340 size=112
    let mut pc: u32 = 0x826DB340;
    'dispatch: loop {
        match pc {
            0x826DB340 => {
    //   block [0x826DB340..0x826DB3B0)
	// 826DB340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB34C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB350: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB354: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB358: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB35C: 390B5A18  addi r8, r11, 0x5a18
	ctx.r[8].s64 = ctx.r[11].s64 + 23064;
	// 826DB360: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB364: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 826DB368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB36C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB378: 386AEA44  addi r3, r10, -0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5564;
	// 826DB37C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB39C: 4BD8BA85  bl 0x82466e20
	ctx.lr = 0x826DB3A0;
	sub_82466E20(ctx, base);
	// 826DB3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB3B0 size=112
    let mut pc: u32 = 0x826DB3B0;
    'dispatch: loop {
        match pc {
            0x826DB3B0 => {
    //   block [0x826DB3B0..0x826DB420)
	// 826DB3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB3BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB3C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB3C4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB3C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB3CC: 390B5A60  addi r8, r11, 0x5a60
	ctx.r[8].s64 = ctx.r[11].s64 + 23136;
	// 826DB3D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB3D4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 826DB3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB3DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB3E8: 386AEA74  addi r3, r10, -0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + -5516;
	// 826DB3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB40C: 4BD8BA15  bl 0x82466e20
	ctx.lr = 0x826DB410;
	sub_82466E20(ctx, base);
	// 826DB410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB420 size=112
    let mut pc: u32 = 0x826DB420;
    'dispatch: loop {
        match pc {
            0x826DB420 => {
    //   block [0x826DB420..0x826DB490)
	// 826DB420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB42C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB430: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB434: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB43C: 390B5AA8  addi r8, r11, 0x5aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 23208;
	// 826DB440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB444: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 826DB448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB44C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB458: 386AEAA4  addi r3, r10, -0x155c
	ctx.r[3].s64 = ctx.r[10].s64 + -5468;
	// 826DB45C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB47C: 4BD8B9A5  bl 0x82466e20
	ctx.lr = 0x826DB480;
	sub_82466E20(ctx, base);
	// 826DB480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB490 size=112
    let mut pc: u32 = 0x826DB490;
    'dispatch: loop {
        match pc {
            0x826DB490 => {
    //   block [0x826DB490..0x826DB500)
	// 826DB490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB49C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB4A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB4A4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB4A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB4AC: 390B5AC0  addi r8, r11, 0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 23232;
	// 826DB4B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB4B4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826DB4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB4BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB4C8: 386AEAD4  addi r3, r10, -0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + -5420;
	// 826DB4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB4EC: 4BD8B935  bl 0x82466e20
	ctx.lr = 0x826DB4F0;
	sub_82466E20(ctx, base);
	// 826DB4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB500 size=116
    let mut pc: u32 = 0x826DB500;
    'dispatch: loop {
        match pc {
            0x826DB500 => {
    //   block [0x826DB500..0x826DB574)
	// 826DB500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB50C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DB510: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DB514: 390A5AF0  addi r8, r10, 0x5af0
	ctx.r[8].s64 = ctx.r[10].s64 + 23280;
	// 826DB518: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB51C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB520: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB524: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB528: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DB52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB534: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 826DB538: 396B5658  addi r11, r11, 0x5658
	ctx.r[11].s64 = ctx.r[11].s64 + 22104;
	// 826DB53C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB544: 386AEB04  addi r3, r10, -0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5372;
	// 826DB548: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DB54C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB550: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DB554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB560: 4BD8B8C1  bl 0x82466e20
	ctx.lr = 0x826DB564;
	sub_82466E20(ctx, base);
	// 826DB564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB578 size=116
    let mut pc: u32 = 0x826DB578;
    'dispatch: loop {
        match pc {
            0x826DB578 => {
    //   block [0x826DB578..0x826DB5EC)
	// 826DB578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB584: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DB588: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826DB58C: 390A5B68  addi r8, r10, 0x5b68
	ctx.r[8].s64 = ctx.r[10].s64 + 23400;
	// 826DB590: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB594: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB598: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB59C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB5A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DB5A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB5AC: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826DB5B0: 396B5670  addi r11, r11, 0x5670
	ctx.r[11].s64 = ctx.r[11].s64 + 22128;
	// 826DB5B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB5B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB5BC: 386AEB34  addi r3, r10, -0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5324;
	// 826DB5C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DB5C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB5C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DB5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB5D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB5D8: 4BD8B849  bl 0x82466e20
	ctx.lr = 0x826DB5DC;
	sub_82466E20(ctx, base);
	// 826DB5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DB5F0 size=24
    let mut pc: u32 = 0x826DB5F0;
    'dispatch: loop {
        match pc {
            0x826DB5F0 => {
    //   block [0x826DB5F0..0x826DB608)
	// 826DB5F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB5F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DB5F8: 394AB720  addi r10, r10, -0x48e0
	ctx.r[10].s64 = ctx.r[10].s64 + -18656;
	// 826DB5FC: 816B5BF8  lwz r11, 0x5bf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23544 as u32) ) } as u64;
	// 826DB600: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826DB604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB608 size=116
    let mut pc: u32 = 0x826DB608;
    'dispatch: loop {
        match pc {
            0x826DB608 => {
    //   block [0x826DB608..0x826DB67C)
	// 826DB608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB614: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB618: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB61C: 392B569C  addi r9, r11, 0x569c
	ctx.r[9].s64 = ctx.r[11].s64 + 22172;
	// 826DB620: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB624: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB628: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826DB62C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826DB630: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DB634: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 826DB638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB63C: 396BB720  addi r11, r11, -0x48e0
	ctx.r[11].s64 = ctx.r[11].s64 + -18656;
	// 826DB640: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DB644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB648: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DB64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB650: 386AEB64  addi r3, r10, -0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + -5276;
	// 826DB654: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DB658: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DB65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB660: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DB664: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DB668: 4BD8B7B9  bl 0x82466e20
	ctx.lr = 0x826DB66C;
	sub_82466E20(ctx, base);
	// 826DB66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB680 size=112
    let mut pc: u32 = 0x826DB680;
    'dispatch: loop {
        match pc {
            0x826DB680 => {
    //   block [0x826DB680..0x826DB6F0)
	// 826DB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB68C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB694: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB69C: 390B5C00  addi r8, r11, 0x5c00
	ctx.r[8].s64 = ctx.r[11].s64 + 23552;
	// 826DB6A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB6A4: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826DB6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB6AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB6B8: 386AEB94  addi r3, r10, -0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + -5228;
	// 826DB6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB6DC: 4BD8B745  bl 0x82466e20
	ctx.lr = 0x826DB6E0;
	sub_82466E20(ctx, base);
	// 826DB6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB6F0 size=112
    let mut pc: u32 = 0x826DB6F0;
    'dispatch: loop {
        match pc {
            0x826DB6F0 => {
    //   block [0x826DB6F0..0x826DB760)
	// 826DB6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB6FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB700: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB704: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB70C: 390B5C60  addi r8, r11, 0x5c60
	ctx.r[8].s64 = ctx.r[11].s64 + 23648;
	// 826DB710: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DB714: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 826DB718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB71C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB728: 386AEBC4  addi r3, r10, -0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + -5180;
	// 826DB72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB74C: 4BD8B6D5  bl 0x82466e20
	ctx.lr = 0x826DB750;
	sub_82466E20(ctx, base);
	// 826DB750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB760 size=112
    let mut pc: u32 = 0x826DB760;
    'dispatch: loop {
        match pc {
            0x826DB760 => {
    //   block [0x826DB760..0x826DB7D0)
	// 826DB760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB76C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB770: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB774: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB77C: 390B5D08  addi r8, r11, 0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + 23816;
	// 826DB780: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DB784: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826DB788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB78C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB798: 386AEBF4  addi r3, r10, -0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + -5132;
	// 826DB79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB7BC: 4BD8B665  bl 0x82466e20
	ctx.lr = 0x826DB7C0;
	sub_82466E20(ctx, base);
	// 826DB7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB7D0 size=112
    let mut pc: u32 = 0x826DB7D0;
    'dispatch: loop {
        match pc {
            0x826DB7D0 => {
    //   block [0x826DB7D0..0x826DB840)
	// 826DB7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB7DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB7E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB7E4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB7EC: 390B5D80  addi r8, r11, 0x5d80
	ctx.r[8].s64 = ctx.r[11].s64 + 23936;
	// 826DB7F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB7F4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 826DB7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB808: 386AEC24  addi r3, r10, -0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5084;
	// 826DB80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB82C: 4BD8B5F5  bl 0x82466e20
	ctx.lr = 0x826DB830;
	sub_82466E20(ctx, base);
	// 826DB830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB840 size=112
    let mut pc: u32 = 0x826DB840;
    'dispatch: loop {
        match pc {
            0x826DB840 => {
    //   block [0x826DB840..0x826DB8B0)
	// 826DB840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB84C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB850: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB854: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB85C: 390B5DC8  addi r8, r11, 0x5dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 24008;
	// 826DB860: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826DB864: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 826DB868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB878: 386AEC54  addi r3, r10, -0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5036;
	// 826DB87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB89C: 4BD8B585  bl 0x82466e20
	ctx.lr = 0x826DB8A0;
	sub_82466E20(ctx, base);
	// 826DB8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB8B0 size=112
    let mut pc: u32 = 0x826DB8B0;
    'dispatch: loop {
        match pc {
            0x826DB8B0 => {
    //   block [0x826DB8B0..0x826DB920)
	// 826DB8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB8BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB8C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB8C4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB8CC: 390B5E58  addi r8, r11, 0x5e58
	ctx.r[8].s64 = ctx.r[11].s64 + 24152;
	// 826DB8D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB8D4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 826DB8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB8E8: 386AEC84  addi r3, r10, -0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + -4988;
	// 826DB8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB90C: 4BD8B515  bl 0x82466e20
	ctx.lr = 0x826DB910;
	sub_82466E20(ctx, base);
	// 826DB910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB920 size=112
    let mut pc: u32 = 0x826DB920;
    'dispatch: loop {
        match pc {
            0x826DB920 => {
    //   block [0x826DB920..0x826DB990)
	// 826DB920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB92C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB930: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB934: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB93C: 390B5EB8  addi r8, r11, 0x5eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24248;
	// 826DB940: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB944: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 826DB948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB958: 386AECB4  addi r3, r10, -0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + -4940;
	// 826DB95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB97C: 4BD8B4A5  bl 0x82466e20
	ctx.lr = 0x826DB980;
	sub_82466E20(ctx, base);
	// 826DB980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB990 size=112
    let mut pc: u32 = 0x826DB990;
    'dispatch: loop {
        match pc {
            0x826DB990 => {
    //   block [0x826DB990..0x826DBA00)
	// 826DB990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB99C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB9A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB9A4: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DB9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB9AC: 390B5F18  addi r8, r11, 0x5f18
	ctx.r[8].s64 = ctx.r[11].s64 + 24344;
	// 826DB9B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB9B4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826DB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB9C8: 386AECE4  addi r3, r10, -0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + -4892;
	// 826DB9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB9EC: 4BD8B435  bl 0x82466e20
	ctx.lr = 0x826DB9F0;
	sub_82466E20(ctx, base);
	// 826DB9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBA00 size=112
    let mut pc: u32 = 0x826DBA00;
    'dispatch: loop {
        match pc {
            0x826DBA00 => {
    //   block [0x826DBA00..0x826DBA70)
	// 826DBA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBA0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBA14: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBA18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBA1C: 390B5F48  addi r8, r11, 0x5f48
	ctx.r[8].s64 = ctx.r[11].s64 + 24392;
	// 826DBA20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DBA24: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 826DBA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBA38: 386AED14  addi r3, r10, -0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4844;
	// 826DBA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBA5C: 4BD8B3C5  bl 0x82466e20
	ctx.lr = 0x826DBA60;
	sub_82466E20(ctx, base);
	// 826DBA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBA70 size=100
    let mut pc: u32 = 0x826DBA70;
    'dispatch: loop {
        match pc {
            0x826DBA70 => {
    //   block [0x826DBA70..0x826DBAD4)
	// 826DBA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBA7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBA84: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBA88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBA90: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826DBA94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBAA4: 386AED44  addi r3, r10, -0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + -4796;
	// 826DBAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBAAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBAB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DBAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBAB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DBABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBAC0: 4BD8B361  bl 0x82466e20
	ctx.lr = 0x826DBAC4;
	sub_82466E20(ctx, base);
	// 826DBAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBAD8 size=112
    let mut pc: u32 = 0x826DBAD8;
    'dispatch: loop {
        match pc {
            0x826DBAD8 => {
    //   block [0x826DBAD8..0x826DBB48)
	// 826DBAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBAE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBAE8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBAEC: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBAF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBAF4: 390B5F90  addi r8, r11, 0x5f90
	ctx.r[8].s64 = ctx.r[11].s64 + 24464;
	// 826DBAF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBAFC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 826DBB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBB04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBB10: 386AED74  addi r3, r10, -0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + -4748;
	// 826DBB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBB34: 4BD8B2ED  bl 0x82466e20
	ctx.lr = 0x826DBB38;
	sub_82466E20(ctx, base);
	// 826DBB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBB48 size=108
    let mut pc: u32 = 0x826DBB48;
    'dispatch: loop {
        match pc {
            0x826DBB48 => {
    //   block [0x826DBB48..0x826DBBB4)
	// 826DBB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBB54: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBB58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBB5C: 38EB5FA8  addi r7, r11, 0x5fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 24488;
	// 826DBB60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DBB64: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 826DBB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBB6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBB70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBB78: 386AEDA4  addi r3, r10, -0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + -4700;
	// 826DBB7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBBA0: 4BD8B281  bl 0x82466e20
	ctx.lr = 0x826DBBA4;
	sub_82466E20(ctx, base);
	// 826DBBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBBB8 size=112
    let mut pc: u32 = 0x826DBBB8;
    'dispatch: loop {
        match pc {
            0x826DBBB8 => {
    //   block [0x826DBBB8..0x826DBC28)
	// 826DBBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBBC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBBCC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBBD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBBD4: 390B5FF0  addi r8, r11, 0x5ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 24560;
	// 826DBBD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DBBDC: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 826DBBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBBE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBBE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBBF0: 386AEDD4  addi r3, r10, -0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + -4652;
	// 826DBBF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBC14: 4BD8B20D  bl 0x82466e20
	ctx.lr = 0x826DBC18;
	sub_82466E20(ctx, base);
	// 826DBC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBC28 size=112
    let mut pc: u32 = 0x826DBC28;
    'dispatch: loop {
        match pc {
            0x826DBC28 => {
    //   block [0x826DBC28..0x826DBC98)
	// 826DBC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBC34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBC38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBC3C: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBC44: 390B6050  addi r8, r11, 0x6050
	ctx.r[8].s64 = ctx.r[11].s64 + 24656;
	// 826DBC48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBC4C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 826DBC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBC60: 386AEE04  addi r3, r10, -0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4604;
	// 826DBC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBC84: 4BD8B19D  bl 0x82466e20
	ctx.lr = 0x826DBC88;
	sub_82466E20(ctx, base);
	// 826DBC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBC98 size=112
    let mut pc: u32 = 0x826DBC98;
    'dispatch: loop {
        match pc {
            0x826DBC98 => {
    //   block [0x826DBC98..0x826DBD08)
	// 826DBC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBCA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBCA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBCAC: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBCB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBCB4: 390B6068  addi r8, r11, 0x6068
	ctx.r[8].s64 = ctx.r[11].s64 + 24680;
	// 826DBCB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DBCBC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 826DBCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBCC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBCC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBCD0: 386AEE34  addi r3, r10, -0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4556;
	// 826DBCD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBCF4: 4BD8B12D  bl 0x82466e20
	ctx.lr = 0x826DBCF8;
	sub_82466E20(ctx, base);
	// 826DBCF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBD08 size=112
    let mut pc: u32 = 0x826DBD08;
    'dispatch: loop {
        match pc {
            0x826DBD08 => {
    //   block [0x826DBD08..0x826DBD78)
	// 826DBD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBD14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBD18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBD1C: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBD20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBD24: 390B6098  addi r8, r11, 0x6098
	ctx.r[8].s64 = ctx.r[11].s64 + 24728;
	// 826DBD28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBD2C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 826DBD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBD34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBD40: 386AEE64  addi r3, r10, -0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + -4508;
	// 826DBD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBD64: 4BD8B0BD  bl 0x82466e20
	ctx.lr = 0x826DBD68;
	sub_82466E20(ctx, base);
	// 826DBD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DBD78 size=24
    let mut pc: u32 = 0x826DBD78;
    'dispatch: loop {
        match pc {
            0x826DBD78 => {
    //   block [0x826DBD78..0x826DBD90)
	// 826DBD78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBD7C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DBD80: 394AB7C8  addi r10, r10, -0x4838
	ctx.r[10].s64 = ctx.r[10].s64 + -18488;
	// 826DBD84: 816B5BFC  lwz r11, 0x5bfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23548 as u32) ) } as u64;
	// 826DBD88: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DBD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBD90 size=112
    let mut pc: u32 = 0x826DBD90;
    'dispatch: loop {
        match pc {
            0x826DBD90 => {
    //   block [0x826DBD90..0x826DBE00)
	// 826DBD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBD9C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBDA0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DBDA4: 392A56F8  addi r9, r10, 0x56f8
	ctx.r[9].s64 = ctx.r[10].s64 + 22264;
	// 826DBDA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBDAC: 390BB7C8  addi r8, r11, -0x4838
	ctx.r[8].s64 = ctx.r[11].s64 + -18488;
	// 826DBDB0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DBDB4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 826DBDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBDBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBDC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBDC8: 386AEE94  addi r3, r10, -0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + -4460;
	// 826DBDCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBDD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DBDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBDEC: 4BD8B035  bl 0x82466e20
	ctx.lr = 0x826DBDF0;
	sub_82466E20(ctx, base);
	// 826DBDF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBE00 size=108
    let mut pc: u32 = 0x826DBE00;
    'dispatch: loop {
        match pc {
            0x826DBE00 => {
    //   block [0x826DBE00..0x826DBE6C)
	// 826DBE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBE0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBE14: 38EB60B0  addi r7, r11, 0x60b0
	ctx.r[7].s64 = ctx.r[11].s64 + 24752;
	// 826DBE18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DBE1C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 826DBE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBE24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBE28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBE30: 386AEEC4  addi r3, r10, -0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + -4412;
	// 826DBE34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBE54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBE58: 4BD8AFC9  bl 0x82466e20
	ctx.lr = 0x826DBE5C;
	sub_82466E20(ctx, base);
	// 826DBE5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBE70 size=108
    let mut pc: u32 = 0x826DBE70;
    'dispatch: loop {
        match pc {
            0x826DBE70 => {
    //   block [0x826DBE70..0x826DBEDC)
	// 826DBE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBE7C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBE80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBE84: 38EB60C8  addi r7, r11, 0x60c8
	ctx.r[7].s64 = ctx.r[11].s64 + 24776;
	// 826DBE88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DBE8C: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 826DBE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBE94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBE98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBEA0: 386AEEF4  addi r3, r10, -0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + -4364;
	// 826DBEA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBEC8: 4BD8AF59  bl 0x82466e20
	ctx.lr = 0x826DBECC;
	sub_82466E20(ctx, base);
	// 826DBECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBEE0 size=116
    let mut pc: u32 = 0x826DBEE0;
    'dispatch: loop {
        match pc {
            0x826DBEE0 => {
    //   block [0x826DBEE0..0x826DBF54)
	// 826DBEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBEEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBEF0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBEF4: 390B6114  addi r8, r11, 0x6114
	ctx.r[8].s64 = ctx.r[11].s64 + 24852;
	// 826DBEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBEFC: 392A57B8  addi r9, r10, 0x57b8
	ctx.r[9].s64 = ctx.r[10].s64 + 22456;
	// 826DBF00: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBF04: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DBF08: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBF0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBF14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBF24: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DBF28: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 826DBF2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBF30: 386BEF24  addi r3, r11, -0x10dc
	ctx.r[3].s64 = ctx.r[11].s64 + -4316;
	// 826DBF34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DBF38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBF40: 4BD8AEE1  bl 0x82466e20
	ctx.lr = 0x826DBF44;
	sub_82466E20(ctx, base);
	// 826DBF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DBF58 size=24
    let mut pc: u32 = 0x826DBF58;
    'dispatch: loop {
        match pc {
            0x826DBF58 => {
    //   block [0x826DBF58..0x826DBF70)
	// 826DBF58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBF5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DBF60: 394AB810  addi r10, r10, -0x47f0
	ctx.r[10].s64 = ctx.r[10].s64 + -18416;
	// 826DBF64: 816B612C  lwz r11, 0x612c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24876 as u32) ) } as u64;
	// 826DBF68: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826DBF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBF70 size=116
    let mut pc: u32 = 0x826DBF70;
    'dispatch: loop {
        match pc {
            0x826DBF70 => {
    //   block [0x826DBF70..0x826DBFE4)
	// 826DBF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBF7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DBF80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBF84: 390BB810  addi r8, r11, -0x47f0
	ctx.r[8].s64 = ctx.r[11].s64 + -18416;
	// 826DBF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBF8C: 392A5828  addi r9, r10, 0x5828
	ctx.r[9].s64 = ctx.r[10].s64 + 22568;
	// 826DBF90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBF94: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DBF98: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBF9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBFA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBFAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBFB4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DBFB8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 826DBFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBFC0: 386BEF54  addi r3, r11, -0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + -4268;
	// 826DBFC4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826DBFC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBFD0: 4BD8AE51  bl 0x82466e20
	ctx.lr = 0x826DBFD4;
	sub_82466E20(ctx, base);
	// 826DBFD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBFE8 size=108
    let mut pc: u32 = 0x826DBFE8;
    'dispatch: loop {
        match pc {
            0x826DBFE8 => {
    //   block [0x826DBFE8..0x826DC054)
	// 826DBFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBFF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBFF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBFFC: 38EB613C  addi r7, r11, 0x613c
	ctx.r[7].s64 = ctx.r[11].s64 + 24892;
	// 826DC000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC004: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 826DC008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC00C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC018: 386AEF84  addi r3, r10, -0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + -4220;
	// 826DC01C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC03C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC040: 4BD8ADE1  bl 0x82466e20
	ctx.lr = 0x826DC044;
	sub_82466E20(ctx, base);
	// 826DC044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC058 size=112
    let mut pc: u32 = 0x826DC058;
    'dispatch: loop {
        match pc {
            0x826DC058 => {
    //   block [0x826DC058..0x826DC0C8)
	// 826DC058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC068: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC06C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC074: 390B616C  addi r8, r11, 0x616c
	ctx.r[8].s64 = ctx.r[11].s64 + 24940;
	// 826DC078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC07C: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 826DC080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC090: 386AEFB4  addi r3, r10, -0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + -4172;
	// 826DC094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC0B4: 4BD8AD6D  bl 0x82466e20
	ctx.lr = 0x826DC0B8;
	sub_82466E20(ctx, base);
	// 826DC0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC0C8 size=112
    let mut pc: u32 = 0x826DC0C8;
    'dispatch: loop {
        match pc {
            0x826DC0C8 => {
    //   block [0x826DC0C8..0x826DC138)
	// 826DC0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC0D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC0D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC0DC: 392A5880  addi r9, r10, 0x5880
	ctx.r[9].s64 = ctx.r[10].s64 + 22656;
	// 826DC0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC0E4: 390B6188  addi r8, r11, 0x6188
	ctx.r[8].s64 = ctx.r[11].s64 + 24968;
	// 826DC0E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DC0EC: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 826DC0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC100: 386AEFE4  addi r3, r10, -0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + -4124;
	// 826DC104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC124: 4BD8ACFD  bl 0x82466e20
	ctx.lr = 0x826DC128;
	sub_82466E20(ctx, base);
	// 826DC128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC138 size=112
    let mut pc: u32 = 0x826DC138;
    'dispatch: loop {
        match pc {
            0x826DC138 => {
    //   block [0x826DC138..0x826DC1A8)
	// 826DC138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC14C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC154: 390B61D0  addi r8, r11, 0x61d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25040;
	// 826DC158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC15C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 826DC160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC170: 386AF014  addi r3, r10, -0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + -4076;
	// 826DC174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC194: 4BD8AC8D  bl 0x82466e20
	ctx.lr = 0x826DC198;
	sub_82466E20(ctx, base);
	// 826DC198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC1A8 size=112
    let mut pc: u32 = 0x826DC1A8;
    'dispatch: loop {
        match pc {
            0x826DC1A8 => {
    //   block [0x826DC1A8..0x826DC218)
	// 826DC1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC1B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC1BC: 392A58AC  addi r9, r10, 0x58ac
	ctx.r[9].s64 = ctx.r[10].s64 + 22700;
	// 826DC1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC1C4: 390B61F0  addi r8, r11, 0x61f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25072;
	// 826DC1C8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826DC1CC: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 826DC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC1E0: 386AF044  addi r3, r10, -0xfbc
	ctx.r[3].s64 = ctx.r[10].s64 + -4028;
	// 826DC1E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC1E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC204: 4BD8AC1D  bl 0x82466e20
	ctx.lr = 0x826DC208;
	sub_82466E20(ctx, base);
	// 826DC208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC218 size=112
    let mut pc: u32 = 0x826DC218;
    'dispatch: loop {
        match pc {
            0x826DC218 => {
    //   block [0x826DC218..0x826DC288)
	// 826DC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC22C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC234: 390B6280  addi r8, r11, 0x6280
	ctx.r[8].s64 = ctx.r[11].s64 + 25216;
	// 826DC238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC23C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 826DC240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC250: 386AF074  addi r3, r10, -0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3980;
	// 826DC254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC274: 4BD8ABAD  bl 0x82466e20
	ctx.lr = 0x826DC278;
	sub_82466E20(ctx, base);
	// 826DC278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC288 size=112
    let mut pc: u32 = 0x826DC288;
    'dispatch: loop {
        match pc {
            0x826DC288 => {
    //   block [0x826DC288..0x826DC2F8)
	// 826DC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC298: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC29C: 38AAF0D4  addi r5, r10, -0xf2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3884;
	// 826DC2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC2A4: 390B6298  addi r8, r11, 0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + 25240;
	// 826DC2A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DC2AC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826DC2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC2C0: 386AF0A4  addi r3, r10, -0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3932;
	// 826DC2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC2E4: 4BD8AB3D  bl 0x82466e20
	ctx.lr = 0x826DC2E8;
	sub_82466E20(ctx, base);
	// 826DC2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC2F8 size=100
    let mut pc: u32 = 0x826DC2F8;
    'dispatch: loop {
        match pc {
            0x826DC2F8 => {
    //   block [0x826DC2F8..0x826DC35C)
	// 826DC2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC30C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DC310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC318: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 826DC31C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC32C: 386AF0D4  addi r3, r10, -0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3884;
	// 826DC330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC338: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DC33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC340: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DC344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC348: 4BD8AAD9  bl 0x82466e20
	ctx.lr = 0x826DC34C;
	sub_82466E20(ctx, base);
	// 826DC34C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DC360 size=24
    let mut pc: u32 = 0x826DC360;
    'dispatch: loop {
        match pc {
            0x826DC360 => {
    //   block [0x826DC360..0x826DC378)
	// 826DC360: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC364: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DC368: 394AB8E8  addi r10, r10, -0x4718
	ctx.r[10].s64 = ctx.r[10].s64 + -18200;
	// 826DC36C: 816B61EC  lwz r11, 0x61ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25068 as u32) ) } as u64;
	// 826DC370: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826DC374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC378 size=116
    let mut pc: u32 = 0x826DC378;
    'dispatch: loop {
        match pc {
            0x826DC378 => {
    //   block [0x826DC378..0x826DC3EC)
	// 826DC378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC384: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DC388: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC38C: 390BB8E8  addi r8, r11, -0x4718
	ctx.r[8].s64 = ctx.r[11].s64 + -18200;
	// 826DC390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC394: 392A58E8  addi r9, r10, 0x58e8
	ctx.r[9].s64 = ctx.r[10].s64 + 22760;
	// 826DC398: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC39C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DC3A0: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC3A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC3AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC3BC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DC3C0: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826DC3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC3C8: 386BF104  addi r3, r11, -0xefc
	ctx.r[3].s64 = ctx.r[11].s64 + -3836;
	// 826DC3CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC3D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC3D8: 4BD8AA49  bl 0x82466e20
	ctx.lr = 0x826DC3DC;
	sub_82466E20(ctx, base);
	// 826DC3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC3F0 size=108
    let mut pc: u32 = 0x826DC3F0;
    'dispatch: loop {
        match pc {
            0x826DC3F0 => {
    //   block [0x826DC3F0..0x826DC45C)
	// 826DC3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC3FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC404: 38EB6310  addi r7, r11, 0x6310
	ctx.r[7].s64 = ctx.r[11].s64 + 25360;
	// 826DC408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC40C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 826DC410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC420: 386AF134  addi r3, r10, -0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + -3788;
	// 826DC424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC448: 4BD8A9D9  bl 0x82466e20
	ctx.lr = 0x826DC44C;
	sub_82466E20(ctx, base);
	// 826DC44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC460 size=112
    let mut pc: u32 = 0x826DC460;
    'dispatch: loop {
        match pc {
            0x826DC460 => {
    //   block [0x826DC460..0x826DC4D0)
	// 826DC460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC46C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC470: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC474: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC478: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC47C: 390B6340  addi r8, r11, 0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + 25408;
	// 826DC480: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC484: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826DC488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC48C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC498: 386AF164  addi r3, r10, -0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3740;
	// 826DC49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC4BC: 4BD8A965  bl 0x82466e20
	ctx.lr = 0x826DC4C0;
	sub_82466E20(ctx, base);
	// 826DC4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC4D0 size=112
    let mut pc: u32 = 0x826DC4D0;
    'dispatch: loop {
        match pc {
            0x826DC4D0 => {
    //   block [0x826DC4D0..0x826DC540)
	// 826DC4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC4DC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC4E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC4E4: 392A590C  addi r9, r10, 0x590c
	ctx.r[9].s64 = ctx.r[10].s64 + 22796;
	// 826DC4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC4EC: 390B6360  addi r8, r11, 0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + 25440;
	// 826DC4F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC4F4: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 826DC4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC508: 386AF194  addi r3, r10, -0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3692;
	// 826DC50C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC510: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC52C: 4BD8A8F5  bl 0x82466e20
	ctx.lr = 0x826DC530;
	sub_82466E20(ctx, base);
	// 826DC530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC540 size=112
    let mut pc: u32 = 0x826DC540;
    'dispatch: loop {
        match pc {
            0x826DC540 => {
    //   block [0x826DC540..0x826DC5B0)
	// 826DC540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC54C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC550: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC554: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC55C: 390B6408  addi r8, r11, 0x6408
	ctx.r[8].s64 = ctx.r[11].s64 + 25608;
	// 826DC560: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC564: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 826DC568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC578: 386AF1C4  addi r3, r10, -0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3644;
	// 826DC57C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC59C: 4BD8A885  bl 0x82466e20
	ctx.lr = 0x826DC5A0;
	sub_82466E20(ctx, base);
	// 826DC5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC5B0 size=112
    let mut pc: u32 = 0x826DC5B0;
    'dispatch: loop {
        match pc {
            0x826DC5B0 => {
    //   block [0x826DC5B0..0x826DC620)
	// 826DC5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC5BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC5C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC5C4: 392A5964  addi r9, r10, 0x5964
	ctx.r[9].s64 = ctx.r[10].s64 + 22884;
	// 826DC5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC5CC: 390B6428  addi r8, r11, 0x6428
	ctx.r[8].s64 = ctx.r[11].s64 + 25640;
	// 826DC5D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC5D4: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 826DC5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC5E8: 386AF1F4  addi r3, r10, -0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3596;
	// 826DC5EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC60C: 4BD8A815  bl 0x82466e20
	ctx.lr = 0x826DC610;
	sub_82466E20(ctx, base);
	// 826DC610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC620 size=116
    let mut pc: u32 = 0x826DC620;
    'dispatch: loop {
        match pc {
            0x826DC620 => {
    //   block [0x826DC620..0x826DC694)
	// 826DC620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC62C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC630: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC634: 390B64D0  addi r8, r11, 0x64d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25808;
	// 826DC638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC63C: 392A5938  addi r9, r10, 0x5938
	ctx.r[9].s64 = ctx.r[10].s64 + 22840;
	// 826DC640: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC644: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DC648: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC664: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DC668: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 826DC66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC670: 386BF224  addi r3, r11, -0xddc
	ctx.r[3].s64 = ctx.r[11].s64 + -3548;
	// 826DC674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC680: 4BD8A7A1  bl 0x82466e20
	ctx.lr = 0x826DC684;
	sub_82466E20(ctx, base);
	// 826DC684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC698 size=108
    let mut pc: u32 = 0x826DC698;
    'dispatch: loop {
        match pc {
            0x826DC698 => {
    //   block [0x826DC698..0x826DC704)
	// 826DC698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC6A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC6A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC6AC: 38EB64E8  addi r7, r11, 0x64e8
	ctx.r[7].s64 = ctx.r[11].s64 + 25832;
	// 826DC6B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC6B4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826DC6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC6BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC6C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC6C8: 386AF254  addi r3, r10, -0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + -3500;
	// 826DC6CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC6EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC6F0: 4BD8A731  bl 0x82466e20
	ctx.lr = 0x826DC6F4;
	sub_82466E20(ctx, base);
	// 826DC6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC708 size=112
    let mut pc: u32 = 0x826DC708;
    'dispatch: loop {
        match pc {
            0x826DC708 => {
    //   block [0x826DC708..0x826DC778)
	// 826DC708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC718: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC71C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC724: 390B6518  addi r8, r11, 0x6518
	ctx.r[8].s64 = ctx.r[11].s64 + 25880;
	// 826DC728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC72C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 826DC730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC740: 386AF284  addi r3, r10, -0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3452;
	// 826DC744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC764: 4BD8A6BD  bl 0x82466e20
	ctx.lr = 0x826DC768;
	sub_82466E20(ctx, base);
	// 826DC768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC778 size=112
    let mut pc: u32 = 0x826DC778;
    'dispatch: loop {
        match pc {
            0x826DC778 => {
    //   block [0x826DC778..0x826DC7E8)
	// 826DC778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC784: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC788: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC78C: 392A5998  addi r9, r10, 0x5998
	ctx.r[9].s64 = ctx.r[10].s64 + 22936;
	// 826DC790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC794: 390B6530  addi r8, r11, 0x6530
	ctx.r[8].s64 = ctx.r[11].s64 + 25904;
	// 826DC798: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC79C: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826DC7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC7A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC7B0: 386AF2B4  addi r3, r10, -0xd4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3404;
	// 826DC7B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC7B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC7D4: 4BD8A64D  bl 0x82466e20
	ctx.lr = 0x826DC7D8;
	sub_82466E20(ctx, base);
	// 826DC7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC7E8 size=112
    let mut pc: u32 = 0x826DC7E8;
    'dispatch: loop {
        match pc {
            0x826DC7E8 => {
    //   block [0x826DC7E8..0x826DC858)
	// 826DC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC7F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC7F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC7FC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC804: 390B65D8  addi r8, r11, 0x65d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26072;
	// 826DC808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DC80C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 826DC810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC820: 386AF2E4  addi r3, r10, -0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + -3356;
	// 826DC824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC844: 4BD8A5DD  bl 0x82466e20
	ctx.lr = 0x826DC848;
	sub_82466E20(ctx, base);
	// 826DC848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC858 size=108
    let mut pc: u32 = 0x826DC858;
    'dispatch: loop {
        match pc {
            0x826DC858 => {
    //   block [0x826DC858..0x826DC8C4)
	// 826DC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC864: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC86C: 38EB6620  addi r7, r11, 0x6620
	ctx.r[7].s64 = ctx.r[11].s64 + 26144;
	// 826DC870: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC874: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826DC878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC87C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC888: 386AF314  addi r3, r10, -0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + -3308;
	// 826DC88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC8B0: 4BD8A571  bl 0x82466e20
	ctx.lr = 0x826DC8B4;
	sub_82466E20(ctx, base);
	// 826DC8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC8C8 size=112
    let mut pc: u32 = 0x826DC8C8;
    'dispatch: loop {
        match pc {
            0x826DC8C8 => {
    //   block [0x826DC8C8..0x826DC938)
	// 826DC8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC8D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC8D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC8DC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC8E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC8E4: 390B6650  addi r8, r11, 0x6650
	ctx.r[8].s64 = ctx.r[11].s64 + 26192;
	// 826DC8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC8EC: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 826DC8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC8F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC900: 386AF344  addi r3, r10, -0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3260;
	// 826DC904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC924: 4BD8A4FD  bl 0x82466e20
	ctx.lr = 0x826DC928;
	sub_82466E20(ctx, base);
	// 826DC928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC938 size=112
    let mut pc: u32 = 0x826DC938;
    'dispatch: loop {
        match pc {
            0x826DC938 => {
    //   block [0x826DC938..0x826DC9A8)
	// 826DC938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC944: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC948: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC94C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC954: 390B6668  addi r8, r11, 0x6668
	ctx.r[8].s64 = ctx.r[11].s64 + 26216;
	// 826DC958: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826DC95C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 826DC960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC964: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC970: 386AF374  addi r3, r10, -0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3212;
	// 826DC974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC994: 4BD8A48D  bl 0x82466e20
	ctx.lr = 0x826DC998;
	sub_82466E20(ctx, base);
	// 826DC998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC9A8 size=100
    let mut pc: u32 = 0x826DC9A8;
    'dispatch: loop {
        match pc {
            0x826DC9A8 => {
    //   block [0x826DC9A8..0x826DCA0C)
	// 826DC9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC9B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC9BC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC9C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC9C8: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826DC9CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC9DC: 386AF3A4  addi r3, r10, -0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3164;
	// 826DC9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC9E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DC9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC9F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DC9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC9F8: 4BD8A429  bl 0x82466e20
	ctx.lr = 0x826DC9FC;
	sub_82466E20(ctx, base);
	// 826DC9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCA10 size=112
    let mut pc: u32 = 0x826DCA10;
    'dispatch: loop {
        match pc {
            0x826DCA10 => {
    //   block [0x826DCA10..0x826DCA80)
	// 826DCA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCA1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCA24: 38AAEF54  addi r5, r10, -0x10ac
	ctx.r[5].s64 = ctx.r[10].s64 + -4268;
	// 826DCA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCA2C: 390B6728  addi r8, r11, 0x6728
	ctx.r[8].s64 = ctx.r[11].s64 + 26408;
	// 826DCA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DCA34: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 826DCA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCA3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCA48: 386AF3D4  addi r3, r10, -0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3116;
	// 826DCA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCA6C: 4BD8A3B5  bl 0x82466e20
	ctx.lr = 0x826DCA70;
	sub_82466E20(ctx, base);
	// 826DCA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCA80 size=112
    let mut pc: u32 = 0x826DCA80;
    'dispatch: loop {
        match pc {
            0x826DCA80 => {
    //   block [0x826DCA80..0x826DCAF0)
	// 826DCA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCA8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCA94: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DCA98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCA9C: 390B6758  addi r8, r11, 0x6758
	ctx.r[8].s64 = ctx.r[11].s64 + 26456;
	// 826DCAA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCAA4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826DCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCAAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCAB8: 386AF404  addi r3, r10, -0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3068;
	// 826DCABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCADC: 4BD8A345  bl 0x82466e20
	ctx.lr = 0x826DCAE0;
	sub_82466E20(ctx, base);
	// 826DCAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCAF0 size=108
    let mut pc: u32 = 0x826DCAF0;
    'dispatch: loop {
        match pc {
            0x826DCAF0 => {
    //   block [0x826DCAF0..0x826DCB5C)
	// 826DCAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCAFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCB04: 38EB6770  addi r7, r11, 0x6770
	ctx.r[7].s64 = ctx.r[11].s64 + 26480;
	// 826DCB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DCB0C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 826DCB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCB14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCB20: 386AF434  addi r3, r10, -0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3020;
	// 826DCB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCB48: 4BD8A2D9  bl 0x82466e20
	ctx.lr = 0x826DCB4C;
	sub_82466E20(ctx, base);
	// 826DCB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCB60 size=112
    let mut pc: u32 = 0x826DCB60;
    'dispatch: loop {
        match pc {
            0x826DCB60 => {
    //   block [0x826DCB60..0x826DCBD0)
	// 826DCB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCB6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCB74: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCB7C: 390B67A0  addi r8, r11, 0x67a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26528;
	// 826DCB80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826DCB84: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 826DCB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCB8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCB98: 386AF464  addi r3, r10, -0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + -2972;
	// 826DCB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCBBC: 4BD8A265  bl 0x82466e20
	ctx.lr = 0x826DCBC0;
	sub_82466E20(ctx, base);
	// 826DCBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCBD0 size=112
    let mut pc: u32 = 0x826DCBD0;
    'dispatch: loop {
        match pc {
            0x826DCBD0 => {
    //   block [0x826DCBD0..0x826DCC40)
	// 826DCBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCBDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DCBE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCBE4: 392A59C4  addi r9, r10, 0x59c4
	ctx.r[9].s64 = ctx.r[10].s64 + 22980;
	// 826DCBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCBEC: 390B6838  addi r8, r11, 0x6838
	ctx.r[8].s64 = ctx.r[11].s64 + 26680;
	// 826DCBF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DCBF4: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 826DCBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCBFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCC08: 386AF494  addi r3, r10, -0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2924;
	// 826DCC0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCC10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DCC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCC2C: 4BD8A1F5  bl 0x82466e20
	ctx.lr = 0x826DCC30;
	sub_82466E20(ctx, base);
	// 826DCC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCC40 size=112
    let mut pc: u32 = 0x826DCC40;
    'dispatch: loop {
        match pc {
            0x826DCC40 => {
    //   block [0x826DCC40..0x826DCCB0)
	// 826DCC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCC4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCC54: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCC5C: 390B6880  addi r8, r11, 0x6880
	ctx.r[8].s64 = ctx.r[11].s64 + 26752;
	// 826DCC60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCC64: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 826DCC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCC6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCC78: 386AF4C4  addi r3, r10, -0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2876;
	// 826DCC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCC9C: 4BD8A185  bl 0x82466e20
	ctx.lr = 0x826DCCA0;
	sub_82466E20(ctx, base);
	// 826DCCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCCB0 size=108
    let mut pc: u32 = 0x826DCCB0;
    'dispatch: loop {
        match pc {
            0x826DCCB0 => {
    //   block [0x826DCCB0..0x826DCD1C)
	// 826DCCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCCBC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCCC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCCC4: 38EB6898  addi r7, r11, 0x6898
	ctx.r[7].s64 = ctx.r[11].s64 + 26776;
	// 826DCCC8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826DCCCC: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 826DCCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCCD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCCE0: 386AF4F4  addi r3, r10, -0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2828;
	// 826DCCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCD08: 4BD8A119  bl 0x82466e20
	ctx.lr = 0x826DCD0C;
	sub_82466E20(ctx, base);
	// 826DCD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCD20 size=116
    let mut pc: u32 = 0x826DCD20;
    'dispatch: loop {
        match pc {
            0x826DCD20 => {
    //   block [0x826DCD20..0x826DCD94)
	// 826DCD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCD2C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DCD30: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DCD34: 390A6928  addi r8, r10, 0x6928
	ctx.r[8].s64 = ctx.r[10].s64 + 26920;
	// 826DCD38: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCD3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DCD40: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCD44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCD48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCD54: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 826DCD58: 396B59D8  addi r11, r11, 0x59d8
	ctx.r[11].s64 = ctx.r[11].s64 + 23000;
	// 826DCD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCD60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCD64: 386AF524  addi r3, r10, -0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + -2780;
	// 826DCD68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DCD6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCD70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DCD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCD80: 4BD8A0A1  bl 0x82466e20
	ctx.lr = 0x826DCD84;
	sub_82466E20(ctx, base);
	// 826DCD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCD98 size=112
    let mut pc: u32 = 0x826DCD98;
    'dispatch: loop {
        match pc {
            0x826DCD98 => {
    //   block [0x826DCD98..0x826DCE08)
	// 826DCD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCDA4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DCDA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCDAC: 392A5A24  addi r9, r10, 0x5a24
	ctx.r[9].s64 = ctx.r[10].s64 + 23076;
	// 826DCDB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCDB4: 390B6A00  addi r8, r11, 0x6a00
	ctx.r[8].s64 = ctx.r[11].s64 + 27136;
	// 826DCDB8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826DCDBC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 826DCDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCDC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCDD0: 386AF554  addi r3, r10, -0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + -2732;
	// 826DCDD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCDD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DCDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCDF4: 4BD8A02D  bl 0x82466e20
	ctx.lr = 0x826DCDF8;
	sub_82466E20(ctx, base);
	// 826DCDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCE08 size=112
    let mut pc: u32 = 0x826DCE08;
    'dispatch: loop {
        match pc {
            0x826DCE08 => {
    //   block [0x826DCE08..0x826DCE78)
	// 826DCE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCE14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCE18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCE1C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCE20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCE24: 390B6A60  addi r8, r11, 0x6a60
	ctx.r[8].s64 = ctx.r[11].s64 + 27232;
	// 826DCE28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCE2C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 826DCE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCE40: 386AF584  addi r3, r10, -0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + -2684;
	// 826DCE44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCE64: 4BD89FBD  bl 0x82466e20
	ctx.lr = 0x826DCE68;
	sub_82466E20(ctx, base);
	// 826DCE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCE78 size=108
    let mut pc: u32 = 0x826DCE78;
    'dispatch: loop {
        match pc {
            0x826DCE78 => {
    //   block [0x826DCE78..0x826DCEE4)
	// 826DCE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCE84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCE88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCE8C: 38EB6A78  addi r7, r11, 0x6a78
	ctx.r[7].s64 = ctx.r[11].s64 + 27256;
	// 826DCE90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DCE94: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 826DCE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCE9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCEA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCEA8: 386AF5B4  addi r3, r10, -0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + -2636;
	// 826DCEAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCEB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCED0: 4BD89F51  bl 0x82466e20
	ctx.lr = 0x826DCED4;
	sub_82466E20(ctx, base);
	// 826DCED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCEE8 size=112
    let mut pc: u32 = 0x826DCEE8;
    'dispatch: loop {
        match pc {
            0x826DCEE8 => {
    //   block [0x826DCEE8..0x826DCF58)
	// 826DCEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCEF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCEF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCEFC: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCF00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCF04: 390B6AC0  addi r8, r11, 0x6ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 27328;
	// 826DCF08: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DCF0C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 826DCF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCF20: 386AF5E4  addi r3, r10, -0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + -2588;
	// 826DCF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCF44: 4BD89EDD  bl 0x82466e20
	ctx.lr = 0x826DCF48;
	sub_82466E20(ctx, base);
	// 826DCF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCF58 size=112
    let mut pc: u32 = 0x826DCF58;
    'dispatch: loop {
        match pc {
            0x826DCF58 => {
    //   block [0x826DCF58..0x826DCFC8)
	// 826DCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCF6C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCF70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCF74: 390B6B38  addi r8, r11, 0x6b38
	ctx.r[8].s64 = ctx.r[11].s64 + 27448;
	// 826DCF78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DCF7C: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 826DCF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCF84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCF90: 386AF614  addi r3, r10, -0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2540;
	// 826DCF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCFB4: 4BD89E6D  bl 0x82466e20
	ctx.lr = 0x826DCFB8;
	sub_82466E20(ctx, base);
	// 826DCFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCFC8 size=108
    let mut pc: u32 = 0x826DCFC8;
    'dispatch: loop {
        match pc {
            0x826DCFC8 => {
    //   block [0x826DCFC8..0x826DD034)
	// 826DCFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCFD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCFD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCFDC: 38EB6B68  addi r7, r11, 0x6b68
	ctx.r[7].s64 = ctx.r[11].s64 + 27496;
	// 826DCFE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DCFE4: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 826DCFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCFEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCFF8: 386AF644  addi r3, r10, -0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2492;
	// 826DCFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD020: 4BD89E01  bl 0x82466e20
	ctx.lr = 0x826DD024;
	sub_82466E20(ctx, base);
	// 826DD024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD038 size=108
    let mut pc: u32 = 0x826DD038;
    'dispatch: loop {
        match pc {
            0x826DD038 => {
    //   block [0x826DD038..0x826DD0A4)
	// 826DD038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD044: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD04C: 38EB6BC8  addi r7, r11, 0x6bc8
	ctx.r[7].s64 = ctx.r[11].s64 + 27592;
	// 826DD050: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826DD054: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 826DD058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD05C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD068: 386AF674  addi r3, r10, -0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + -2444;
	// 826DD06C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD090: 4BD89D91  bl 0x82466e20
	ctx.lr = 0x826DD094;
	sub_82466E20(ctx, base);
	// 826DD094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD0A8 size=112
    let mut pc: u32 = 0x826DD0A8;
    'dispatch: loop {
        match pc {
            0x826DD0A8 => {
    //   block [0x826DD0A8..0x826DD118)
	// 826DD0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD0B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD0B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD0BC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DD0C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD0C4: 390B6C40  addi r8, r11, 0x6c40
	ctx.r[8].s64 = ctx.r[11].s64 + 27712;
	// 826DD0C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DD0CC: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 826DD0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD0E0: 386AF6A4  addi r3, r10, -0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + -2396;
	// 826DD0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD104: 4BD89D1D  bl 0x82466e20
	ctx.lr = 0x826DD108;
	sub_82466E20(ctx, base);
	// 826DD108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD118 size=24
    let mut pc: u32 = 0x826DD118;
    'dispatch: loop {
        match pc {
            0x826DD118 => {
    //   block [0x826DD118..0x826DD130)
	// 826DD118: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD11C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD120: 394AB960  addi r10, r10, -0x46a0
	ctx.r[10].s64 = ctx.r[10].s64 + -18080;
	// 826DD124: 816B6C88  lwz r11, 0x6c88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27784 as u32) ) } as u64;
	// 826DD128: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DD12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD130 size=116
    let mut pc: u32 = 0x826DD130;
    'dispatch: loop {
        match pc {
            0x826DD130 => {
    //   block [0x826DD130..0x826DD1A4)
	// 826DD130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD13C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD140: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD144: 390BB960  addi r8, r11, -0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18080;
	// 826DD148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD14C: 392A5A68  addi r9, r10, 0x5a68
	ctx.r[9].s64 = ctx.r[10].s64 + 23144;
	// 826DD150: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD154: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DD158: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD15C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD164: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD174: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD178: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826DD17C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD180: 386BF6D4  addi r3, r11, -0x92c
	ctx.r[3].s64 = ctx.r[11].s64 + -2348;
	// 826DD184: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD190: 4BD89C91  bl 0x82466e20
	ctx.lr = 0x826DD194;
	sub_82466E20(ctx, base);
	// 826DD194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD1A8 size=112
    let mut pc: u32 = 0x826DD1A8;
    'dispatch: loop {
        match pc {
            0x826DD1A8 => {
    //   block [0x826DD1A8..0x826DD218)
	// 826DD1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD1BC: 38AAF6D4  addi r5, r10, -0x92c
	ctx.r[5].s64 = ctx.r[10].s64 + -2348;
	// 826DD1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD1C4: 390B6C8C  addi r8, r11, 0x6c8c
	ctx.r[8].s64 = ctx.r[11].s64 + 27788;
	// 826DD1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DD1CC: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 826DD1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD1E0: 386AF704  addi r3, r10, -0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2300;
	// 826DD1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD204: 4BD89C1D  bl 0x82466e20
	ctx.lr = 0x826DD208;
	sub_82466E20(ctx, base);
	// 826DD208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD218 size=24
    let mut pc: u32 = 0x826DD218;
    'dispatch: loop {
        match pc {
            0x826DD218 => {
    //   block [0x826DD218..0x826DD230)
	// 826DD218: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD21C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD220: 394AB978  addi r10, r10, -0x4688
	ctx.r[10].s64 = ctx.r[10].s64 + -18056;
	// 826DD224: 816B6CBC  lwz r11, 0x6cbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27836 as u32) ) } as u64;
	// 826DD228: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826DD22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD230 size=116
    let mut pc: u32 = 0x826DD230;
    'dispatch: loop {
        match pc {
            0x826DD230 => {
    //   block [0x826DD230..0x826DD2A4)
	// 826DD230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD23C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD240: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD244: 390BB978  addi r8, r11, -0x4688
	ctx.r[8].s64 = ctx.r[11].s64 + -18056;
	// 826DD248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD24C: 392A5AA4  addi r9, r10, 0x5aa4
	ctx.r[9].s64 = ctx.r[10].s64 + 23204;
	// 826DD250: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD254: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DD258: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD25C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD264: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD274: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD278: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826DD27C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD280: 386BF734  addi r3, r11, -0x8cc
	ctx.r[3].s64 = ctx.r[11].s64 + -2252;
	// 826DD284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD288: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD290: 4BD89B91  bl 0x82466e20
	ctx.lr = 0x826DD294;
	sub_82466E20(ctx, base);
	// 826DD294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD2A8 size=112
    let mut pc: u32 = 0x826DD2A8;
    'dispatch: loop {
        match pc {
            0x826DD2A8 => {
    //   block [0x826DD2A8..0x826DD318)
	// 826DD2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD2B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD2BC: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD2C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD2C4: 390B6CC0  addi r8, r11, 0x6cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 27840;
	// 826DD2C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DD2CC: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 826DD2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD2D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD2D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD2E0: 386AF764  addi r3, r10, -0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + -2204;
	// 826DD2E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD2E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD2FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD304: 4BD89B1D  bl 0x82466e20
	ctx.lr = 0x826DD308;
	sub_82466E20(ctx, base);
	// 826DD308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD318 size=112
    let mut pc: u32 = 0x826DD318;
    'dispatch: loop {
        match pc {
            0x826DD318 => {
    //   block [0x826DD318..0x826DD388)
	// 826DD318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD328: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD32C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD334: 390B6D20  addi r8, r11, 0x6d20
	ctx.r[8].s64 = ctx.r[11].s64 + 27936;
	// 826DD338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DD33C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 826DD340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD344: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD350: 386AF794  addi r3, r10, -0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + -2156;
	// 826DD354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD374: 4BD89AAD  bl 0x82466e20
	ctx.lr = 0x826DD378;
	sub_82466E20(ctx, base);
	// 826DD378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD388 size=112
    let mut pc: u32 = 0x826DD388;
    'dispatch: loop {
        match pc {
            0x826DD388 => {
    //   block [0x826DD388..0x826DD3F8)
	// 826DD388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD398: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD39C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD3A4: 390B6D50  addi r8, r11, 0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + 27984;
	// 826DD3A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DD3AC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826DD3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD3C0: 386AF7C4  addi r3, r10, -0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + -2108;
	// 826DD3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD3E4: 4BD89A3D  bl 0x82466e20
	ctx.lr = 0x826DD3E8;
	sub_82466E20(ctx, base);
	// 826DD3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD3F8 size=108
    let mut pc: u32 = 0x826DD3F8;
    'dispatch: loop {
        match pc {
            0x826DD3F8 => {
    //   block [0x826DD3F8..0x826DD464)
	// 826DD3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD404: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD40C: 38EB6D98  addi r7, r11, 0x6d98
	ctx.r[7].s64 = ctx.r[11].s64 + 28056;
	// 826DD410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD414: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 826DD418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD41C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD428: 386AF7F4  addi r3, r10, -0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + -2060;
	// 826DD42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD450: 4BD899D1  bl 0x82466e20
	ctx.lr = 0x826DD454;
	sub_82466E20(ctx, base);
	// 826DD454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD468 size=112
    let mut pc: u32 = 0x826DD468;
    'dispatch: loop {
        match pc {
            0x826DD468 => {
    //   block [0x826DD468..0x826DD4D8)
	// 826DD468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD478: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD47C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DD480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD484: 390B6DC8  addi r8, r11, 0x6dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 28104;
	// 826DD488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DD48C: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826DD490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD4A0: 386AF824  addi r3, r10, -0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2012;
	// 826DD4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD4C4: 4BD8995D  bl 0x82466e20
	ctx.lr = 0x826DD4C8;
	sub_82466E20(ctx, base);
	// 826DD4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD4D8 size=108
    let mut pc: u32 = 0x826DD4D8;
    'dispatch: loop {
        match pc {
            0x826DD4D8 => {
    //   block [0x826DD4D8..0x826DD544)
	// 826DD4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD4E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD4EC: 38EB6DE8  addi r7, r11, 0x6de8
	ctx.r[7].s64 = ctx.r[11].s64 + 28136;
	// 826DD4F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DD4F4: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 826DD4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD508: 386AF854  addi r3, r10, -0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1964;
	// 826DD50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD530: 4BD898F1  bl 0x82466e20
	ctx.lr = 0x826DD534;
	sub_82466E20(ctx, base);
	// 826DD534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD548 size=108
    let mut pc: u32 = 0x826DD548;
    'dispatch: loop {
        match pc {
            0x826DD548 => {
    //   block [0x826DD548..0x826DD5B4)
	// 826DD548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD554: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD55C: 38EB6E30  addi r7, r11, 0x6e30
	ctx.r[7].s64 = ctx.r[11].s64 + 28208;
	// 826DD560: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DD564: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 826DD568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD578: 386AF884  addi r3, r10, -0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + -1916;
	// 826DD57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD5A0: 4BD89881  bl 0x82466e20
	ctx.lr = 0x826DD5A4;
	sub_82466E20(ctx, base);
	// 826DD5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD5B8 size=108
    let mut pc: u32 = 0x826DD5B8;
    'dispatch: loop {
        match pc {
            0x826DD5B8 => {
    //   block [0x826DD5B8..0x826DD624)
	// 826DD5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD5C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD5CC: 38EB6E90  addi r7, r11, 0x6e90
	ctx.r[7].s64 = ctx.r[11].s64 + 28304;
	// 826DD5D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD5D4: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 826DD5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD5E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD5E8: 386AF8B4  addi r3, r10, -0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + -1868;
	// 826DD5EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD60C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD610: 4BD89811  bl 0x82466e20
	ctx.lr = 0x826DD614;
	sub_82466E20(ctx, base);
	// 826DD614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD628 size=116
    let mut pc: u32 = 0x826DD628;
    'dispatch: loop {
        match pc {
            0x826DD628 => {
    //   block [0x826DD628..0x826DD69C)
	// 826DD628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DD638: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD63C: 392B5AE0  addi r9, r11, 0x5ae0
	ctx.r[9].s64 = ctx.r[11].s64 + 23264;
	// 826DD640: 38AAFDC4  addi r5, r10, -0x23c
	ctx.r[5].s64 = ctx.r[10].s64 + -572;
	// 826DD644: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD648: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826DD64C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826DD650: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD654: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 826DD658: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD65C: 396B6EC0  addi r11, r11, 0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 28352;
	// 826DD660: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DD664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD668: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DD66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD670: 386AF8E4  addi r3, r10, -0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + -1820;
	// 826DD674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD678: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DD67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD680: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DD684: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD688: 4BD89799  bl 0x82466e20
	ctx.lr = 0x826DD68C;
	sub_82466E20(ctx, base);
	// 826DD68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD6A0 size=100
    let mut pc: u32 = 0x826DD6A0;
    'dispatch: loop {
        match pc {
            0x826DD6A0 => {
    //   block [0x826DD6A0..0x826DD704)
	// 826DD6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD6AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD6B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD6C0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826DD6C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD6D4: 386AF914  addi r3, r10, -0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1772;
	// 826DD6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD6DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD6E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD6E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD6F0: 4BD89731  bl 0x82466e20
	ctx.lr = 0x826DD6F4;
	sub_82466E20(ctx, base);
	// 826DD6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD708 size=100
    let mut pc: u32 = 0x826DD708;
    'dispatch: loop {
        match pc {
            0x826DD708 => {
    //   block [0x826DD708..0x826DD76C)
	// 826DD708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD71C: 38AAF9A4  addi r5, r10, -0x65c
	ctx.r[5].s64 = ctx.r[10].s64 + -1628;
	// 826DD720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD728: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 826DD72C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD73C: 386AF944  addi r3, r10, -0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1724;
	// 826DD740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD758: 4BD896C9  bl 0x82466e20
	ctx.lr = 0x826DD75C;
	sub_82466E20(ctx, base);
	// 826DD75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD770 size=100
    let mut pc: u32 = 0x826DD770;
    'dispatch: loop {
        match pc {
            0x826DD770 => {
    //   block [0x826DD770..0x826DD7D4)
	// 826DD770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD77C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD784: 38AAF8E4  addi r5, r10, -0x71c
	ctx.r[5].s64 = ctx.r[10].s64 + -1820;
	// 826DD788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD790: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826DD794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD7A4: 386AF974  addi r3, r10, -0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + -1676;
	// 826DD7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD7B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD7B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD7C0: 4BD89661  bl 0x82466e20
	ctx.lr = 0x826DD7C4;
	sub_82466E20(ctx, base);
	// 826DD7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD7D8 size=104
    let mut pc: u32 = 0x826DD7D8;
    'dispatch: loop {
        match pc {
            0x826DD7D8 => {
    //   block [0x826DD7D8..0x826DD840)
	// 826DD7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD7E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD7EC: 392A5B60  addi r9, r10, 0x5b60
	ctx.r[9].s64 = ctx.r[10].s64 + 23392;
	// 826DD7F0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD7F8: 38AAF914  addi r5, r10, -0x6ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1772;
	// 826DD7FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD80C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 826DD810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD818: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD824: 386AF9A4  addi r3, r10, -0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + -1628;
	// 826DD828: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD82C: 4BD895F5  bl 0x82466e20
	ctx.lr = 0x826DD830;
	sub_82466E20(ctx, base);
	// 826DD830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD840 size=108
    let mut pc: u32 = 0x826DD840;
    'dispatch: loop {
        match pc {
            0x826DD840 => {
    //   block [0x826DD840..0x826DD8AC)
	// 826DD840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD84C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD854: 38EB7058  addi r7, r11, 0x7058
	ctx.r[7].s64 = ctx.r[11].s64 + 28760;
	// 826DD858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD85C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 826DD860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD870: 386AF9D4  addi r3, r10, -0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + -1580;
	// 826DD874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD898: 4BD89589  bl 0x82466e20
	ctx.lr = 0x826DD89C;
	sub_82466E20(ctx, base);
	// 826DD89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD8B0 size=112
    let mut pc: u32 = 0x826DD8B0;
    'dispatch: loop {
        match pc {
            0x826DD8B0 => {
    //   block [0x826DD8B0..0x826DD920)
	// 826DD8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD8BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD8C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD8C4: 38AAF9A4  addi r5, r10, -0x65c
	ctx.r[5].s64 = ctx.r[10].s64 + -1628;
	// 826DD8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD8CC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 826DD8D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DD8D4: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 826DD8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD8E8: 386AFA04  addi r3, r10, -0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1532;
	// 826DD8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD90C: 4BD89515  bl 0x82466e20
	ctx.lr = 0x826DD910;
	sub_82466E20(ctx, base);
	// 826DD910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD920 size=24
    let mut pc: u32 = 0x826DD920;
    'dispatch: loop {
        match pc {
            0x826DD920 => {
    //   block [0x826DD920..0x826DD938)
	// 826DD920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD928: 394AB9F0  addi r10, r10, -0x4610
	ctx.r[10].s64 = ctx.r[10].s64 + -17936;
	// 826DD92C: 816B7130  lwz r11, 0x7130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28976 as u32) ) } as u64;
	// 826DD930: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DD934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD938 size=116
    let mut pc: u32 = 0x826DD938;
    'dispatch: loop {
        match pc {
            0x826DD938 => {
    //   block [0x826DD938..0x826DD9AC)
	// 826DD938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD944: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD948: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD94C: 390BB9F0  addi r8, r11, -0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + -17936;
	// 826DD950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD954: 392A5BD0  addi r9, r10, 0x5bd0
	ctx.r[9].s64 = ctx.r[10].s64 + 23504;
	// 826DD958: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD95C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826DD960: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD964: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD96C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD97C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD980: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826DD984: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD988: 386BFA34  addi r3, r11, -0x5cc
	ctx.r[3].s64 = ctx.r[11].s64 + -1484;
	// 826DD98C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD990: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD998: 4BD89489  bl 0x82466e20
	ctx.lr = 0x826DD99C;
	sub_82466E20(ctx, base);
	// 826DD99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD9B0 size=100
    let mut pc: u32 = 0x826DD9B0;
    'dispatch: loop {
        match pc {
            0x826DD9B0 => {
    //   block [0x826DD9B0..0x826DDA14)
	// 826DD9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD9C4: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DD9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD9D0: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 826DD9D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD9E4: 386AFA64  addi r3, r10, -0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + -1436;
	// 826DD9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD9EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD9F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD9F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDA00: 4BD89421  bl 0x82466e20
	ctx.lr = 0x826DDA04;
	sub_82466E20(ctx, base);
	// 826DDA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDA18 size=100
    let mut pc: u32 = 0x826DDA18;
    'dispatch: loop {
        match pc {
            0x826DDA18 => {
    //   block [0x826DDA18..0x826DDA7C)
	// 826DDA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDA24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDA2C: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDA30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDA38: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 826DDA3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDA4C: 386AFA94  addi r3, r10, -0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + -1388;
	// 826DDA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDA54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDA58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDA60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDA68: 4BD893B9  bl 0x82466e20
	ctx.lr = 0x826DDA6C;
	sub_82466E20(ctx, base);
	// 826DDA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDA80 size=100
    let mut pc: u32 = 0x826DDA80;
    'dispatch: loop {
        match pc {
            0x826DDA80 => {
    //   block [0x826DDA80..0x826DDAE4)
	// 826DDA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDA8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDA94: 38AAFAF4  addi r5, r10, -0x50c
	ctx.r[5].s64 = ctx.r[10].s64 + -1292;
	// 826DDA98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDAA0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826DDAA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDAB4: 386AFAC4  addi r3, r10, -0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + -1340;
	// 826DDAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDAC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDAC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDAD0: 4BD89351  bl 0x82466e20
	ctx.lr = 0x826DDAD4;
	sub_82466E20(ctx, base);
	// 826DDAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDAE8 size=100
    let mut pc: u32 = 0x826DDAE8;
    'dispatch: loop {
        match pc {
            0x826DDAE8 => {
    //   block [0x826DDAE8..0x826DDB4C)
	// 826DDAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDAF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDAFC: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDB08: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 826DDB0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDB1C: 386AFAF4  addi r3, r10, -0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + -1292;
	// 826DDB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDB24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDB28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDB30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDB38: 4BD892E9  bl 0x82466e20
	ctx.lr = 0x826DDB3C;
	sub_82466E20(ctx, base);
	// 826DDB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDB50 size=100
    let mut pc: u32 = 0x826DDB50;
    'dispatch: loop {
        match pc {
            0x826DDB50 => {
    //   block [0x826DDB50..0x826DDBB4)
	// 826DDB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDB5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDB64: 38AAFAF4  addi r5, r10, -0x50c
	ctx.r[5].s64 = ctx.r[10].s64 + -1292;
	// 826DDB68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDB70: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 826DDB74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDB84: 386AFB24  addi r3, r10, -0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1244;
	// 826DDB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDB8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDB90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDB98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDBA0: 4BD89281  bl 0x82466e20
	ctx.lr = 0x826DDBA4;
	sub_82466E20(ctx, base);
	// 826DDBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDBB8 size=100
    let mut pc: u32 = 0x826DDBB8;
    'dispatch: loop {
        match pc {
            0x826DDBB8 => {
    //   block [0x826DDBB8..0x826DDC1C)
	// 826DDBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDBCC: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDBD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDBD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDBD8: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 826DDBDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDBEC: 386AFB54  addi r3, r10, -0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1196;
	// 826DDBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDBF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDBF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDC00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDC08: 4BD89219  bl 0x82466e20
	ctx.lr = 0x826DDC0C;
	sub_82466E20(ctx, base);
	// 826DDC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDC20 size=100
    let mut pc: u32 = 0x826DDC20;
    'dispatch: loop {
        match pc {
            0x826DDC20 => {
    //   block [0x826DDC20..0x826DDC84)
	// 826DDC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDC2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDC34: 38AAFA64  addi r5, r10, -0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + -1436;
	// 826DDC38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDC40: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 826DDC44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDC54: 386AFB84  addi r3, r10, -0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + -1148;
	// 826DDC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDC60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDC68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDC70: 4BD891B1  bl 0x82466e20
	ctx.lr = 0x826DDC74;
	sub_82466E20(ctx, base);
	// 826DDC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDC88 size=100
    let mut pc: u32 = 0x826DDC88;
    'dispatch: loop {
        match pc {
            0x826DDC88 => {
    //   block [0x826DDC88..0x826DDCEC)
	// 826DDC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDC94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDC9C: 38AAFB54  addi r5, r10, -0x4ac
	ctx.r[5].s64 = ctx.r[10].s64 + -1196;
	// 826DDCA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDCA8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 826DDCAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDCBC: 386AFBB4  addi r3, r10, -0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + -1100;
	// 826DDCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDCC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDCC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDCD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDCD8: 4BD89149  bl 0x82466e20
	ctx.lr = 0x826DDCDC;
	sub_82466E20(ctx, base);
	// 826DDCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDCF0 size=100
    let mut pc: u32 = 0x826DDCF0;
    'dispatch: loop {
        match pc {
            0x826DDCF0 => {
    //   block [0x826DDCF0..0x826DDD54)
	// 826DDCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDCFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDD04: 38AAFA64  addi r5, r10, -0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + -1436;
	// 826DDD08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDD10: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 826DDD14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDD24: 386AFBE4  addi r3, r10, -0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + -1052;
	// 826DDD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDD2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDD30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDD38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDD40: 4BD890E1  bl 0x82466e20
	ctx.lr = 0x826DDD44;
	sub_82466E20(ctx, base);
	// 826DDD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDD58 size=112
    let mut pc: u32 = 0x826DDD58;
    'dispatch: loop {
        match pc {
            0x826DDD58 => {
    //   block [0x826DDD58..0x826DDDC8)
	// 826DDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDD64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDD68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDD6C: 38AAFC74  addi r5, r10, -0x38c
	ctx.r[5].s64 = ctx.r[10].s64 + -908;
	// 826DDD70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDD74: 390B7134  addi r8, r11, 0x7134
	ctx.r[8].s64 = ctx.r[11].s64 + 28980;
	// 826DDD78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DDD7C: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 826DDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDD84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DDD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDD90: 386AFC14  addi r3, r10, -0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1004;
	// 826DDD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDDB4: 4BD8906D  bl 0x82466e20
	ctx.lr = 0x826DDDB8;
	sub_82466E20(ctx, base);
	// 826DDDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDDC8 size=112
    let mut pc: u32 = 0x826DDDC8;
    'dispatch: loop {
        match pc {
            0x826DDDC8 => {
    //   block [0x826DDDC8..0x826DDE38)
	// 826DDDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDDD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDDD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDDDC: 38AAFCA4  addi r5, r10, -0x35c
	ctx.r[5].s64 = ctx.r[10].s64 + -860;
	// 826DDDE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDDE4: 390B7164  addi r8, r11, 0x7164
	ctx.r[8].s64 = ctx.r[11].s64 + 29028;
	// 826DDDE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DDDEC: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 826DDDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDDF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDDF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DDDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDE00: 386AFC44  addi r3, r10, -0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + -956;
	// 826DDE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DDE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDE24: 4BD88FFD  bl 0x82466e20
	ctx.lr = 0x826DDE28;
	sub_82466E20(ctx, base);
	// 826DDE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDE38 size=112
    let mut pc: u32 = 0x826DDE38;
    'dispatch: loop {
        match pc {
            0x826DDE38 => {
    //   block [0x826DDE38..0x826DDEA8)
	// 826DDE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDE48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDE4C: 38AAFDC4  addi r5, r10, -0x23c
	ctx.r[5].s64 = ctx.r[10].s64 + -572;
	// 826DDE50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDE54: 390B717C  addi r8, r11, 0x717c
	ctx.r[8].s64 = ctx.r[11].s64 + 29052;
	// 826DDE58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DDE5C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 826DDE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDE64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DDE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDE70: 386AFC74  addi r3, r10, -0x38c
	ctx.r[3].s64 = ctx.r[10].s64 + -908;
	// 826DDE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DDE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDE94: 4BD88F8D  bl 0x82466e20
	ctx.lr = 0x826DDE98;
	sub_82466E20(ctx, base);
	// 826DDE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDEA8 size=112
    let mut pc: u32 = 0x826DDEA8;
    'dispatch: loop {
        match pc {
            0x826DDEA8 => {
    //   block [0x826DDEA8..0x826DDF18)
	// 826DDEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDEB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDEB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDEBC: 38AAFC74  addi r5, r10, -0x38c
	ctx.r[5].s64 = ctx.r[10].s64 + -908;
	// 826DDEC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDEC4: 390B71AC  addi r8, r11, 0x71ac
	ctx.r[8].s64 = ctx.r[11].s64 + 29100;
	// 826DDEC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DDECC: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 826DDED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDED4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DDEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDEE0: 386AFCA4  addi r3, r10, -0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + -860;
	// 826DDEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DDEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDF04: 4BD88F1D  bl 0x82466e20
	ctx.lr = 0x826DDF08;
	sub_82466E20(ctx, base);
	// 826DDF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDF18 size=108
    let mut pc: u32 = 0x826DDF18;
    'dispatch: loop {
        match pc {
            0x826DDF18 => {
    //   block [0x826DDF18..0x826DDF84)
	// 826DDF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDF24: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDF2C: 38EB71C4  addi r7, r11, 0x71c4
	ctx.r[7].s64 = ctx.r[11].s64 + 29124;
	// 826DDF30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DDF34: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 826DDF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDF3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DDF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDF48: 386AFCD4  addi r3, r10, -0x32c
	ctx.r[3].s64 = ctx.r[10].s64 + -812;
	// 826DDF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DDF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DDF70: 4BD88EB1  bl 0x82466e20
	ctx.lr = 0x826DDF74;
	sub_82466E20(ctx, base);
	// 826DDF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDF88 size=112
    let mut pc: u32 = 0x826DDF88;
    'dispatch: loop {
        match pc {
            0x826DDF88 => {
    //   block [0x826DDF88..0x826DDFF8)
	// 826DDF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDF98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DDF9C: 38AAFCA4  addi r5, r10, -0x35c
	ctx.r[5].s64 = ctx.r[10].s64 + -860;
	// 826DDFA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDFA4: 390B71DC  addi r8, r11, 0x71dc
	ctx.r[8].s64 = ctx.r[11].s64 + 29148;
	// 826DDFA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DDFAC: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 826DDFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDFB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DDFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDFC0: 386AFD04  addi r3, r10, -0x2fc
	ctx.r[3].s64 = ctx.r[10].s64 + -764;
	// 826DDFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DDFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDFE4: 4BD88E3D  bl 0x82466e20
	ctx.lr = 0x826DDFE8;
	sub_82466E20(ctx, base);
	// 826DDFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDFF8 size=116
    let mut pc: u32 = 0x826DDFF8;
    'dispatch: loop {
        match pc {
            0x826DDFF8 => {
    //   block [0x826DDFF8..0x826DE06C)
	// 826DDFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE004: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DE008: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DE00C: 390A71F8  addi r8, r10, 0x71f8
	ctx.r[8].s64 = ctx.r[10].s64 + 29176;
	// 826DE010: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE014: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DE018: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE01C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE020: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DE024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE02C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 826DE030: 396B5BE4  addi r11, r11, 0x5be4
	ctx.r[11].s64 = ctx.r[11].s64 + 23524;
	// 826DE034: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE038: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE03C: 386AFD34  addi r3, r10, -0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + -716;
	// 826DE040: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DE044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE048: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DE04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE058: 4BD88DC9  bl 0x82466e20
	ctx.lr = 0x826DE05C;
	sub_82466E20(ctx, base);
	// 826DE05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE070 size=116
    let mut pc: u32 = 0x826DE070;
    'dispatch: loop {
        match pc {
            0x826DE070 => {
    //   block [0x826DE070..0x826DE0E4)
	// 826DE070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE07C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE080: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DE084: 390B72B8  addi r8, r11, 0x72b8
	ctx.r[8].s64 = ctx.r[11].s64 + 29368;
	// 826DE088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE08C: 392A5D0C  addi r9, r10, 0x5d0c
	ctx.r[9].s64 = ctx.r[10].s64 + 23820;
	// 826DE090: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE094: 38E00043  li r7, 0x43
	ctx.r[7].s64 = 67;
	// 826DE098: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE09C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE0A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE0B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DE0B8: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 826DE0BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DE0C0: 386BFD64  addi r3, r11, -0x29c
	ctx.r[3].s64 = ctx.r[11].s64 + -668;
	// 826DE0C4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826DE0C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE0D0: 4BD88D51  bl 0x82466e20
	ctx.lr = 0x826DE0D4;
	sub_82466E20(ctx, base);
	// 826DE0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DE0E8 size=48
    let mut pc: u32 = 0x826DE0E8;
    'dispatch: loop {
        match pc {
            0x826DE0E8 => {
    //   block [0x826DE0E8..0x826DE118)
	// 826DE0E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE0EC: 814B790C  lwz r10, 0x790c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30988 as u32) ) } as u64;
	// 826DE0F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DE0F4: 396BBAF8  addi r11, r11, -0x4508
	ctx.r[11].s64 = ctx.r[11].s64 + -17672;
	// 826DE0F8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826DE0FC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DE100: 814A7908  lwz r10, 0x7908(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30984 as u32) ) } as u64;
	// 826DE104: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826DE108: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DE10C: 814A7904  lwz r10, 0x7904(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30980 as u32) ) } as u64;
	// 826DE110: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 826DE114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE118 size=116
    let mut pc: u32 = 0x826DE118;
    'dispatch: loop {
        match pc {
            0x826DE118 => {
    //   block [0x826DE118..0x826DE18C)
	// 826DE118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DE128: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE12C: 392B5DE8  addi r9, r11, 0x5de8
	ctx.r[9].s64 = ctx.r[11].s64 + 24040;
	// 826DE130: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE134: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE138: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826DE13C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 826DE140: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DE144: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 826DE148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE14C: 396BBAF8  addi r11, r11, -0x4508
	ctx.r[11].s64 = ctx.r[11].s64 + -17672;
	// 826DE150: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DE154: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE158: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DE15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE160: 386AFD94  addi r3, r10, -0x26c
	ctx.r[3].s64 = ctx.r[10].s64 + -620;
	// 826DE164: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826DE168: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DE16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE170: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DE174: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DE178: 4BD88CA9  bl 0x82466e20
	ctx.lr = 0x826DE17C;
	sub_82466E20(ctx, base);
	// 826DE17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE190 size=116
    let mut pc: u32 = 0x826DE190;
    'dispatch: loop {
        match pc {
            0x826DE190 => {
    //   block [0x826DE190..0x826DE204)
	// 826DE190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE19C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE1A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DE1A4: 390B7918  addi r8, r11, 0x7918
	ctx.r[8].s64 = ctx.r[11].s64 + 31000;
	// 826DE1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE1AC: 392A5F84  addi r9, r10, 0x5f84
	ctx.r[9].s64 = ctx.r[10].s64 + 24452;
	// 826DE1B0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE1B4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826DE1B8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE1BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE1C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE1D4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DE1D8: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 826DE1DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DE1E0: 386BFDC4  addi r3, r11, -0x23c
	ctx.r[3].s64 = ctx.r[11].s64 + -572;
	// 826DE1E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DE1E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE1F0: 4BD88C31  bl 0x82466e20
	ctx.lr = 0x826DE1F4;
	sub_82466E20(ctx, base);
	// 826DE1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE208 size=112
    let mut pc: u32 = 0x826DE208;
    'dispatch: loop {
        match pc {
            0x826DE208 => {
    //   block [0x826DE208..0x826DE278)
	// 826DE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE214: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE218: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE21C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE224: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 826DE228: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DE22C: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 826DE230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE234: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE240: 386AFDF4  addi r3, r10, -0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + -524;
	// 826DE244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE264: 4BD88BBD  bl 0x82466e20
	ctx.lr = 0x826DE268;
	sub_82466E20(ctx, base);
	// 826DE268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DE278 size=36
    let mut pc: u32 = 0x826DE278;
    'dispatch: loop {
        match pc {
            0x826DE278 => {
    //   block [0x826DE278..0x826DE29C)
	// 826DE278: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE27C: 814B79C8  lwz r10, 0x79c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31176 as u32) ) } as u64;
	// 826DE280: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DE284: 396BBEA0  addi r11, r11, -0x4160
	ctx.r[11].s64 = ctx.r[11].s64 + -16736;
	// 826DE288: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826DE28C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DE290: 814A79C0  lwz r10, 0x79c0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31168 as u32) ) } as u64;
	// 826DE294: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826DE298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE2A0 size=116
    let mut pc: u32 = 0x826DE2A0;
    'dispatch: loop {
        match pc {
            0x826DE2A0 => {
    //   block [0x826DE2A0..0x826DE314)
	// 826DE2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE2AC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DE2B0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DE2B4: 390BBEA0  addi r8, r11, -0x4160
	ctx.r[8].s64 = ctx.r[11].s64 + -16736;
	// 826DE2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE2BC: 392A5FEC  addi r9, r10, 0x5fec
	ctx.r[9].s64 = ctx.r[10].s64 + 24556;
	// 826DE2C0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE2C4: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826DE2C8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE2CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE2D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE2E4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DE2E8: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 826DE2EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DE2F0: 386BFE24  addi r3, r11, -0x1dc
	ctx.r[3].s64 = ctx.r[11].s64 + -476;
	// 826DE2F4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826DE2F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE2FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE300: 4BD88B21  bl 0x82466e20
	ctx.lr = 0x826DE304;
	sub_82466E20(ctx, base);
	// 826DE304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE318 size=108
    let mut pc: u32 = 0x826DE318;
    'dispatch: loop {
        match pc {
            0x826DE318 => {
    //   block [0x826DE318..0x826DE384)
	// 826DE318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE324: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE328: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE32C: 38EB79D0  addi r7, r11, 0x79d0
	ctx.r[7].s64 = ctx.r[11].s64 + 31184;
	// 826DE330: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826DE334: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 826DE338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE33C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE348: 386AFE54  addi r3, r10, -0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + -428;
	// 826DE34C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE36C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DE370: 4BD88AB1  bl 0x82466e20
	ctx.lr = 0x826DE374;
	sub_82466E20(ctx, base);
	// 826DE374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE388 size=112
    let mut pc: u32 = 0x826DE388;
    'dispatch: loop {
        match pc {
            0x826DE388 => {
    //   block [0x826DE388..0x826DE3F8)
	// 826DE388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE398: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE39C: 38AADB74  addi r5, r10, -0x248c
	ctx.r[5].s64 = ctx.r[10].s64 + -9356;
	// 826DE3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE3A4: 390B7A48  addi r8, r11, 0x7a48
	ctx.r[8].s64 = ctx.r[11].s64 + 31304;
	// 826DE3A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DE3AC: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826DE3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE3C0: 386AFE84  addi r3, r10, -0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + -380;
	// 826DE3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE3E4: 4BD88A3D  bl 0x82466e20
	ctx.lr = 0x826DE3E8;
	sub_82466E20(ctx, base);
	// 826DE3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE3F8 size=108
    let mut pc: u32 = 0x826DE3F8;
    'dispatch: loop {
        match pc {
            0x826DE3F8 => {
    //   block [0x826DE3F8..0x826DE464)
	// 826DE3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE404: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE40C: 38EB7A60  addi r7, r11, 0x7a60
	ctx.r[7].s64 = ctx.r[11].s64 + 31328;
	// 826DE410: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DE414: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 826DE418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE41C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE428: 386AFEB4  addi r3, r10, -0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + -332;
	// 826DE42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DE450: 4BD889D1  bl 0x82466e20
	ctx.lr = 0x826DE454;
	sub_82466E20(ctx, base);
	// 826DE454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE468 size=112
    let mut pc: u32 = 0x826DE468;
    'dispatch: loop {
        match pc {
            0x826DE468 => {
    //   block [0x826DE468..0x826DE4D8)
	// 826DE468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE478: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE47C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE484: 390B7A78  addi r8, r11, 0x7a78
	ctx.r[8].s64 = ctx.r[11].s64 + 31352;
	// 826DE488: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DE48C: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826DE490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE4A0: 386AFEE4  addi r3, r10, -0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + -284;
	// 826DE4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE4C4: 4BD8895D  bl 0x82466e20
	ctx.lr = 0x826DE4C8;
	sub_82466E20(ctx, base);
	// 826DE4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE4D8 size=108
    let mut pc: u32 = 0x826DE4D8;
    'dispatch: loop {
        match pc {
            0x826DE4D8 => {
    //   block [0x826DE4D8..0x826DE544)
	// 826DE4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE4E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE4EC: 38EB7AC0  addi r7, r11, 0x7ac0
	ctx.r[7].s64 = ctx.r[11].s64 + 31424;
	// 826DE4F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DE4F4: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 826DE4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE508: 386AFF14  addi r3, r10, -0xec
	ctx.r[3].s64 = ctx.r[10].s64 + -236;
	// 826DE50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DE530: 4BD888F1  bl 0x82466e20
	ctx.lr = 0x826DE534;
	sub_82466E20(ctx, base);
	// 826DE534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE548 size=108
    let mut pc: u32 = 0x826DE548;
    'dispatch: loop {
        match pc {
            0x826DE548 => {
    //   block [0x826DE548..0x826DE5B4)
	// 826DE548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE554: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE55C: 38EB7AF0  addi r7, r11, 0x7af0
	ctx.r[7].s64 = ctx.r[11].s64 + 31472;
	// 826DE560: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DE564: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 826DE568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE578: 386AFF44  addi r3, r10, -0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + -188;
	// 826DE57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DE5A0: 4BD88881  bl 0x82466e20
	ctx.lr = 0x826DE5A4;
	sub_82466E20(ctx, base);
	// 826DE5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE5B8 size=112
    let mut pc: u32 = 0x826DE5B8;
    'dispatch: loop {
        match pc {
            0x826DE5B8 => {
    //   block [0x826DE5B8..0x826DE628)
	// 826DE5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE5C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE5C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE5CC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DE5D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE5D4: 390B7B08  addi r8, r11, 0x7b08
	ctx.r[8].s64 = ctx.r[11].s64 + 31496;
	// 826DE5D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DE5DC: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 826DE5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE5E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE5F0: 386AFF74  addi r3, r10, -0x8c
	ctx.r[3].s64 = ctx.r[10].s64 + -140;
	// 826DE5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE614: 4BD8880D  bl 0x82466e20
	ctx.lr = 0x826DE618;
	sub_82466E20(ctx, base);
	// 826DE618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE628 size=112
    let mut pc: u32 = 0x826DE628;
    'dispatch: loop {
        match pc {
            0x826DE628 => {
    //   block [0x826DE628..0x826DE698)
	// 826DE628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE638: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE63C: 38AAEE34  addi r5, r10, -0x11cc
	ctx.r[5].s64 = ctx.r[10].s64 + -4556;
	// 826DE640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE644: 390B7B38  addi r8, r11, 0x7b38
	ctx.r[8].s64 = ctx.r[11].s64 + 31544;
	// 826DE648: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DE64C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 826DE650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE654: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE660: 386AFFA4  addi r3, r10, -0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + -92;
	// 826DE664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE684: 4BD8879D  bl 0x82466e20
	ctx.lr = 0x826DE688;
	sub_82466E20(ctx, base);
	// 826DE688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE698 size=112
    let mut pc: u32 = 0x826DE698;
    'dispatch: loop {
        match pc {
            0x826DE698 => {
    //   block [0x826DE698..0x826DE708)
	// 826DE698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE6A8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE6AC: 38AAEE34  addi r5, r10, -0x11cc
	ctx.r[5].s64 = ctx.r[10].s64 + -4556;
	// 826DE6B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE6B4: 390B7B80  addi r8, r11, 0x7b80
	ctx.r[8].s64 = ctx.r[11].s64 + 31616;
	// 826DE6B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DE6BC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826DE6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE6C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE6D0: 386AFFD4  addi r3, r10, -0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + -44;
	// 826DE6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE6F4: 4BD8872D  bl 0x82466e20
	ctx.lr = 0x826DE6F8;
	sub_82466E20(ctx, base);
	// 826DE6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE708 size=112
    let mut pc: u32 = 0x826DE708;
    'dispatch: loop {
        match pc {
            0x826DE708 => {
    //   block [0x826DE708..0x826DE778)
	// 826DE708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE718: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE71C: 38AAEE64  addi r5, r10, -0x119c
	ctx.r[5].s64 = ctx.r[10].s64 + -4508;
	// 826DE720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE724: 390B7BE0  addi r8, r11, 0x7be0
	ctx.r[8].s64 = ctx.r[11].s64 + 31712;
	// 826DE728: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DE72C: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 826DE730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE740: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 826DE744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE764: 4BD886BD  bl 0x82466e20
	ctx.lr = 0x826DE768;
	sub_82466E20(ctx, base);
	// 826DE768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE778 size=112
    let mut pc: u32 = 0x826DE778;
    'dispatch: loop {
        match pc {
            0x826DE778 => {
    //   block [0x826DE778..0x826DE7E8)
	// 826DE778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE788: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE78C: 38AAEE64  addi r5, r10, -0x119c
	ctx.r[5].s64 = ctx.r[10].s64 + -4508;
	// 826DE790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE794: 390B7C40  addi r8, r11, 0x7c40
	ctx.r[8].s64 = ctx.r[11].s64 + 31808;
	// 826DE798: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826DE79C: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826DE7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE7A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE7B0: 386A0034  addi r3, r10, 0x34
	ctx.r[3].s64 = ctx.r[10].s64 + 52;
	// 826DE7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE7D4: 4BD8864D  bl 0x82466e20
	ctx.lr = 0x826DE7D8;
	sub_82466E20(ctx, base);
	// 826DE7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE7E8 size=112
    let mut pc: u32 = 0x826DE7E8;
    'dispatch: loop {
        match pc {
            0x826DE7E8 => {
    //   block [0x826DE7E8..0x826DE858)
	// 826DE7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE7F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE7F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE7FC: 38AAEE64  addi r5, r10, -0x119c
	ctx.r[5].s64 = ctx.r[10].s64 + -4508;
	// 826DE800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE804: 390B7D00  addi r8, r11, 0x7d00
	ctx.r[8].s64 = ctx.r[11].s64 + 32000;
	// 826DE808: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DE80C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 826DE810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE820: 386A0064  addi r3, r10, 0x64
	ctx.r[3].s64 = ctx.r[10].s64 + 100;
	// 826DE824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE844: 4BD885DD  bl 0x82466e20
	ctx.lr = 0x826DE848;
	sub_82466E20(ctx, base);
	// 826DE848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE858 size=112
    let mut pc: u32 = 0x826DE858;
    'dispatch: loop {
        match pc {
            0x826DE858 => {
    //   block [0x826DE858..0x826DE8C8)
	// 826DE858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE868: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE86C: 38AAEE34  addi r5, r10, -0x11cc
	ctx.r[5].s64 = ctx.r[10].s64 + -4556;
	// 826DE870: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE874: 390B7D60  addi r8, r11, 0x7d60
	ctx.r[8].s64 = ctx.r[11].s64 + 32096;
	// 826DE878: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826DE87C: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826DE880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE884: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE890: 386A0094  addi r3, r10, 0x94
	ctx.r[3].s64 = ctx.r[10].s64 + 148;
	// 826DE894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE8B4: 4BD8856D  bl 0x82466e20
	ctx.lr = 0x826DE8B8;
	sub_82466E20(ctx, base);
	// 826DE8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE8C8 size=112
    let mut pc: u32 = 0x826DE8C8;
    'dispatch: loop {
        match pc {
            0x826DE8C8 => {
    //   block [0x826DE8C8..0x826DE938)
	// 826DE8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE8D4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DE8D8: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826DE8DC: 38EA7E20  addi r7, r10, 0x7e20
	ctx.r[7].s64 = ctx.r[10].s64 + 32288;
	// 826DE8E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE8E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DE8E8: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 826DE8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE8F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE8F4: 396B6018  addi r11, r11, 0x6018
	ctx.r[11].s64 = ctx.r[11].s64 + 24600;
	// 826DE8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE8FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE900: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE904: 386A00C4  addi r3, r10, 0xc4
	ctx.r[3].s64 = ctx.r[10].s64 + 196;
	// 826DE908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE90C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DE910: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE914: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DE918: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE91C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE920: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DE924: 4BD884FD  bl 0x82466e20
	ctx.lr = 0x826DE928;
	sub_82466E20(ctx, base);
	// 826DE928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE938 size=112
    let mut pc: u32 = 0x826DE938;
    'dispatch: loop {
        match pc {
            0x826DE938 => {
    //   block [0x826DE938..0x826DE9A8)
	// 826DE938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE944: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE948: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DE94C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826DE950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE954: 390B7FE8  addi r8, r11, 0x7fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 32744;
	// 826DE958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DE95C: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 826DE960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE964: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DE96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE970: 386A00F4  addi r3, r10, 0xf4
	ctx.r[3].s64 = ctx.r[10].s64 + 244;
	// 826DE974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DE978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DE988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE994: 4BD8848D  bl 0x82466e20
	ctx.lr = 0x826DE998;
	sub_82466E20(ctx, base);
	// 826DE998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DE99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DE9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DE9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DE9A8 size=108
    let mut pc: u32 = 0x826DE9A8;
    'dispatch: loop {
        match pc {
            0x826DE9A8 => {
    //   block [0x826DE9A8..0x826DEA14)
	// 826DE9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DE9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DE9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DE9B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DE9B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DE9BC: 38EB8000  addi r7, r11, -0x8000
	ctx.r[7].s64 = ctx.r[11].s64 + -32768;
	// 826DE9C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DE9C4: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826DE9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DE9CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DE9D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DE9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DE9D8: 386A0124  addi r3, r10, 0x124
	ctx.r[3].s64 = ctx.r[10].s64 + 292;
	// 826DE9DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DE9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DE9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DE9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DE9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DE9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DE9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DE9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DE9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEA00: 4BD88421  bl 0x82466e20
	ctx.lr = 0x826DEA04;
	sub_82466E20(ctx, base);
	// 826DEA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEA18 size=112
    let mut pc: u32 = 0x826DEA18;
    'dispatch: loop {
        match pc {
            0x826DEA18 => {
    //   block [0x826DEA18..0x826DEA88)
	// 826DEA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEA24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEA28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEA2C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826DEA30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEA34: 390B8030  addi r8, r11, -0x7fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -32720;
	// 826DEA38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DEA3C: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826DEA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEA44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEA50: 386A0154  addi r3, r10, 0x154
	ctx.r[3].s64 = ctx.r[10].s64 + 340;
	// 826DEA54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DEA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEA64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DEA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEA74: 4BD883AD  bl 0x82466e20
	ctx.lr = 0x826DEA78;
	sub_82466E20(ctx, base);
	// 826DEA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEA88 size=108
    let mut pc: u32 = 0x826DEA88;
    'dispatch: loop {
        match pc {
            0x826DEA88 => {
    //   block [0x826DEA88..0x826DEAF4)
	// 826DEA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEA94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEA98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEA9C: 38EB8048  addi r7, r11, -0x7fb8
	ctx.r[7].s64 = ctx.r[11].s64 + -32696;
	// 826DEAA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DEAA4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826DEAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEAAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEAB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DEAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEAB8: 386A0184  addi r3, r10, 0x184
	ctx.r[3].s64 = ctx.r[10].s64 + 388;
	// 826DEABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DEAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEAE0: 4BD88341  bl 0x82466e20
	ctx.lr = 0x826DEAE4;
	sub_82466E20(ctx, base);
	// 826DEAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEAF8 size=108
    let mut pc: u32 = 0x826DEAF8;
    'dispatch: loop {
        match pc {
            0x826DEAF8 => {
    //   block [0x826DEAF8..0x826DEB64)
	// 826DEAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEB04: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEB08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEB0C: 38EB8078  addi r7, r11, -0x7f88
	ctx.r[7].s64 = ctx.r[11].s64 + -32648;
	// 826DEB10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DEB14: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826DEB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEB1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DEB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEB28: 386A01B4  addi r3, r10, 0x1b4
	ctx.r[3].s64 = ctx.r[10].s64 + 436;
	// 826DEB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DEB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEB50: 4BD882D1  bl 0x82466e20
	ctx.lr = 0x826DEB54;
	sub_82466E20(ctx, base);
	// 826DEB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEB68 size=112
    let mut pc: u32 = 0x826DEB68;
    'dispatch: loop {
        match pc {
            0x826DEB68 => {
    //   block [0x826DEB68..0x826DEBD8)
	// 826DEB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEB74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEB78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEB7C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DEB80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEB84: 390B80C0  addi r8, r11, -0x7f40
	ctx.r[8].s64 = ctx.r[11].s64 + -32576;
	// 826DEB88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DEB8C: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826DEB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEB94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEB98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEBA0: 386A01E4  addi r3, r10, 0x1e4
	ctx.r[3].s64 = ctx.r[10].s64 + 484;
	// 826DEBA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DEBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEBC4: 4BD8825D  bl 0x82466e20
	ctx.lr = 0x826DEBC8;
	sub_82466E20(ctx, base);
	// 826DEBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEBD8 size=112
    let mut pc: u32 = 0x826DEBD8;
    'dispatch: loop {
        match pc {
            0x826DEBD8 => {
    //   block [0x826DEBD8..0x826DEC48)
	// 826DEBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEBE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEBE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEBEC: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826DEBF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEBF4: 390B8108  addi r8, r11, -0x7ef8
	ctx.r[8].s64 = ctx.r[11].s64 + -32504;
	// 826DEBF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DEBFC: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826DEC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEC04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEC08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEC10: 386A0214  addi r3, r10, 0x214
	ctx.r[3].s64 = ctx.r[10].s64 + 532;
	// 826DEC14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DEC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEC1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEC24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DEC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEC34: 4BD881ED  bl 0x82466e20
	ctx.lr = 0x826DEC38;
	sub_82466E20(ctx, base);
	// 826DEC38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEC48 size=112
    let mut pc: u32 = 0x826DEC48;
    'dispatch: loop {
        match pc {
            0x826DEC48 => {
    //   block [0x826DEC48..0x826DECB8)
	// 826DEC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEC58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEC5C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826DEC60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEC64: 390B8120  addi r8, r11, -0x7ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -32480;
	// 826DEC68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DEC6C: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826DEC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEC74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEC80: 386A0244  addi r3, r10, 0x244
	ctx.r[3].s64 = ctx.r[10].s64 + 580;
	// 826DEC84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DEC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DECA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DECA4: 4BD8817D  bl 0x82466e20
	ctx.lr = 0x826DECA8;
	sub_82466E20(ctx, base);
	// 826DECA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DECAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DECB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DECB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DECB8 size=112
    let mut pc: u32 = 0x826DECB8;
    'dispatch: loop {
        match pc {
            0x826DECB8 => {
    //   block [0x826DECB8..0x826DED28)
	// 826DECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DECC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DECC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DECC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DECCC: 38AAFD34  addi r5, r10, -0x2cc
	ctx.r[5].s64 = ctx.r[10].s64 + -716;
	// 826DECD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DECD4: 390B8150  addi r8, r11, -0x7eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -32432;
	// 826DECD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DECDC: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826DECE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DECE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DECE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DECF0: 386A0274  addi r3, r10, 0x274
	ctx.r[3].s64 = ctx.r[10].s64 + 628;
	// 826DECF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DECF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DECFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DED00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DED04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DED08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DED0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DED10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DED14: 4BD8810D  bl 0x82466e20
	ctx.lr = 0x826DED18;
	sub_82466E20(ctx, base);
	// 826DED18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DED28 size=108
    let mut pc: u32 = 0x826DED28;
    'dispatch: loop {
        match pc {
            0x826DED28 => {
    //   block [0x826DED28..0x826DED94)
	// 826DED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DED30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DED34: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DED38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DED3C: 38EB8168  addi r7, r11, -0x7e98
	ctx.r[7].s64 = ctx.r[11].s64 + -32408;
	// 826DED40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DED44: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826DED48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DED4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DED50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DED54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DED58: 386A02A4  addi r3, r10, 0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + 676;
	// 826DED5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DED60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DED68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DED6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DED70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DED74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DED78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DED7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DED80: 4BD880A1  bl 0x82466e20
	ctx.lr = 0x826DED84;
	sub_82466E20(ctx, base);
	// 826DED84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DED88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DED8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DED90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DED98 size=112
    let mut pc: u32 = 0x826DED98;
    'dispatch: loop {
        match pc {
            0x826DED98 => {
    //   block [0x826DED98..0x826DEE08)
	// 826DED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEDA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEDA8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEDAC: 38AA02A4  addi r5, r10, 0x2a4
	ctx.r[5].s64 = ctx.r[10].s64 + 676;
	// 826DEDB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEDB4: 390B8198  addi r8, r11, -0x7e68
	ctx.r[8].s64 = ctx.r[11].s64 + -32360;
	// 826DEDB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DEDBC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826DEDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEDC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEDCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEDD0: 386A02D4  addi r3, r10, 0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + 724;
	// 826DEDD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DEDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEDDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEDF4: 4BD8802D  bl 0x82466e20
	ctx.lr = 0x826DEDF8;
	sub_82466E20(ctx, base);
	// 826DEDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DEE08 size=24
    let mut pc: u32 = 0x826DEE08;
    'dispatch: loop {
        match pc {
            0x826DEE08 => {
    //   block [0x826DEE08..0x826DEE20)
	// 826DEE08: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DEE0C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DEE10: 394ABF60  addi r10, r10, -0x40a0
	ctx.r[10].s64 = ctx.r[10].s64 + -16544;
	// 826DEE14: 816B79CC  lwz r11, 0x79cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31180 as u32) ) } as u64;
	// 826DEE18: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826DEE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEE20 size=116
    let mut pc: u32 = 0x826DEE20;
    'dispatch: loop {
        match pc {
            0x826DEE20 => {
    //   block [0x826DEE20..0x826DEE94)
	// 826DEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEE2C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEE30: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DEE34: 390BBF60  addi r8, r11, -0x40a0
	ctx.r[8].s64 = ctx.r[11].s64 + -16544;
	// 826DEE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEE3C: 392A60B8  addi r9, r10, 0x60b8
	ctx.r[9].s64 = ctx.r[10].s64 + 24760;
	// 826DEE40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEE44: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826DEE48: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DEE4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DEE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEE54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEE64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DEE68: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826DEE6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DEE70: 386B0304  addi r3, r11, 0x304
	ctx.r[3].s64 = ctx.r[11].s64 + 772;
	// 826DEE74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DEE78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEE80: 4BD87FA1  bl 0x82466e20
	ctx.lr = 0x826DEE84;
	sub_82466E20(ctx, base);
	// 826DEE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEE98 size=108
    let mut pc: u32 = 0x826DEE98;
    'dispatch: loop {
        match pc {
            0x826DEE98 => {
    //   block [0x826DEE98..0x826DEF04)
	// 826DEE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEEA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEEA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEEAC: 38EB81C8  addi r7, r11, -0x7e38
	ctx.r[7].s64 = ctx.r[11].s64 + -32312;
	// 826DEEB0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DEEB4: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826DEEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEEBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DEEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEEC8: 386A0334  addi r3, r10, 0x334
	ctx.r[3].s64 = ctx.r[10].s64 + 820;
	// 826DEECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DEED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEEF0: 4BD87F31  bl 0x82466e20
	ctx.lr = 0x826DEEF4;
	sub_82466E20(ctx, base);
	// 826DEEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEF08 size=108
    let mut pc: u32 = 0x826DEF08;
    'dispatch: loop {
        match pc {
            0x826DEF08 => {
    //   block [0x826DEF08..0x826DEF74)
	// 826DEF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEF14: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEF18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEF1C: 38EB8210  addi r7, r11, -0x7df0
	ctx.r[7].s64 = ctx.r[11].s64 + -32240;
	// 826DEF20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DEF24: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826DEF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEF2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DEF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEF38: 386A0364  addi r3, r10, 0x364
	ctx.r[3].s64 = ctx.r[10].s64 + 868;
	// 826DEF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DEF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEF60: 4BD87EC1  bl 0x82466e20
	ctx.lr = 0x826DEF64;
	sub_82466E20(ctx, base);
	// 826DEF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEF78 size=108
    let mut pc: u32 = 0x826DEF78;
    'dispatch: loop {
        match pc {
            0x826DEF78 => {
    //   block [0x826DEF78..0x826DEFE4)
	// 826DEF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEF84: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEF88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DEF8C: 38EB8240  addi r7, r11, -0x7dc0
	ctx.r[7].s64 = ctx.r[11].s64 + -32192;
	// 826DEF90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DEF94: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826DEF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DEF9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEFA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DEFA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DEFA8: 386A0394  addi r3, r10, 0x394
	ctx.r[3].s64 = ctx.r[10].s64 + 916;
	// 826DEFAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DEFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DEFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DEFB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DEFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DEFC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DEFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DEFC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DEFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DEFD0: 4BD87E51  bl 0x82466e20
	ctx.lr = 0x826DEFD4;
	sub_82466E20(ctx, base);
	// 826DEFD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DEFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DEFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DEFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DEFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DEFE8 size=112
    let mut pc: u32 = 0x826DEFE8;
    'dispatch: loop {
        match pc {
            0x826DEFE8 => {
    //   block [0x826DEFE8..0x826DF058)
	// 826DEFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DEFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DEFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DEFF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DEFF8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DEFFC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF000: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF004: 390B8270  addi r8, r11, -0x7d90
	ctx.r[8].s64 = ctx.r[11].s64 + -32144;
	// 826DF008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DF00C: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826DF010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF014: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF020: 386A03C4  addi r3, r10, 0x3c4
	ctx.r[3].s64 = ctx.r[10].s64 + 964;
	// 826DF024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF044: 4BD87DDD  bl 0x82466e20
	ctx.lr = 0x826DF048;
	sub_82466E20(ctx, base);
	// 826DF048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF058 size=112
    let mut pc: u32 = 0x826DF058;
    'dispatch: loop {
        match pc {
            0x826DF058 => {
    //   block [0x826DF058..0x826DF0C8)
	// 826DF058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF068: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF06C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF074: 390B82A0  addi r8, r11, -0x7d60
	ctx.r[8].s64 = ctx.r[11].s64 + -32096;
	// 826DF078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DF07C: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826DF080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF090: 386A03F4  addi r3, r10, 0x3f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1012;
	// 826DF094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF0B4: 4BD87D6D  bl 0x82466e20
	ctx.lr = 0x826DF0B8;
	sub_82466E20(ctx, base);
	// 826DF0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF0C8 size=112
    let mut pc: u32 = 0x826DF0C8;
    'dispatch: loop {
        match pc {
            0x826DF0C8 => {
    //   block [0x826DF0C8..0x826DF138)
	// 826DF0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF0D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF0DC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF0E4: 390B82B8  addi r8, r11, -0x7d48
	ctx.r[8].s64 = ctx.r[11].s64 + -32072;
	// 826DF0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DF0EC: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826DF0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF100: 386A0424  addi r3, r10, 0x424
	ctx.r[3].s64 = ctx.r[10].s64 + 1060;
	// 826DF104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF124: 4BD87CFD  bl 0x82466e20
	ctx.lr = 0x826DF128;
	sub_82466E20(ctx, base);
	// 826DF128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF138 size=108
    let mut pc: u32 = 0x826DF138;
    'dispatch: loop {
        match pc {
            0x826DF138 => {
    //   block [0x826DF138..0x826DF1A4)
	// 826DF138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF144: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF14C: 38EB82D0  addi r7, r11, -0x7d30
	ctx.r[7].s64 = ctx.r[11].s64 + -32048;
	// 826DF150: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DF154: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826DF158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF15C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF168: 386A0454  addi r3, r10, 0x454
	ctx.r[3].s64 = ctx.r[10].s64 + 1108;
	// 826DF16C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF18C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF190: 4BD87C91  bl 0x82466e20
	ctx.lr = 0x826DF194;
	sub_82466E20(ctx, base);
	// 826DF194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF1A8 size=112
    let mut pc: u32 = 0x826DF1A8;
    'dispatch: loop {
        match pc {
            0x826DF1A8 => {
    //   block [0x826DF1A8..0x826DF218)
	// 826DF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF1B8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF1BC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF1C4: 390B8300  addi r8, r11, -0x7d00
	ctx.r[8].s64 = ctx.r[11].s64 + -32000;
	// 826DF1C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DF1CC: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826DF1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF1E0: 386A0484  addi r3, r10, 0x484
	ctx.r[3].s64 = ctx.r[10].s64 + 1156;
	// 826DF1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF204: 4BD87C1D  bl 0x82466e20
	ctx.lr = 0x826DF208;
	sub_82466E20(ctx, base);
	// 826DF208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF218 size=108
    let mut pc: u32 = 0x826DF218;
    'dispatch: loop {
        match pc {
            0x826DF218 => {
    //   block [0x826DF218..0x826DF284)
	// 826DF218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF224: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF228: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF22C: 38EB8318  addi r7, r11, -0x7ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -31976;
	// 826DF230: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826DF234: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826DF238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF23C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF248: 386A04B4  addi r3, r10, 0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1204;
	// 826DF24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF270: 4BD87BB1  bl 0x82466e20
	ctx.lr = 0x826DF274;
	sub_82466E20(ctx, base);
	// 826DF274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF288 size=116
    let mut pc: u32 = 0x826DF288;
    'dispatch: loop {
        match pc {
            0x826DF288 => {
    //   block [0x826DF288..0x826DF2FC)
	// 826DF288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF294: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DF298: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826DF29C: 390A8408  addi r8, r10, -0x7bf8
	ctx.r[8].s64 = ctx.r[10].s64 + -31736;
	// 826DF2A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF2A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DF2A8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF2AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF2B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DF2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF2BC: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826DF2C0: 396B60D0  addi r11, r11, 0x60d0
	ctx.r[11].s64 = ctx.r[11].s64 + 24784;
	// 826DF2C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF2C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF2CC: 386A04E4  addi r3, r10, 0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1252;
	// 826DF2D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DF2D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF2D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DF2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF2E8: 4BD87B39  bl 0x82466e20
	ctx.lr = 0x826DF2EC;
	sub_82466E20(ctx, base);
	// 826DF2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF300 size=108
    let mut pc: u32 = 0x826DF300;
    'dispatch: loop {
        match pc {
            0x826DF300 => {
    //   block [0x826DF300..0x826DF36C)
	// 826DF300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF30C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF314: 38EB85D0  addi r7, r11, -0x7a30
	ctx.r[7].s64 = ctx.r[11].s64 + -31280;
	// 826DF318: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826DF31C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826DF320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF330: 386A0514  addi r3, r10, 0x514
	ctx.r[3].s64 = ctx.r[10].s64 + 1300;
	// 826DF334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF358: 4BD87AC9  bl 0x82466e20
	ctx.lr = 0x826DF35C;
	sub_82466E20(ctx, base);
	// 826DF35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF370 size=112
    let mut pc: u32 = 0x826DF370;
    'dispatch: loop {
        match pc {
            0x826DF370 => {
    //   block [0x826DF370..0x826DF3E0)
	// 826DF370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF37C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF380: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF384: 38AAEE64  addi r5, r10, -0x119c
	ctx.r[5].s64 = ctx.r[10].s64 + -4508;
	// 826DF388: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF38C: 390B8768  addi r8, r11, -0x7898
	ctx.r[8].s64 = ctx.r[11].s64 + -30872;
	// 826DF390: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826DF394: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826DF398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF3A8: 386A0544  addi r3, r10, 0x544
	ctx.r[3].s64 = ctx.r[10].s64 + 1348;
	// 826DF3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF3CC: 4BD87A55  bl 0x82466e20
	ctx.lr = 0x826DF3D0;
	sub_82466E20(ctx, base);
	// 826DF3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF3E0 size=100
    let mut pc: u32 = 0x826DF3E0;
    'dispatch: loop {
        match pc {
            0x826DF3E0 => {
    //   block [0x826DF3E0..0x826DF444)
	// 826DF3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF3EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF3F4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF3F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF400: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826DF404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF414: 386A0574  addi r3, r10, 0x574
	ctx.r[3].s64 = ctx.r[10].s64 + 1396;
	// 826DF418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF41C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DF424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DF42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF430: 4BD879F1  bl 0x82466e20
	ctx.lr = 0x826DF434;
	sub_82466E20(ctx, base);
	// 826DF434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF448 size=112
    let mut pc: u32 = 0x826DF448;
    'dispatch: loop {
        match pc {
            0x826DF448 => {
    //   block [0x826DF448..0x826DF4B8)
	// 826DF448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF454: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF458: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF45C: 38AA0574  addi r5, r10, 0x574
	ctx.r[5].s64 = ctx.r[10].s64 + 1396;
	// 826DF460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF464: 390B89C0  addi r8, r11, -0x7640
	ctx.r[8].s64 = ctx.r[11].s64 + -30272;
	// 826DF468: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DF46C: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826DF470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF480: 386A05A4  addi r3, r10, 0x5a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1444;
	// 826DF484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF4A4: 4BD8797D  bl 0x82466e20
	ctx.lr = 0x826DF4A8;
	sub_82466E20(ctx, base);
	// 826DF4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF4B8 size=100
    let mut pc: u32 = 0x826DF4B8;
    'dispatch: loop {
        match pc {
            0x826DF4B8 => {
    //   block [0x826DF4B8..0x826DF51C)
	// 826DF4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF4C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF4CC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF4D8: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826DF4DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF4EC: 386A05D4  addi r3, r10, 0x5d4
	ctx.r[3].s64 = ctx.r[10].s64 + 1492;
	// 826DF4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF4F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF4F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DF4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF500: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DF504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF508: 4BD87919  bl 0x82466e20
	ctx.lr = 0x826DF50C;
	sub_82466E20(ctx, base);
	// 826DF50C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF520 size=108
    let mut pc: u32 = 0x826DF520;
    'dispatch: loop {
        match pc {
            0x826DF520 => {
    //   block [0x826DF520..0x826DF58C)
	// 826DF520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF52C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF534: 38EB8A38  addi r7, r11, -0x75c8
	ctx.r[7].s64 = ctx.r[11].s64 + -30152;
	// 826DF538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DF53C: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826DF540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF550: 386A0604  addi r3, r10, 0x604
	ctx.r[3].s64 = ctx.r[10].s64 + 1540;
	// 826DF554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF578: 4BD878A9  bl 0x82466e20
	ctx.lr = 0x826DF57C;
	sub_82466E20(ctx, base);
	// 826DF57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF590 size=112
    let mut pc: u32 = 0x826DF590;
    'dispatch: loop {
        match pc {
            0x826DF590 => {
    //   block [0x826DF590..0x826DF600)
	// 826DF590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF59C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF5A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF5A4: 38AA05D4  addi r5, r10, 0x5d4
	ctx.r[5].s64 = ctx.r[10].s64 + 1492;
	// 826DF5A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF5AC: 390B8A80  addi r8, r11, -0x7580
	ctx.r[8].s64 = ctx.r[11].s64 + -30080;
	// 826DF5B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DF5B4: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826DF5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF5BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF5C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF5C8: 386A0634  addi r3, r10, 0x634
	ctx.r[3].s64 = ctx.r[10].s64 + 1588;
	// 826DF5CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF5D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF5DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF5EC: 4BD87835  bl 0x82466e20
	ctx.lr = 0x826DF5F0;
	sub_82466E20(ctx, base);
	// 826DF5F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF600 size=100
    let mut pc: u32 = 0x826DF600;
    'dispatch: loop {
        match pc {
            0x826DF600 => {
    //   block [0x826DF600..0x826DF664)
	// 826DF600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF60C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF614: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF620: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826DF624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF634: 386A0664  addi r3, r10, 0x664
	ctx.r[3].s64 = ctx.r[10].s64 + 1636;
	// 826DF638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF63C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF640: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DF644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF648: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DF64C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF650: 4BD877D1  bl 0x82466e20
	ctx.lr = 0x826DF654;
	sub_82466E20(ctx, base);
	// 826DF654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF668 size=100
    let mut pc: u32 = 0x826DF668;
    'dispatch: loop {
        match pc {
            0x826DF668 => {
    //   block [0x826DF668..0x826DF6CC)
	// 826DF668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF674: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF67C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF680: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF688: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826DF68C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF69C: 386A0694  addi r3, r10, 0x694
	ctx.r[3].s64 = ctx.r[10].s64 + 1684;
	// 826DF6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF6A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF6A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DF6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF6B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DF6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF6B8: 4BD87769  bl 0x82466e20
	ctx.lr = 0x826DF6BC;
	sub_82466E20(ctx, base);
	// 826DF6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF6D0 size=112
    let mut pc: u32 = 0x826DF6D0;
    'dispatch: loop {
        match pc {
            0x826DF6D0 => {
    //   block [0x826DF6D0..0x826DF740)
	// 826DF6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF6DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF6E0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF6E4: 38AA0664  addi r5, r10, 0x664
	ctx.r[5].s64 = ctx.r[10].s64 + 1636;
	// 826DF6E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF6EC: 390B8AB0  addi r8, r11, -0x7550
	ctx.r[8].s64 = ctx.r[11].s64 + -30032;
	// 826DF6F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DF6F4: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826DF6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF6FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF708: 386A06C4  addi r3, r10, 0x6c4
	ctx.r[3].s64 = ctx.r[10].s64 + 1732;
	// 826DF70C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF72C: 4BD876F5  bl 0x82466e20
	ctx.lr = 0x826DF730;
	sub_82466E20(ctx, base);
	// 826DF730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF740 size=112
    let mut pc: u32 = 0x826DF740;
    'dispatch: loop {
        match pc {
            0x826DF740 => {
    //   block [0x826DF740..0x826DF7B0)
	// 826DF740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF74C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF750: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF754: 38AA0694  addi r5, r10, 0x694
	ctx.r[5].s64 = ctx.r[10].s64 + 1684;
	// 826DF758: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF75C: 390B8B10  addi r8, r11, -0x74f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29936;
	// 826DF760: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DF764: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826DF768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF76C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF778: 386A06F4  addi r3, r10, 0x6f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1780;
	// 826DF77C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF79C: 4BD87685  bl 0x82466e20
	ctx.lr = 0x826DF7A0;
	sub_82466E20(ctx, base);
	// 826DF7A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF7B0 size=100
    let mut pc: u32 = 0x826DF7B0;
    'dispatch: loop {
        match pc {
            0x826DF7B0 => {
    //   block [0x826DF7B0..0x826DF814)
	// 826DF7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF7BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF7C4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DF7C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF7D0: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826DF7D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF7E4: 386A0724  addi r3, r10, 0x724
	ctx.r[3].s64 = ctx.r[10].s64 + 1828;
	// 826DF7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF7EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF7F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DF7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF7F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DF7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF800: 4BD87621  bl 0x82466e20
	ctx.lr = 0x826DF804;
	sub_82466E20(ctx, base);
	// 826DF804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF818 size=112
    let mut pc: u32 = 0x826DF818;
    'dispatch: loop {
        match pc {
            0x826DF818 => {
    //   block [0x826DF818..0x826DF888)
	// 826DF818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF824: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF828: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF82C: 38AA0724  addi r5, r10, 0x724
	ctx.r[5].s64 = ctx.r[10].s64 + 1828;
	// 826DF830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF834: 390B8B70  addi r8, r11, -0x7490
	ctx.r[8].s64 = ctx.r[11].s64 + -29840;
	// 826DF838: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826DF83C: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826DF840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF844: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DF84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF850: 386A0754  addi r3, r10, 0x754
	ctx.r[3].s64 = ctx.r[10].s64 + 1876;
	// 826DF854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DF858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF85C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF874: 4BD875AD  bl 0x82466e20
	ctx.lr = 0x826DF878;
	sub_82466E20(ctx, base);
	// 826DF878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF888 size=108
    let mut pc: u32 = 0x826DF888;
    'dispatch: loop {
        match pc {
            0x826DF888 => {
    //   block [0x826DF888..0x826DF8F4)
	// 826DF888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF894: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF898: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF89C: 38EB8C60  addi r7, r11, -0x73a0
	ctx.r[7].s64 = ctx.r[11].s64 + -29600;
	// 826DF8A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826DF8A4: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826DF8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF8AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF8B8: 386A0784  addi r3, r10, 0x784
	ctx.r[3].s64 = ctx.r[10].s64 + 1924;
	// 826DF8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF8E0: 4BD87541  bl 0x82466e20
	ctx.lr = 0x826DF8E4;
	sub_82466E20(ctx, base);
	// 826DF8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF8F8 size=108
    let mut pc: u32 = 0x826DF8F8;
    'dispatch: loop {
        match pc {
            0x826DF8F8 => {
    //   block [0x826DF8F8..0x826DF964)
	// 826DF8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF904: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF90C: 38EB8D50  addi r7, r11, -0x72b0
	ctx.r[7].s64 = ctx.r[11].s64 + -29360;
	// 826DF910: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DF914: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826DF918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF91C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF928: 386A07B4  addi r3, r10, 0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1972;
	// 826DF92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF950: 4BD874D1  bl 0x82466e20
	ctx.lr = 0x826DF954;
	sub_82466E20(ctx, base);
	// 826DF954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF968 size=108
    let mut pc: u32 = 0x826DF968;
    'dispatch: loop {
        match pc {
            0x826DF968 => {
    //   block [0x826DF968..0x826DF9D4)
	// 826DF968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF974: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF97C: 38EB8D98  addi r7, r11, -0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + -29288;
	// 826DF980: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826DF984: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826DF988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF98C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DF990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DF994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DF998: 386A07E4  addi r3, r10, 0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2020;
	// 826DF99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DF9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DF9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DF9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DF9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DF9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DF9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DF9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DF9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DF9C0: 4BD87461  bl 0x82466e20
	ctx.lr = 0x826DF9C4;
	sub_82466E20(ctx, base);
	// 826DF9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DF9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DF9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DF9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DF9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DF9D8 size=108
    let mut pc: u32 = 0x826DF9D8;
    'dispatch: loop {
        match pc {
            0x826DF9D8 => {
    //   block [0x826DF9D8..0x826DFA44)
	// 826DF9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DF9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DF9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DF9E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DF9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DF9EC: 38EB8E70  addi r7, r11, -0x7190
	ctx.r[7].s64 = ctx.r[11].s64 + -29072;
	// 826DF9F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DF9F4: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826DF9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DF9FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFA00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DFA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFA08: 386A0814  addi r3, r10, 0x814
	ctx.r[3].s64 = ctx.r[10].s64 + 2068;
	// 826DFA0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DFA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DFA30: 4BD873F1  bl 0x82466e20
	ctx.lr = 0x826DFA34;
	sub_82466E20(ctx, base);
	// 826DFA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFA48 size=100
    let mut pc: u32 = 0x826DFA48;
    'dispatch: loop {
        match pc {
            0x826DFA48 => {
    //   block [0x826DFA48..0x826DFAAC)
	// 826DFA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFA54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFA5C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFA60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFA68: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826DFA6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFA7C: 386A0844  addi r3, r10, 0x844
	ctx.r[3].s64 = ctx.r[10].s64 + 2116;
	// 826DFA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFA88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DFA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFA90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DFA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFA98: 4BD87389  bl 0x82466e20
	ctx.lr = 0x826DFA9C;
	sub_82466E20(ctx, base);
	// 826DFA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFAB0 size=112
    let mut pc: u32 = 0x826DFAB0;
    'dispatch: loop {
        match pc {
            0x826DFAB0 => {
    //   block [0x826DFAB0..0x826DFB20)
	// 826DFAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFAC0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFAC4: 38AA0844  addi r5, r10, 0x844
	ctx.r[5].s64 = ctx.r[10].s64 + 2116;
	// 826DFAC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFACC: 390B8E88  addi r8, r11, -0x7178
	ctx.r[8].s64 = ctx.r[11].s64 + -29048;
	// 826DFAD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DFAD4: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826DFAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFAE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFAE8: 386A0874  addi r3, r10, 0x874
	ctx.r[3].s64 = ctx.r[10].s64 + 2164;
	// 826DFAEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFB0C: 4BD87315  bl 0x82466e20
	ctx.lr = 0x826DFB10;
	sub_82466E20(ctx, base);
	// 826DFB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFB20 size=108
    let mut pc: u32 = 0x826DFB20;
    'dispatch: loop {
        match pc {
            0x826DFB20 => {
    //   block [0x826DFB20..0x826DFB8C)
	// 826DFB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFB2C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFB34: 38EB8ED0  addi r7, r11, -0x7130
	ctx.r[7].s64 = ctx.r[11].s64 + -28976;
	// 826DFB38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DFB3C: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826DFB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFB44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DFB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFB50: 386A08A4  addi r3, r10, 0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + 2212;
	// 826DFB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DFB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DFB78: 4BD872A9  bl 0x82466e20
	ctx.lr = 0x826DFB7C;
	sub_82466E20(ctx, base);
	// 826DFB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFB90 size=112
    let mut pc: u32 = 0x826DFB90;
    'dispatch: loop {
        match pc {
            0x826DFB90 => {
    //   block [0x826DFB90..0x826DFC00)
	// 826DFB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFB9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFBA0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFBA4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFBA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFBAC: 390B8F18  addi r8, r11, -0x70e8
	ctx.r[8].s64 = ctx.r[11].s64 + -28904;
	// 826DFBB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DFBB4: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826DFBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFBBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFBC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFBC8: 386A08D4  addi r3, r10, 0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2260;
	// 826DFBCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFBD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFBD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFBEC: 4BD87235  bl 0x82466e20
	ctx.lr = 0x826DFBF0;
	sub_82466E20(ctx, base);
	// 826DFBF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFC00 size=108
    let mut pc: u32 = 0x826DFC00;
    'dispatch: loop {
        match pc {
            0x826DFC00 => {
    //   block [0x826DFC00..0x826DFC6C)
	// 826DFC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFC0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFC14: 38EB8F30  addi r7, r11, -0x70d0
	ctx.r[7].s64 = ctx.r[11].s64 + -28880;
	// 826DFC18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DFC1C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826DFC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFC24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DFC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFC30: 386A0904  addi r3, r10, 0x904
	ctx.r[3].s64 = ctx.r[10].s64 + 2308;
	// 826DFC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DFC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DFC58: 4BD871C9  bl 0x82466e20
	ctx.lr = 0x826DFC5C;
	sub_82466E20(ctx, base);
	// 826DFC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFC70 size=112
    let mut pc: u32 = 0x826DFC70;
    'dispatch: loop {
        match pc {
            0x826DFC70 => {
    //   block [0x826DFC70..0x826DFCE0)
	// 826DFC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFC7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFC80: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFC84: 38AA08D4  addi r5, r10, 0x8d4
	ctx.r[5].s64 = ctx.r[10].s64 + 2260;
	// 826DFC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFC8C: 390B8F78  addi r8, r11, -0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + -28808;
	// 826DFC90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DFC94: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826DFC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFCA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFCA8: 386A0934  addi r3, r10, 0x934
	ctx.r[3].s64 = ctx.r[10].s64 + 2356;
	// 826DFCAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFCB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFCCC: 4BD87155  bl 0x82466e20
	ctx.lr = 0x826DFCD0;
	sub_82466E20(ctx, base);
	// 826DFCD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFCE0 size=100
    let mut pc: u32 = 0x826DFCE0;
    'dispatch: loop {
        match pc {
            0x826DFCE0 => {
    //   block [0x826DFCE0..0x826DFD44)
	// 826DFCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFCEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFCF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFCF4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFCF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFD00: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826DFD04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFD14: 386A0964  addi r3, r10, 0x964
	ctx.r[3].s64 = ctx.r[10].s64 + 2404;
	// 826DFD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFD1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFD20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DFD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFD28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DFD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFD30: 4BD870F1  bl 0x82466e20
	ctx.lr = 0x826DFD34;
	sub_82466E20(ctx, base);
	// 826DFD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFD48 size=112
    let mut pc: u32 = 0x826DFD48;
    'dispatch: loop {
        match pc {
            0x826DFD48 => {
    //   block [0x826DFD48..0x826DFDB8)
	// 826DFD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFD54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFD58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFD5C: 38AA0964  addi r5, r10, 0x964
	ctx.r[5].s64 = ctx.r[10].s64 + 2404;
	// 826DFD60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFD64: 390B8F90  addi r8, r11, -0x7070
	ctx.r[8].s64 = ctx.r[11].s64 + -28784;
	// 826DFD68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DFD6C: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826DFD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFD74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFD78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFD80: 386A0994  addi r3, r10, 0x994
	ctx.r[3].s64 = ctx.r[10].s64 + 2452;
	// 826DFD84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFDA4: 4BD8707D  bl 0x82466e20
	ctx.lr = 0x826DFDA8;
	sub_82466E20(ctx, base);
	// 826DFDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFDB8 size=108
    let mut pc: u32 = 0x826DFDB8;
    'dispatch: loop {
        match pc {
            0x826DFDB8 => {
    //   block [0x826DFDB8..0x826DFE24)
	// 826DFDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFDC4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFDCC: 38EB9038  addi r7, r11, -0x6fc8
	ctx.r[7].s64 = ctx.r[11].s64 + -28616;
	// 826DFDD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DFDD4: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826DFDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFDDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DFDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFDE8: 386A09C4  addi r3, r10, 0x9c4
	ctx.r[3].s64 = ctx.r[10].s64 + 2500;
	// 826DFDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DFDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DFE10: 4BD87011  bl 0x82466e20
	ctx.lr = 0x826DFE14;
	sub_82466E20(ctx, base);
	// 826DFE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFE28 size=112
    let mut pc: u32 = 0x826DFE28;
    'dispatch: loop {
        match pc {
            0x826DFE28 => {
    //   block [0x826DFE28..0x826DFE98)
	// 826DFE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFE38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFE3C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFE40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFE44: 390B9068  addi r8, r11, -0x6f98
	ctx.r[8].s64 = ctx.r[11].s64 + -28568;
	// 826DFE48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DFE4C: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826DFE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFE54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFE60: 386A09F4  addi r3, r10, 0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2548;
	// 826DFE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFE84: 4BD86F9D  bl 0x82466e20
	ctx.lr = 0x826DFE88;
	sub_82466E20(ctx, base);
	// 826DFE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFE98 size=112
    let mut pc: u32 = 0x826DFE98;
    'dispatch: loop {
        match pc {
            0x826DFE98 => {
    //   block [0x826DFE98..0x826DFF08)
	// 826DFE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFEA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFEA8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFEAC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826DFEB4: 390B90B0  addi r8, r11, -0x6f50
	ctx.r[8].s64 = ctx.r[11].s64 + -28496;
	// 826DFEB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DFEBC: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826DFEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFEC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFED0: 386A0A24  addi r3, r10, 0xa24
	ctx.r[3].s64 = ctx.r[10].s64 + 2596;
	// 826DFED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFEF4: 4BD86F2D  bl 0x82466e20
	ctx.lr = 0x826DFEF8;
	sub_82466E20(ctx, base);
	// 826DFEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFF08 size=100
    let mut pc: u32 = 0x826DFF08;
    'dispatch: loop {
        match pc {
            0x826DFF08 => {
    //   block [0x826DFF08..0x826DFF6C)
	// 826DFF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFF1C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFF20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFF28: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826DFF2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFF3C: 386A0A54  addi r3, r10, 0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + 2644;
	// 826DFF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFF44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFF48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DFF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFF50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DFF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFF58: 4BD86EC9  bl 0x82466e20
	ctx.lr = 0x826DFF5C;
	sub_82466E20(ctx, base);
	// 826DFF5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFF70 size=112
    let mut pc: u32 = 0x826DFF70;
    'dispatch: loop {
        match pc {
            0x826DFF70 => {
    //   block [0x826DFF70..0x826DFFE0)
	// 826DFF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFF7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFF80: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFF84: 38AA0A54  addi r5, r10, 0xa54
	ctx.r[5].s64 = ctx.r[10].s64 + 2644;
	// 826DFF88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFF8C: 390B90F8  addi r8, r11, -0x6f08
	ctx.r[8].s64 = ctx.r[11].s64 + -28424;
	// 826DFF90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DFF94: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826DFF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DFF9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFFA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DFFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DFFA8: 386A0A84  addi r3, r10, 0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + 2692;
	// 826DFFAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DFFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DFFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DFFB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DFFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DFFC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DFFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DFFC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DFFCC: 4BD86E55  bl 0x82466e20
	ctx.lr = 0x826DFFD0;
	sub_82466E20(ctx, base);
	// 826DFFD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DFFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DFFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DFFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DFFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DFFE0 size=112
    let mut pc: u32 = 0x826DFFE0;
    'dispatch: loop {
        match pc {
            0x826DFFE0 => {
    //   block [0x826DFFE0..0x826E0050)
	// 826DFFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DFFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DFFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DFFEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DFFF0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DFFF4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DFFF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DFFFC: 390B9140  addi r8, r11, -0x6ec0
	ctx.r[8].s64 = ctx.r[11].s64 + -28352;
	// 826E0000: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E0004: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826E0008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E000C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0018: 386A0AB4  addi r3, r10, 0xab4
	ctx.r[3].s64 = ctx.r[10].s64 + 2740;
	// 826E001C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E002C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E003C: 4BD86DE5  bl 0x82466e20
	ctx.lr = 0x826E0040;
	sub_82466E20(ctx, base);
	// 826E0040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E004C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0050 size=112
    let mut pc: u32 = 0x826E0050;
    'dispatch: loop {
        match pc {
            0x826E0050 => {
    //   block [0x826E0050..0x826E00C0)
	// 826E0050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E005C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0060: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0064: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826E0068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E006C: 390B9158  addi r8, r11, -0x6ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -28328;
	// 826E0070: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E0074: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826E0078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E007C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0088: 386A0AE4  addi r3, r10, 0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + 2788;
	// 826E008C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E009C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E00A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E00A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E00A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E00AC: 4BD86D75  bl 0x82466e20
	ctx.lr = 0x826E00B0;
	sub_82466E20(ctx, base);
	// 826E00B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E00B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E00B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E00BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E00C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E00C0 size=112
    let mut pc: u32 = 0x826E00C0;
    'dispatch: loop {
        match pc {
            0x826E00C0 => {
    //   block [0x826E00C0..0x826E0130)
	// 826E00C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E00C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E00C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E00CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E00D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E00D4: 38AA0AB4  addi r5, r10, 0xab4
	ctx.r[5].s64 = ctx.r[10].s64 + 2740;
	// 826E00D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E00DC: 390B9170  addi r8, r11, -0x6e90
	ctx.r[8].s64 = ctx.r[11].s64 + -28304;
	// 826E00E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E00E4: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826E00E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E00EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E00F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E00F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E00F8: 386A0B14  addi r3, r10, 0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + 2836;
	// 826E00FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E010C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E011C: 4BD86D05  bl 0x82466e20
	ctx.lr = 0x826E0120;
	sub_82466E20(ctx, base);
	// 826E0120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E012C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0130 size=72
    let mut pc: u32 = 0x826E0130;
    'dispatch: loop {
        match pc {
            0x826E0130 => {
    //   block [0x826E0130..0x826E0178)
	// 826E0130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E013C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E0140: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826E0144: 38CB40B8  addi r6, r11, 0x40b8
	ctx.r[6].s64 = ctx.r[11].s64 + 16568;
	// 826E0148: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E014C: 388B6120  addi r4, r11, 0x6120
	ctx.r[4].s64 = ctx.r[11].s64 + 24864;
	// 826E0150: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E0154: 386B0B44  addi r3, r11, 0xb44
	ctx.r[3].s64 = ctx.r[11].s64 + 2884;
	// 826E0158: 4BD9B931  bl 0x8247ba88
	ctx.lr = 0x826E015C;
	sub_8247BA88(ctx, base);
	// 826E015C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826E0160: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 826E0164: 4BE529D5  bl 0x82532b38
	ctx.lr = 0x826E0168;
	sub_82532B38(ctx, base);
	// 826E0168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826E016C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0178 size=108
    let mut pc: u32 = 0x826E0178;
    'dispatch: loop {
        match pc {
            0x826E0178 => {
    //   block [0x826E0178..0x826E01E4)
	// 826E0178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E017C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0184: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E018C: 38EBC098  addi r7, r11, -0x3f68
	ctx.r[7].s64 = ctx.r[11].s64 + -16232;
	// 826E0190: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E0194: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 826E0198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E019C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E01A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E01A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E01A8: 386A0B5C  addi r3, r10, 0xb5c
	ctx.r[3].s64 = ctx.r[10].s64 + 2908;
	// 826E01AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E01B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E01B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E01B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E01BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E01C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E01C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E01C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E01CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E01D0: 4BD86C51  bl 0x82466e20
	ctx.lr = 0x826E01D4;
	sub_82466E20(ctx, base);
	// 826E01D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E01D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E01DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E01E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E01E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E01E8 size=24
    let mut pc: u32 = 0x826E01E8;
    'dispatch: loop {
        match pc {
            0x826E01E8 => {
    //   block [0x826E01E8..0x826E0200)
	// 826E01E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E01EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E01F0: 394A58C8  addi r10, r10, 0x58c8
	ctx.r[10].s64 = ctx.r[10].s64 + 22728;
	// 826E01F4: 816BC110  lwz r11, -0x3ef0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16112 as u32) ) } as u64;
	// 826E01F8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826E01FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0200 size=112
    let mut pc: u32 = 0x826E0200;
    'dispatch: loop {
        match pc {
            0x826E0200 => {
    //   block [0x826E0200..0x826E0270)
	// 826E0200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E020C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0210: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0214: 392A68EC  addi r9, r10, 0x68ec
	ctx.r[9].s64 = ctx.r[10].s64 + 26860;
	// 826E0218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E021C: 390B58C8  addi r8, r11, 0x58c8
	ctx.r[8].s64 = ctx.r[11].s64 + 22728;
	// 826E0220: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E0224: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 826E0228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E022C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0238: 386A0B8C  addi r3, r10, 0xb8c
	ctx.r[3].s64 = ctx.r[10].s64 + 2956;
	// 826E023C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E0240: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E024C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E025C: 4BD86BC5  bl 0x82466e20
	ctx.lr = 0x826E0260;
	sub_82466E20(ctx, base);
	// 826E0260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E026C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0270 size=108
    let mut pc: u32 = 0x826E0270;
    'dispatch: loop {
        match pc {
            0x826E0270 => {
    //   block [0x826E0270..0x826E02DC)
	// 826E0270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E027C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0280: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0284: 38EBC114  addi r7, r11, -0x3eec
	ctx.r[7].s64 = ctx.r[11].s64 + -16108;
	// 826E0288: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E028C: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 826E0290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0298: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E029C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E02A0: 386A0BBC  addi r3, r10, 0xbbc
	ctx.r[3].s64 = ctx.r[10].s64 + 3004;
	// 826E02A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E02A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E02AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E02B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E02B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E02B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E02BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E02C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E02C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E02C8: 4BD86B59  bl 0x82466e20
	ctx.lr = 0x826E02CC;
	sub_82466E20(ctx, base);
	// 826E02CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E02D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E02D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E02D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E02E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E02E0 size=108
    let mut pc: u32 = 0x826E02E0;
    'dispatch: loop {
        match pc {
            0x826E02E0 => {
    //   block [0x826E02E0..0x826E034C)
	// 826E02E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E02E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E02E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E02EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E02F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E02F4: 38EBC144  addi r7, r11, -0x3ebc
	ctx.r[7].s64 = ctx.r[11].s64 + -16060;
	// 826E02F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E02FC: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 826E0300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E030C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0310: 386A0BEC  addi r3, r10, 0xbec
	ctx.r[3].s64 = ctx.r[10].s64 + 3052;
	// 826E0314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E031C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E032C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0338: 4BD86AE9  bl 0x82466e20
	ctx.lr = 0x826E033C;
	sub_82466E20(ctx, base);
	// 826E033C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E0350 size=24
    let mut pc: u32 = 0x826E0350;
    'dispatch: loop {
        match pc {
            0x826E0350 => {
    //   block [0x826E0350..0x826E0368)
	// 826E0350: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0354: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0358: 394A5910  addi r10, r10, 0x5910
	ctx.r[10].s64 = ctx.r[10].s64 + 22800;
	// 826E035C: 816BC174  lwz r11, -0x3e8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16012 as u32) ) } as u64;
	// 826E0360: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E0364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0368 size=116
    let mut pc: u32 = 0x826E0368;
    'dispatch: loop {
        match pc {
            0x826E0368 => {
    //   block [0x826E0368..0x826E03DC)
	// 826E0368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E036C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0374: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0378: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E037C: 390B5910  addi r8, r11, 0x5910
	ctx.r[8].s64 = ctx.r[11].s64 + 22800;
	// 826E0380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0384: 392A6928  addi r9, r10, 0x6928
	ctx.r[9].s64 = ctx.r[10].s64 + 26920;
	// 826E0388: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E038C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E0390: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0394: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E039C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E03A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E03A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E03A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E03AC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E03B0: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 826E03B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E03B8: 386B0C1C  addi r3, r11, 0xc1c
	ctx.r[3].s64 = ctx.r[11].s64 + 3100;
	// 826E03BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E03C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E03C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E03C8: 4BD86A59  bl 0x82466e20
	ctx.lr = 0x826E03CC;
	sub_82466E20(ctx, base);
	// 826E03CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E03D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E03D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E03D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E03E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E03E0 size=108
    let mut pc: u32 = 0x826E03E0;
    'dispatch: loop {
        match pc {
            0x826E03E0 => {
    //   block [0x826E03E0..0x826E044C)
	// 826E03E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E03E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E03E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E03EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E03F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E03F4: 38EBC178  addi r7, r11, -0x3e88
	ctx.r[7].s64 = ctx.r[11].s64 + -16008;
	// 826E03F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E03FC: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 826E0400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E040C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0410: 386A0C4C  addi r3, r10, 0xc4c
	ctx.r[3].s64 = ctx.r[10].s64 + 3148;
	// 826E0414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E041C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E042C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0438: 4BD869E9  bl 0x82466e20
	ctx.lr = 0x826E043C;
	sub_82466E20(ctx, base);
	// 826E043C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0450 size=112
    let mut pc: u32 = 0x826E0450;
    'dispatch: loop {
        match pc {
            0x826E0450 => {
    //   block [0x826E0450..0x826E04C0)
	// 826E0450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E045C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0460: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0464: 38AA0C1C  addi r5, r10, 0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + 3100;
	// 826E0468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E046C: 390BC208  addi r8, r11, -0x3df8
	ctx.r[8].s64 = ctx.r[11].s64 + -15864;
	// 826E0470: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826E0474: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826E0478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E047C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0488: 386A0C7C  addi r3, r10, 0xc7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3196;
	// 826E048C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E049C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E04A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E04A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E04A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E04AC: 4BD86975  bl 0x82466e20
	ctx.lr = 0x826E04B0;
	sub_82466E20(ctx, base);
	// 826E04B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E04B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E04B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E04BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E04C0 size=112
    let mut pc: u32 = 0x826E04C0;
    'dispatch: loop {
        match pc {
            0x826E04C0 => {
    //   block [0x826E04C0..0x826E0530)
	// 826E04C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E04C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E04C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E04CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E04D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E04D4: 38AA0C1C  addi r5, r10, 0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + 3100;
	// 826E04D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E04DC: 390BC328  addi r8, r11, -0x3cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -15576;
	// 826E04E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E04E4: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 826E04E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E04EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E04F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E04F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E04F8: 386A0CAC  addi r3, r10, 0xcac
	ctx.r[3].s64 = ctx.r[10].s64 + 3244;
	// 826E04FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E050C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E051C: 4BD86905  bl 0x82466e20
	ctx.lr = 0x826E0520;
	sub_82466E20(ctx, base);
	// 826E0520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E052C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0530 size=108
    let mut pc: u32 = 0x826E0530;
    'dispatch: loop {
        match pc {
            0x826E0530 => {
    //   block [0x826E0530..0x826E059C)
	// 826E0530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E053C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0544: 38EBC340  addi r7, r11, -0x3cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -15552;
	// 826E0548: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E054C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826E0550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0554: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E055C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0560: 386A0CDC  addi r3, r10, 0xcdc
	ctx.r[3].s64 = ctx.r[10].s64 + 3292;
	// 826E0564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E056C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E057C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0588: 4BD86899  bl 0x82466e20
	ctx.lr = 0x826E058C;
	sub_82466E20(ctx, base);
	// 826E058C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E05A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E05A0 size=108
    let mut pc: u32 = 0x826E05A0;
    'dispatch: loop {
        match pc {
            0x826E05A0 => {
    //   block [0x826E05A0..0x826E060C)
	// 826E05A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E05A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E05A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E05AC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E05B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E05B4: 38EBC418  addi r7, r11, -0x3be8
	ctx.r[7].s64 = ctx.r[11].s64 + -15336;
	// 826E05B8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E05BC: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 826E05C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E05C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E05C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E05CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E05D0: 386A0D0C  addi r3, r10, 0xd0c
	ctx.r[3].s64 = ctx.r[10].s64 + 3340;
	// 826E05D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E05D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E05DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E05E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E05E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E05E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E05EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E05F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E05F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E05F8: 4BD86829  bl 0x82466e20
	ctx.lr = 0x826E05FC;
	sub_82466E20(ctx, base);
	// 826E05FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0610 size=112
    let mut pc: u32 = 0x826E0610;
    'dispatch: loop {
        match pc {
            0x826E0610 => {
    //   block [0x826E0610..0x826E0680)
	// 826E0610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E061C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0620: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0624: 38AA0C1C  addi r5, r10, 0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + 3100;
	// 826E0628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E062C: 390BC4A8  addi r8, r11, -0x3b58
	ctx.r[8].s64 = ctx.r[11].s64 + -15192;
	// 826E0630: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826E0634: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826E0638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E063C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0648: 386A0D3C  addi r3, r10, 0xd3c
	ctx.r[3].s64 = ctx.r[10].s64 + 3388;
	// 826E064C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E065C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E066C: 4BD867B5  bl 0x82466e20
	ctx.lr = 0x826E0670;
	sub_82466E20(ctx, base);
	// 826E0670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


