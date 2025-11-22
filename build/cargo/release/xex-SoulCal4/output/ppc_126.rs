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


pub fn sub_826E0680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0680 size=108
    let mut pc: u32 = 0x826E0680;
    'dispatch: loop {
        match pc {
            0x826E0680 => {
    //   block [0x826E0680..0x826E06EC)
	// 826E0680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E068C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0694: 38EBC598  addi r7, r11, -0x3a68
	ctx.r[7].s64 = ctx.r[11].s64 + -14952;
	// 826E0698: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E069C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826E06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E06A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E06A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E06AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E06B0: 386A0D6C  addi r3, r10, 0xd6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3436;
	// 826E06B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E06B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E06BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E06C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E06CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E06D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E06D8: 4BD86749  bl 0x82466e20
	ctx.lr = 0x826E06DC;
	sub_82466E20(ctx, base);
	// 826E06DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E06E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E06E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E06E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E06F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E06F0 size=108
    let mut pc: u32 = 0x826E06F0;
    'dispatch: loop {
        match pc {
            0x826E06F0 => {
    //   block [0x826E06F0..0x826E075C)
	// 826E06F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E06F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E06F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E06FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0704: 38EBC5B0  addi r7, r11, -0x3a50
	ctx.r[7].s64 = ctx.r[11].s64 + -14928;
	// 826E0708: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E070C: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826E0710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0720: 386A0D9C  addi r3, r10, 0xd9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3484;
	// 826E0724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E072C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E073C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0748: 4BD866D9  bl 0x82466e20
	ctx.lr = 0x826E074C;
	sub_82466E20(ctx, base);
	// 826E074C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0760 size=116
    let mut pc: u32 = 0x826E0760;
    'dispatch: loop {
        match pc {
            0x826E0760 => {
    //   block [0x826E0760..0x826E07D4)
	// 826E0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E076C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0770: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0774: 390BC614  addi r8, r11, -0x39ec
	ctx.r[8].s64 = ctx.r[11].s64 + -14828;
	// 826E0778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E077C: 392A6954  addi r9, r10, 0x6954
	ctx.r[9].s64 = ctx.r[10].s64 + 26964;
	// 826E0780: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0784: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E0788: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E078C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E079C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E07A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E07A4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E07A8: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826E07AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E07B0: 386B0DCC  addi r3, r11, 0xdcc
	ctx.r[3].s64 = ctx.r[11].s64 + 3532;
	// 826E07B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E07B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E07BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E07C0: 4BD86661  bl 0x82466e20
	ctx.lr = 0x826E07C4;
	sub_82466E20(ctx, base);
	// 826E07C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E07C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E07CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E07D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E07D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E07D8 size=108
    let mut pc: u32 = 0x826E07D8;
    'dispatch: loop {
        match pc {
            0x826E07D8 => {
    //   block [0x826E07D8..0x826E0844)
	// 826E07D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E07DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E07E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E07E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E07E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E07EC: 38EBC630  addi r7, r11, -0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + -14800;
	// 826E07F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E07F4: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826E07F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E07FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0808: 386A0DFC  addi r3, r10, 0xdfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3580;
	// 826E080C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E081C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E082C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0830: 4BD865F1  bl 0x82466e20
	ctx.lr = 0x826E0834;
	sub_82466E20(ctx, base);
	// 826E0834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E083C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0848 size=108
    let mut pc: u32 = 0x826E0848;
    'dispatch: loop {
        match pc {
            0x826E0848 => {
    //   block [0x826E0848..0x826E08B4)
	// 826E0848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0854: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E085C: 38EBC678  addi r7, r11, -0x3988
	ctx.r[7].s64 = ctx.r[11].s64 + -14728;
	// 826E0860: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E0864: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826E0868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E086C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0878: 386A0E2C  addi r3, r10, 0xe2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3628;
	// 826E087C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E088C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E089C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E08A0: 4BD86581  bl 0x82466e20
	ctx.lr = 0x826E08A4;
	sub_82466E20(ctx, base);
	// 826E08A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E08A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E08AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E08B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E08B8 size=108
    let mut pc: u32 = 0x826E08B8;
    'dispatch: loop {
        match pc {
            0x826E08B8 => {
    //   block [0x826E08B8..0x826E0924)
	// 826E08B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E08BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E08C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E08C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E08C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E08CC: 38EBC708  addi r7, r11, -0x38f8
	ctx.r[7].s64 = ctx.r[11].s64 + -14584;
	// 826E08D0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E08D4: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826E08D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E08DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E08E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E08E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E08E8: 386A0E5C  addi r3, r10, 0xe5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3676;
	// 826E08EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E08F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E08F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E08F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E08FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E090C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0910: 4BD86511  bl 0x82466e20
	ctx.lr = 0x826E0914;
	sub_82466E20(ctx, base);
	// 826E0914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E091C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0928 size=100
    let mut pc: u32 = 0x826E0928;
    'dispatch: loop {
        match pc {
            0x826E0928 => {
    //   block [0x826E0928..0x826E098C)
	// 826E0928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E093C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0948: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826E094C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E095C: 386A0E8C  addi r3, r10, 0xe8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3724;
	// 826E0960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E096C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0978: 4BD864A9  bl 0x82466e20
	ctx.lr = 0x826E097C;
	sub_82466E20(ctx, base);
	// 826E097C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0990 size=112
    let mut pc: u32 = 0x826E0990;
    'dispatch: loop {
        match pc {
            0x826E0990 => {
    //   block [0x826E0990..0x826E0A00)
	// 826E0990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E099C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E09A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E09A4: 38AA0E8C  addi r5, r10, 0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + 3724;
	// 826E09A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E09AC: 390BC798  addi r8, r11, -0x3868
	ctx.r[8].s64 = ctx.r[11].s64 + -14440;
	// 826E09B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E09B4: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826E09B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E09BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E09C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E09C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E09C8: 386A0EBC  addi r3, r10, 0xebc
	ctx.r[3].s64 = ctx.r[10].s64 + 3772;
	// 826E09CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E09D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E09D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E09D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E09DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E09E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E09E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E09E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E09EC: 4BD86435  bl 0x82466e20
	ctx.lr = 0x826E09F0;
	sub_82466E20(ctx, base);
	// 826E09F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E09F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E09F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E09FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0A00 size=108
    let mut pc: u32 = 0x826E0A00;
    'dispatch: loop {
        match pc {
            0x826E0A00 => {
    //   block [0x826E0A00..0x826E0A6C)
	// 826E0A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0A0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0A10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0A14: 38EBC7F8  addi r7, r11, -0x3808
	ctx.r[7].s64 = ctx.r[11].s64 + -14344;
	// 826E0A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E0A1C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826E0A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0A30: 386A0EEC  addi r3, r10, 0xeec
	ctx.r[3].s64 = ctx.r[10].s64 + 3820;
	// 826E0A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0A58: 4BD863C9  bl 0x82466e20
	ctx.lr = 0x826E0A5C;
	sub_82466E20(ctx, base);
	// 826E0A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0A70 size=108
    let mut pc: u32 = 0x826E0A70;
    'dispatch: loop {
        match pc {
            0x826E0A70 => {
    //   block [0x826E0A70..0x826E0ADC)
	// 826E0A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0A7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0A80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0A84: 38EBC828  addi r7, r11, -0x37d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14296;
	// 826E0A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0A8C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826E0A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0A94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0AA0: 386A0F1C  addi r3, r10, 0xf1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3868;
	// 826E0AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0AC8: 4BD86359  bl 0x82466e20
	ctx.lr = 0x826E0ACC;
	sub_82466E20(ctx, base);
	// 826E0ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0AE0 size=108
    let mut pc: u32 = 0x826E0AE0;
    'dispatch: loop {
        match pc {
            0x826E0AE0 => {
    //   block [0x826E0AE0..0x826E0B4C)
	// 826E0AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0AEC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0AF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0AF4: 38EBC888  addi r7, r11, -0x3778
	ctx.r[7].s64 = ctx.r[11].s64 + -14200;
	// 826E0AF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0AFC: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826E0B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0B04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0B10: 386A0F4C  addi r3, r10, 0xf4c
	ctx.r[3].s64 = ctx.r[10].s64 + 3916;
	// 826E0B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0B38: 4BD862E9  bl 0x82466e20
	ctx.lr = 0x826E0B3C;
	sub_82466E20(ctx, base);
	// 826E0B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0B50 size=112
    let mut pc: u32 = 0x826E0B50;
    'dispatch: loop {
        match pc {
            0x826E0B50 => {
    //   block [0x826E0B50..0x826E0BC0)
	// 826E0B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0B5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0B60: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826E0B64: 38EAC8E8  addi r7, r10, -0x3718
	ctx.r[7].s64 = ctx.r[10].s64 + -14104;
	// 826E0B68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0B6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E0B70: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826E0B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0B78: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0B7C: 396B6968  addi r11, r11, 0x6968
	ctx.r[11].s64 = ctx.r[11].s64 + 26984;
	// 826E0B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0B88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0B8C: 386A0F7C  addi r3, r10, 0xf7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3964;
	// 826E0B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0B94: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E0B98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0B9C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E0BA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0BA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0BA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0BAC: 4BD86275  bl 0x82466e20
	ctx.lr = 0x826E0BB0;
	sub_82466E20(ctx, base);
	// 826E0BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0BC0 size=96
    let mut pc: u32 = 0x826E0BC0;
    'dispatch: loop {
        match pc {
            0x826E0BC0 => {
    //   block [0x826E0BC0..0x826E0C20)
	// 826E0BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0BCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0BD4: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826E0BD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0BE0: 386A0FAC  addi r3, r10, 0xfac
	ctx.r[3].s64 = ctx.r[10].s64 + 4012;
	// 826E0BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0C00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E0C04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0C08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0C0C: 4BD86215  bl 0x82466e20
	ctx.lr = 0x826E0C10;
	sub_82466E20(ctx, base);
	// 826E0C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0C20 size=112
    let mut pc: u32 = 0x826E0C20;
    'dispatch: loop {
        match pc {
            0x826E0C20 => {
    //   block [0x826E0C20..0x826E0C90)
	// 826E0C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0C2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0C30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0C34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0C38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0C3C: 390BCA08  addi r8, r11, -0x35f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13816;
	// 826E0C40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E0C44: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826E0C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0C4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0C50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0C58: 386A0FDC  addi r3, r10, 0xfdc
	ctx.r[3].s64 = ctx.r[10].s64 + 4060;
	// 826E0C5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0C7C: 4BD861A5  bl 0x82466e20
	ctx.lr = 0x826E0C80;
	sub_82466E20(ctx, base);
	// 826E0C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E0C90 size=24
    let mut pc: u32 = 0x826E0C90;
    'dispatch: loop {
        match pc {
            0x826E0C90 => {
    //   block [0x826E0C90..0x826E0CA8)
	// 826E0C90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0C94: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0C98: 394A5988  addi r10, r10, 0x5988
	ctx.r[10].s64 = ctx.r[10].s64 + 22920;
	// 826E0C9C: 816BC62C  lwz r11, -0x39d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14804 as u32) ) } as u64;
	// 826E0CA0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826E0CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0CA8 size=116
    let mut pc: u32 = 0x826E0CA8;
    'dispatch: loop {
        match pc {
            0x826E0CA8 => {
    //   block [0x826E0CA8..0x826E0D1C)
	// 826E0CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0CB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0CB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0CBC: 390B5988  addi r8, r11, 0x5988
	ctx.r[8].s64 = ctx.r[11].s64 + 22920;
	// 826E0CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0CC4: 392A69E4  addi r9, r10, 0x69e4
	ctx.r[9].s64 = ctx.r[10].s64 + 27108;
	// 826E0CC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0CCC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E0CD0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0CD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0CDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0CEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E0CF0: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826E0CF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E0CF8: 386B100C  addi r3, r11, 0x100c
	ctx.r[3].s64 = ctx.r[11].s64 + 4108;
	// 826E0CFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0D00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0D08: 4BD86119  bl 0x82466e20
	ctx.lr = 0x826E0D0C;
	sub_82466E20(ctx, base);
	// 826E0D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E0D20 size=24
    let mut pc: u32 = 0x826E0D20;
    'dispatch: loop {
        match pc {
            0x826E0D20 => {
    //   block [0x826E0D20..0x826E0D38)
	// 826E0D20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0D24: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0D28: 394A5A60  addi r10, r10, 0x5a60
	ctx.r[10].s64 = ctx.r[10].s64 + 23136;
	// 826E0D2C: 816BCA70  lwz r11, -0x3590(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13712 as u32) ) } as u64;
	// 826E0D30: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E0D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0D38 size=116
    let mut pc: u32 = 0x826E0D38;
    'dispatch: loop {
        match pc {
            0x826E0D38 => {
    //   block [0x826E0D38..0x826E0DAC)
	// 826E0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0D44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0D48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0D4C: 390B5A60  addi r8, r11, 0x5a60
	ctx.r[8].s64 = ctx.r[11].s64 + 23136;
	// 826E0D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0D54: 392A6A98  addi r9, r10, 0x6a98
	ctx.r[9].s64 = ctx.r[10].s64 + 27288;
	// 826E0D58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0D5C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E0D60: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E0D64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0D6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0D7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E0D80: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 826E0D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E0D88: 386B103C  addi r3, r11, 0x103c
	ctx.r[3].s64 = ctx.r[11].s64 + 4156;
	// 826E0D8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0D98: 4BD86089  bl 0x82466e20
	ctx.lr = 0x826E0D9C;
	sub_82466E20(ctx, base);
	// 826E0D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0DB0 size=112
    let mut pc: u32 = 0x826E0DB0;
    'dispatch: loop {
        match pc {
            0x826E0DB0 => {
    //   block [0x826E0DB0..0x826E0E20)
	// 826E0DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0DC0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0DC4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0DCC: 390BCA78  addi r8, r11, -0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + -13704;
	// 826E0DD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E0DD4: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 826E0DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0DE8: 386A106C  addi r3, r10, 0x106c
	ctx.r[3].s64 = ctx.r[10].s64 + 4204;
	// 826E0DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0E0C: 4BD86015  bl 0x82466e20
	ctx.lr = 0x826E0E10;
	sub_82466E20(ctx, base);
	// 826E0E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0E20 size=112
    let mut pc: u32 = 0x826E0E20;
    'dispatch: loop {
        match pc {
            0x826E0E20 => {
    //   block [0x826E0E20..0x826E0E90)
	// 826E0E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0E30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0E34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0E3C: 390BCAC0  addi r8, r11, -0x3540
	ctx.r[8].s64 = ctx.r[11].s64 + -13632;
	// 826E0E40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E0E44: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 826E0E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0E58: 386A109C  addi r3, r10, 0x109c
	ctx.r[3].s64 = ctx.r[10].s64 + 4252;
	// 826E0E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0E7C: 4BD85FA5  bl 0x82466e20
	ctx.lr = 0x826E0E80;
	sub_82466E20(ctx, base);
	// 826E0E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0E90 size=108
    let mut pc: u32 = 0x826E0E90;
    'dispatch: loop {
        match pc {
            0x826E0E90 => {
    //   block [0x826E0E90..0x826E0EFC)
	// 826E0E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0E9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0EA4: 38EBCB08  addi r7, r11, -0x34f8
	ctx.r[7].s64 = ctx.r[11].s64 + -13560;
	// 826E0EA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0EAC: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826E0EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0EC0: 386A10CC  addi r3, r10, 0x10cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4300;
	// 826E0EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0EE8: 4BD85F39  bl 0x82466e20
	ctx.lr = 0x826E0EEC;
	sub_82466E20(ctx, base);
	// 826E0EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0F00 size=108
    let mut pc: u32 = 0x826E0F00;
    'dispatch: loop {
        match pc {
            0x826E0F00 => {
    //   block [0x826E0F00..0x826E0F6C)
	// 826E0F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0F0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0F14: 38EBCB68  addi r7, r11, -0x3498
	ctx.r[7].s64 = ctx.r[11].s64 + -13464;
	// 826E0F18: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E0F1C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826E0F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0F30: 386A10FC  addi r3, r10, 0x10fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4348;
	// 826E0F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0F58: 4BD85EC9  bl 0x82466e20
	ctx.lr = 0x826E0F5C;
	sub_82466E20(ctx, base);
	// 826E0F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0F70 size=112
    let mut pc: u32 = 0x826E0F70;
    'dispatch: loop {
        match pc {
            0x826E0F70 => {
    //   block [0x826E0F70..0x826E0FE0)
	// 826E0F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0F7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E0F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0F84: 392B6ACC  addi r9, r11, 0x6acc
	ctx.r[9].s64 = ctx.r[11].s64 + 27340;
	// 826E0F88: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826E0F8C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E0F90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0F94: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826E0F98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0F9C: 396BCC10  addi r11, r11, -0x33f0
	ctx.r[11].s64 = ctx.r[11].s64 + -13296;
	// 826E0FA0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E0FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0FA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E0FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0FB0: 386A112C  addi r3, r10, 0x112c
	ctx.r[3].s64 = ctx.r[10].s64 + 4396;
	// 826E0FB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0FB8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E0FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0FC0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E0FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0FC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0FCC: 4BD85E55  bl 0x82466e20
	ctx.lr = 0x826E0FD0;
	sub_82466E20(ctx, base);
	// 826E0FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0FE0 size=116
    let mut pc: u32 = 0x826E0FE0;
    'dispatch: loop {
        match pc {
            0x826E0FE0 => {
    //   block [0x826E0FE0..0x826E1054)
	// 826E0FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0FEC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0FF0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826E0FF4: 390ACD60  addi r8, r10, -0x32a0
	ctx.r[8].s64 = ctx.r[10].s64 + -12960;
	// 826E0FF8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0FFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1000: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1008: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E100C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1014: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 826E1018: 396B6B20  addi r11, r11, 0x6b20
	ctx.r[11].s64 = ctx.r[11].s64 + 27424;
	// 826E101C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1020: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1024: 386A115C  addi r3, r10, 0x115c
	ctx.r[3].s64 = ctx.r[10].s64 + 4444;
	// 826E1028: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E102C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1030: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E103C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1040: 4BD85DE1  bl 0x82466e20
	ctx.lr = 0x826E1044;
	sub_82466E20(ctx, base);
	// 826E1044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E104C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1058 size=112
    let mut pc: u32 = 0x826E1058;
    'dispatch: loop {
        match pc {
            0x826E1058 => {
    //   block [0x826E1058..0x826E10C8)
	// 826E1058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1068: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E106C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1074: 390BCDF0  addi r8, r11, -0x3210
	ctx.r[8].s64 = ctx.r[11].s64 + -12816;
	// 826E1078: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E107C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826E1080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E108C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1090: 386A118C  addi r3, r10, 0x118c
	ctx.r[3].s64 = ctx.r[10].s64 + 4492;
	// 826E1094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E109C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E10A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E10A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E10AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E10B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E10B4: 4BD85D6D  bl 0x82466e20
	ctx.lr = 0x826E10B8;
	sub_82466E20(ctx, base);
	// 826E10B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E10BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E10C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E10C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E10C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E10C8 size=112
    let mut pc: u32 = 0x826E10C8;
    'dispatch: loop {
        match pc {
            0x826E10C8 => {
    //   block [0x826E10C8..0x826E1138)
	// 826E10C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E10CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E10D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E10D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E10D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E10DC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E10E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E10E4: 390BCE98  addi r8, r11, -0x3168
	ctx.r[8].s64 = ctx.r[11].s64 + -12648;
	// 826E10E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E10EC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826E10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E10F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E10F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E10FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1100: 386A11BC  addi r3, r10, 0x11bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4540;
	// 826E1104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E110C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E111C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1124: 4BD85CFD  bl 0x82466e20
	ctx.lr = 0x826E1128;
	sub_82466E20(ctx, base);
	// 826E1128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E112C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1138 size=112
    let mut pc: u32 = 0x826E1138;
    'dispatch: loop {
        match pc {
            0x826E1138 => {
    //   block [0x826E1138..0x826E11A8)
	// 826E1138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E113C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1148: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E114C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1154: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826E1158: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E115C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826E1160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E116C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1170: 386A11EC  addi r3, r10, 0x11ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4588;
	// 826E1174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1194: 4BD85C8D  bl 0x82466e20
	ctx.lr = 0x826E1198;
	sub_82466E20(ctx, base);
	// 826E1198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E11A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E11A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E11A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E11A8 size=108
    let mut pc: u32 = 0x826E11A8;
    'dispatch: loop {
        match pc {
            0x826E11A8 => {
    //   block [0x826E11A8..0x826E1214)
	// 826E11A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E11AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E11B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E11B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E11B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E11BC: 38EBCFD0  addi r7, r11, -0x3030
	ctx.r[7].s64 = ctx.r[11].s64 + -12336;
	// 826E11C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E11C4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826E11C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E11CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E11D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E11D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E11D8: 386A121C  addi r3, r10, 0x121c
	ctx.r[3].s64 = ctx.r[10].s64 + 4636;
	// 826E11DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E11E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E11E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E11E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E11EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E11F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E11F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E11F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E11FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1200: 4BD85C21  bl 0x82466e20
	ctx.lr = 0x826E1204;
	sub_82466E20(ctx, base);
	// 826E1204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E120C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1218 size=112
    let mut pc: u32 = 0x826E1218;
    'dispatch: loop {
        match pc {
            0x826E1218 => {
    //   block [0x826E1218..0x826E1288)
	// 826E1218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E121C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1224: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1228: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E122C: 392A6BE0  addi r9, r10, 0x6be0
	ctx.r[9].s64 = ctx.r[10].s64 + 27616;
	// 826E1230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1234: 390BD07C  addi r8, r11, -0x2f84
	ctx.r[8].s64 = ctx.r[11].s64 + -12164;
	// 826E1238: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E123C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826E1240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E124C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1250: 386A124C  addi r3, r10, 0x124c
	ctx.r[3].s64 = ctx.r[10].s64 + 4684;
	// 826E1254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E125C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E126C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1274: 4BD85BAD  bl 0x82466e20
	ctx.lr = 0x826E1278;
	sub_82466E20(ctx, base);
	// 826E1278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E127C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1288 size=100
    let mut pc: u32 = 0x826E1288;
    'dispatch: loop {
        match pc {
            0x826E1288 => {
    //   block [0x826E1288..0x826E12EC)
	// 826E1288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E129C: 38AA1ABC  addi r5, r10, 0x1abc
	ctx.r[5].s64 = ctx.r[10].s64 + 6844;
	// 826E12A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E12A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E12A8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826E12AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E12B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E12B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E12B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E12BC: 386A127C  addi r3, r10, 0x127c
	ctx.r[3].s64 = ctx.r[10].s64 + 4732;
	// 826E12C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E12C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E12C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E12CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E12D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E12D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E12D8: 4BD85B49  bl 0x82466e20
	ctx.lr = 0x826E12DC;
	sub_82466E20(ctx, base);
	// 826E12DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E12E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E12E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E12E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E12F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E12F0 size=24
    let mut pc: u32 = 0x826E12F0;
    'dispatch: loop {
        match pc {
            0x826E12F0 => {
    //   block [0x826E12F0..0x826E1308)
	// 826E12F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E12F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E12F8: 394A5B80  addi r10, r10, 0x5b80
	ctx.r[10].s64 = ctx.r[10].s64 + 23424;
	// 826E12FC: 816BD0B0  lwz r11, -0x2f50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12112 as u32) ) } as u64;
	// 826E1300: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826E1304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1308 size=108
    let mut pc: u32 = 0x826E1308;
    'dispatch: loop {
        match pc {
            0x826E1308 => {
    //   block [0x826E1308..0x826E1374)
	// 826E1308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E130C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1314: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1318: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E131C: 38EB5B80  addi r7, r11, 0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + 23424;
	// 826E1320: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E1324: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826E1328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E132C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1338: 386A12AC  addi r3, r10, 0x12ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4780;
	// 826E133C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E134C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E135C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1360: 4BD85AC1  bl 0x82466e20
	ctx.lr = 0x826E1364;
	sub_82466E20(ctx, base);
	// 826E1364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E136C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1378 size=112
    let mut pc: u32 = 0x826E1378;
    'dispatch: loop {
        match pc {
            0x826E1378 => {
    //   block [0x826E1378..0x826E13E8)
	// 826E1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E137C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1384: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1388: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E138C: 392A6C60  addi r9, r10, 0x6c60
	ctx.r[9].s64 = ctx.r[10].s64 + 27744;
	// 826E1390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1394: 390BD0B8  addi r8, r11, -0x2f48
	ctx.r[8].s64 = ctx.r[11].s64 + -12104;
	// 826E1398: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E139C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826E13A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E13A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E13A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E13AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E13B0: 386A12DC  addi r3, r10, 0x12dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4828;
	// 826E13B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E13B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E13BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E13C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E13C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E13C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E13CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E13D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E13D4: 4BD85A4D  bl 0x82466e20
	ctx.lr = 0x826E13D8;
	sub_82466E20(ctx, base);
	// 826E13D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E13DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E13E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E13E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E13E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E13E8 size=112
    let mut pc: u32 = 0x826E13E8;
    'dispatch: loop {
        match pc {
            0x826E13E8 => {
    //   block [0x826E13E8..0x826E1458)
	// 826E13E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E13EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E13F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E13F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E13F8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E13FC: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1404: 390BD100  addi r8, r11, -0x2f00
	ctx.r[8].s64 = ctx.r[11].s64 + -12032;
	// 826E1408: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E140C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826E1410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E141C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1420: 386A130C  addi r3, r10, 0x130c
	ctx.r[3].s64 = ctx.r[10].s64 + 4876;
	// 826E1424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E142C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E143C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1444: 4BD859DD  bl 0x82466e20
	ctx.lr = 0x826E1448;
	sub_82466E20(ctx, base);
	// 826E1448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1458 size=116
    let mut pc: u32 = 0x826E1458;
    'dispatch: loop {
        match pc {
            0x826E1458 => {
    //   block [0x826E1458..0x826E14CC)
	// 826E1458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1464: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1468: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E146C: 390AD130  addi r8, r10, -0x2ed0
	ctx.r[8].s64 = ctx.r[10].s64 + -11984;
	// 826E1470: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1474: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1478: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E147C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1480: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E148C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826E1490: 396B6C88  addi r11, r11, 0x6c88
	ctx.r[11].s64 = ctx.r[11].s64 + 27784;
	// 826E1494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1498: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E149C: 386A133C  addi r3, r10, 0x133c
	ctx.r[3].s64 = ctx.r[10].s64 + 4924;
	// 826E14A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E14A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E14A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E14AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E14B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E14B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E14B8: 4BD85969  bl 0x82466e20
	ctx.lr = 0x826E14BC;
	sub_82466E20(ctx, base);
	// 826E14BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E14C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E14C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E14C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E14D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E14D0 size=100
    let mut pc: u32 = 0x826E14D0;
    'dispatch: loop {
        match pc {
            0x826E14D0 => {
    //   block [0x826E14D0..0x826E1534)
	// 826E14D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E14D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E14D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E14DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E14E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E14E4: 38AA133C  addi r5, r10, 0x133c
	ctx.r[5].s64 = ctx.r[10].s64 + 4924;
	// 826E14E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E14EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E14F0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826E14F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E14F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E14FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1504: 386A136C  addi r3, r10, 0x136c
	ctx.r[3].s64 = ctx.r[10].s64 + 4972;
	// 826E1508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E150C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1510: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E1514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1518: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E151C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1520: 4BD85901  bl 0x82466e20
	ctx.lr = 0x826E1524;
	sub_82466E20(ctx, base);
	// 826E1524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E152C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E1538 size=24
    let mut pc: u32 = 0x826E1538;
    'dispatch: loop {
        match pc {
            0x826E1538 => {
    //   block [0x826E1538..0x826E1550)
	// 826E1538: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E153C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1540: 394A5BC8  addi r10, r10, 0x5bc8
	ctx.r[10].s64 = ctx.r[10].s64 + 23496;
	// 826E1544: 816BD0B4  lwz r11, -0x2f4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12108 as u32) ) } as u64;
	// 826E1548: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1550 size=116
    let mut pc: u32 = 0x826E1550;
    'dispatch: loop {
        match pc {
            0x826E1550 => {
    //   block [0x826E1550..0x826E15C4)
	// 826E1550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E155C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1560: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1564: 392B6CC4  addi r9, r11, 0x6cc4
	ctx.r[9].s64 = ctx.r[11].s64 + 27844;
	// 826E1568: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E156C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1570: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E1574: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826E1578: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E157C: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826E1580: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1584: 396B5BC8  addi r11, r11, 0x5bc8
	ctx.r[11].s64 = ctx.r[11].s64 + 23496;
	// 826E1588: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E158C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1590: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E1594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1598: 386A139C  addi r3, r10, 0x139c
	ctx.r[3].s64 = ctx.r[10].s64 + 5020;
	// 826E159C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E15A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E15A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E15A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E15AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E15B0: 4BD85871  bl 0x82466e20
	ctx.lr = 0x826E15B4;
	sub_82466E20(ctx, base);
	// 826E15B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E15B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E15BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E15C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E15C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E15C8 size=116
    let mut pc: u32 = 0x826E15C8;
    'dispatch: loop {
        match pc {
            0x826E15C8 => {
    //   block [0x826E15C8..0x826E163C)
	// 826E15C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E15CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E15D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E15D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E15D8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E15DC: 392B6D20  addi r9, r11, 0x6d20
	ctx.r[9].s64 = ctx.r[11].s64 + 27936;
	// 826E15E0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E15E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E15E8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826E15EC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826E15F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E15F4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826E15F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E15FC: 396BD1E0  addi r11, r11, -0x2e20
	ctx.r[11].s64 = ctx.r[11].s64 + -11808;
	// 826E1600: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E1604: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1608: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1610: 386A13CC  addi r3, r10, 0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5068;
	// 826E1614: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E1618: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1620: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E1624: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E1628: 4BD857F9  bl 0x82466e20
	ctx.lr = 0x826E162C;
	sub_82466E20(ctx, base);
	// 826E162C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1640 size=108
    let mut pc: u32 = 0x826E1640;
    'dispatch: loop {
        match pc {
            0x826E1640 => {
    //   block [0x826E1640..0x826E16AC)
	// 826E1640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E164C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1654: 38EBD330  addi r7, r11, -0x2cd0
	ctx.r[7].s64 = ctx.r[11].s64 + -11472;
	// 826E1658: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E165C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826E1660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1670: 386A13FC  addi r3, r10, 0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5116;
	// 826E1674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E167C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1698: 4BD85789  bl 0x82466e20
	ctx.lr = 0x826E169C;
	sub_82466E20(ctx, base);
	// 826E169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E16A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E16A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E16A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E16B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E16B0 size=24
    let mut pc: u32 = 0x826E16B0;
    'dispatch: loop {
        match pc {
            0x826E16B0 => {
    //   block [0x826E16B0..0x826E16C8)
	// 826E16B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E16B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E16B8: 394A5C70  addi r10, r10, 0x5c70
	ctx.r[10].s64 = ctx.r[10].s64 + 23664;
	// 826E16BC: 816BD1DC  lwz r11, -0x2e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11812 as u32) ) } as u64;
	// 826E16C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E16C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E16C8 size=116
    let mut pc: u32 = 0x826E16C8;
    'dispatch: loop {
        match pc {
            0x826E16C8 => {
    //   block [0x826E16C8..0x826E173C)
	// 826E16C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E16CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E16D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E16D4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E16D8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E16DC: 390B5C70  addi r8, r11, 0x5c70
	ctx.r[8].s64 = ctx.r[11].s64 + 23664;
	// 826E16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E16E4: 392A6DB8  addi r9, r10, 0x6db8
	ctx.r[9].s64 = ctx.r[10].s64 + 28088;
	// 826E16E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E16EC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826E16F0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E16F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E16F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E16FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E170C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E1710: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826E1714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1718: 386B142C  addi r3, r11, 0x142c
	ctx.r[3].s64 = ctx.r[11].s64 + 5164;
	// 826E171C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E1720: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1728: 4BD856F9  bl 0x82466e20
	ctx.lr = 0x826E172C;
	sub_82466E20(ctx, base);
	// 826E172C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1740 size=112
    let mut pc: u32 = 0x826E1740;
    'dispatch: loop {
        match pc {
            0x826E1740 => {
    //   block [0x826E1740..0x826E17B0)
	// 826E1740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E174C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1750: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1754: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E175C: 390BD3AC  addi r8, r11, -0x2c54
	ctx.r[8].s64 = ctx.r[11].s64 + -11348;
	// 826E1760: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E1764: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826E1768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E176C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1778: 386A145C  addi r3, r10, 0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + 5212;
	// 826E177C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E178C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E179C: 4BD85685  bl 0x82466e20
	ctx.lr = 0x826E17A0;
	sub_82466E20(ctx, base);
	// 826E17A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E17A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E17A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E17AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E17B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E17B0 size=24
    let mut pc: u32 = 0x826E17B0;
    'dispatch: loop {
        match pc {
            0x826E17B0 => {
    //   block [0x826E17B0..0x826E17C8)
	// 826E17B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E17B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E17B8: 394A5E08  addi r10, r10, 0x5e08
	ctx.r[10].s64 = ctx.r[10].s64 + 24072;
	// 826E17BC: 816BD3DC  lwz r11, -0x2c24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11300 as u32) ) } as u64;
	// 826E17C0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E17C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E17C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E17C8 size=116
    let mut pc: u32 = 0x826E17C8;
    'dispatch: loop {
        match pc {
            0x826E17C8 => {
    //   block [0x826E17C8..0x826E183C)
	// 826E17C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E17CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E17D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E17D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E17D8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E17DC: 392B6DF0  addi r9, r11, 0x6df0
	ctx.r[9].s64 = ctx.r[11].s64 + 28144;
	// 826E17E0: 38AA13CC  addi r5, r10, 0x13cc
	ctx.r[5].s64 = ctx.r[10].s64 + 5068;
	// 826E17E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E17E8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826E17EC: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826E17F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E17F4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826E17F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E17FC: 396B5E08  addi r11, r11, 0x5e08
	ctx.r[11].s64 = ctx.r[11].s64 + 24072;
	// 826E1800: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E1804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1808: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E180C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1810: 386A148C  addi r3, r10, 0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + 5260;
	// 826E1814: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E1818: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E181C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1820: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E1824: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E1828: 4BD855F9  bl 0x82466e20
	ctx.lr = 0x826E182C;
	sub_82466E20(ctx, base);
	// 826E182C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1840 size=112
    let mut pc: u32 = 0x826E1840;
    'dispatch: loop {
        match pc {
            0x826E1840 => {
    //   block [0x826E1840..0x826E18B0)
	// 826E1840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E184C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1850: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1854: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E185C: 390BD3E0  addi r8, r11, -0x2c20
	ctx.r[8].s64 = ctx.r[11].s64 + -11296;
	// 826E1860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1864: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826E1868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E186C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1878: 386A14BC  addi r3, r10, 0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5308;
	// 826E187C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E188C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E189C: 4BD85585  bl 0x82466e20
	ctx.lr = 0x826E18A0;
	sub_82466E20(ctx, base);
	// 826E18A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E18A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E18A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E18AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E18B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E18B0 size=100
    let mut pc: u32 = 0x826E18B0;
    'dispatch: loop {
        match pc {
            0x826E18B0 => {
    //   block [0x826E18B0..0x826E1914)
	// 826E18B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E18B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E18B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E18BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E18C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E18C4: 38AA1ABC  addi r5, r10, 0x1abc
	ctx.r[5].s64 = ctx.r[10].s64 + 6844;
	// 826E18C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E18CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E18D0: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826E18D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E18D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E18DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E18E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E18E4: 386A14EC  addi r3, r10, 0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + 5356;
	// 826E18E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E18EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E18F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E18F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E18F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E18FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1900: 4BD85521  bl 0x82466e20
	ctx.lr = 0x826E1904;
	sub_82466E20(ctx, base);
	// 826E1904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E190C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1918 size=112
    let mut pc: u32 = 0x826E1918;
    'dispatch: loop {
        match pc {
            0x826E1918 => {
    //   block [0x826E1918..0x826E1988)
	// 826E1918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E191C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1928: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826E192C: 38EAD3F8  addi r7, r10, -0x2c08
	ctx.r[7].s64 = ctx.r[10].s64 + -11272;
	// 826E1930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1934: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1938: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826E193C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1940: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1944: 396B6E60  addi r11, r11, 0x6e60
	ctx.r[11].s64 = ctx.r[11].s64 + 28256;
	// 826E1948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E194C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1950: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1954: 386A151C  addi r3, r10, 0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + 5404;
	// 826E1958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E195C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1960: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1964: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1968: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E196C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1974: 4BD854AD  bl 0x82466e20
	ctx.lr = 0x826E1978;
	sub_82466E20(ctx, base);
	// 826E1978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E197C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1988 size=112
    let mut pc: u32 = 0x826E1988;
    'dispatch: loop {
        match pc {
            0x826E1988 => {
    //   block [0x826E1988..0x826E19F8)
	// 826E1988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E198C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1998: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E199C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E19A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E19A4: 390BD530  addi r8, r11, -0x2ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -10960;
	// 826E19A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E19AC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826E19B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E19B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E19B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E19BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E19C0: 386A154C  addi r3, r10, 0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + 5452;
	// 826E19C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E19C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E19CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E19D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E19D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E19D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E19DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E19E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E19E4: 4BD8543D  bl 0x82466e20
	ctx.lr = 0x826E19E8;
	sub_82466E20(ctx, base);
	// 826E19E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E19EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E19F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E19F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E19F8 size=112
    let mut pc: u32 = 0x826E19F8;
    'dispatch: loop {
        match pc {
            0x826E19F8 => {
    //   block [0x826E19F8..0x826E1A68)
	// 826E19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E19FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1A04: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1A08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E1A0C: 38EAD560  addi r7, r10, -0x2aa0
	ctx.r[7].s64 = ctx.r[10].s64 + -10912;
	// 826E1A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1A14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1A18: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826E1A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1A20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1A24: 396B6EB4  addi r11, r11, 0x6eb4
	ctx.r[11].s64 = ctx.r[11].s64 + 28340;
	// 826E1A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1A34: 386A157C  addi r3, r10, 0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + 5500;
	// 826E1A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1A3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1A44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1A48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1A4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1A50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1A54: 4BD853CD  bl 0x82466e20
	ctx.lr = 0x826E1A58;
	sub_82466E20(ctx, base);
	// 826E1A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1A68 size=112
    let mut pc: u32 = 0x826E1A68;
    'dispatch: loop {
        match pc {
            0x826E1A68 => {
    //   block [0x826E1A68..0x826E1AD8)
	// 826E1A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1A7C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1A84: 390BD590  addi r8, r11, -0x2a70
	ctx.r[8].s64 = ctx.r[11].s64 + -10864;
	// 826E1A88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1A8C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826E1A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1A94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1AA0: 386A15AC  addi r3, r10, 0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5548;
	// 826E1AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1AC4: 4BD8535D  bl 0x82466e20
	ctx.lr = 0x826E1AC8;
	sub_82466E20(ctx, base);
	// 826E1AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1AD8 size=108
    let mut pc: u32 = 0x826E1AD8;
    'dispatch: loop {
        match pc {
            0x826E1AD8 => {
    //   block [0x826E1AD8..0x826E1B44)
	// 826E1AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1AE4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1AEC: 38EBD5A8  addi r7, r11, -0x2a58
	ctx.r[7].s64 = ctx.r[11].s64 + -10840;
	// 826E1AF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E1AF4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826E1AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1B08: 386A15DC  addi r3, r10, 0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5596;
	// 826E1B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1B30: 4BD852F1  bl 0x82466e20
	ctx.lr = 0x826E1B34;
	sub_82466E20(ctx, base);
	// 826E1B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1B48 size=112
    let mut pc: u32 = 0x826E1B48;
    'dispatch: loop {
        match pc {
            0x826E1B48 => {
    //   block [0x826E1B48..0x826E1BB8)
	// 826E1B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1B54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1B5C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1B64: 390BD5C0  addi r8, r11, -0x2a40
	ctx.r[8].s64 = ctx.r[11].s64 + -10816;
	// 826E1B68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1B6C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826E1B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1B74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1B80: 386A160C  addi r3, r10, 0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + 5644;
	// 826E1B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1BA4: 4BD8527D  bl 0x82466e20
	ctx.lr = 0x826E1BA8;
	sub_82466E20(ctx, base);
	// 826E1BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1BB8 size=112
    let mut pc: u32 = 0x826E1BB8;
    'dispatch: loop {
        match pc {
            0x826E1BB8 => {
    //   block [0x826E1BB8..0x826E1C28)
	// 826E1BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1BC4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1BC8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E1BCC: 38EAD5D8  addi r7, r10, -0x2a28
	ctx.r[7].s64 = ctx.r[10].s64 + -10792;
	// 826E1BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1BD4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1BD8: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826E1BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1BE0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1BE4: 396B6EC0  addi r11, r11, 0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 28352;
	// 826E1BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1BF4: 386A163C  addi r3, r10, 0x163c
	ctx.r[3].s64 = ctx.r[10].s64 + 5692;
	// 826E1BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1BFC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1C04: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1C0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1C14: 4BD8520D  bl 0x82466e20
	ctx.lr = 0x826E1C18;
	sub_82466E20(ctx, base);
	// 826E1C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1C28 size=112
    let mut pc: u32 = 0x826E1C28;
    'dispatch: loop {
        match pc {
            0x826E1C28 => {
    //   block [0x826E1C28..0x826E1C98)
	// 826E1C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1C34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1C38: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826E1C3C: 38EAD6B0  addi r7, r10, -0x2950
	ctx.r[7].s64 = ctx.r[10].s64 + -10576;
	// 826E1C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1C44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1C48: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826E1C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1C50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1C54: 396B6F00  addi r11, r11, 0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + 28416;
	// 826E1C58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1C60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1C64: 386A166C  addi r3, r10, 0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + 5740;
	// 826E1C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1C6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1C70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1C74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1C78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1C7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1C80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1C84: 4BD8519D  bl 0x82466e20
	ctx.lr = 0x826E1C88;
	sub_82466E20(ctx, base);
	// 826E1C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1C98 size=108
    let mut pc: u32 = 0x826E1C98;
    'dispatch: loop {
        match pc {
            0x826E1C98 => {
    //   block [0x826E1C98..0x826E1D04)
	// 826E1C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1CA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1CAC: 38EBD800  addi r7, r11, -0x2800
	ctx.r[7].s64 = ctx.r[11].s64 + -10240;
	// 826E1CB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E1CB4: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826E1CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1CBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1CC8: 386A169C  addi r3, r10, 0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + 5788;
	// 826E1CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1CF0: 4BD85131  bl 0x82466e20
	ctx.lr = 0x826E1CF4;
	sub_82466E20(ctx, base);
	// 826E1CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1D08 size=116
    let mut pc: u32 = 0x826E1D08;
    'dispatch: loop {
        match pc {
            0x826E1D08 => {
    //   block [0x826E1D08..0x826E1D7C)
	// 826E1D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1D14: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1D18: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E1D1C: 390AD860  addi r8, r10, -0x27a0
	ctx.r[8].s64 = ctx.r[10].s64 + -10144;
	// 826E1D20: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1D28: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1D2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1D30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1D3C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826E1D40: 396B6FA0  addi r11, r11, 0x6fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 28576;
	// 826E1D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1D4C: 386A16CC  addi r3, r10, 0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5836;
	// 826E1D50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1D54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1D58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1D68: 4BD850B9  bl 0x82466e20
	ctx.lr = 0x826E1D6C;
	sub_82466E20(ctx, base);
	// 826E1D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1D80 size=116
    let mut pc: u32 = 0x826E1D80;
    'dispatch: loop {
        match pc {
            0x826E1D80 => {
    //   block [0x826E1D80..0x826E1DF4)
	// 826E1D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1D8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1D90: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826E1D94: 390AD980  addi r8, r10, -0x2680
	ctx.r[8].s64 = ctx.r[10].s64 + -9856;
	// 826E1D98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1DA0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1DA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1DA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1DB4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826E1DB8: 396B6FDC  addi r11, r11, 0x6fdc
	ctx.r[11].s64 = ctx.r[11].s64 + 28636;
	// 826E1DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1DC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1DC4: 386A16FC  addi r3, r10, 0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5884;
	// 826E1DC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1DD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1DE0: 4BD85041  bl 0x82466e20
	ctx.lr = 0x826E1DE4;
	sub_82466E20(ctx, base);
	// 826E1DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1DF8 size=108
    let mut pc: u32 = 0x826E1DF8;
    'dispatch: loop {
        match pc {
            0x826E1DF8 => {
    //   block [0x826E1DF8..0x826E1E64)
	// 826E1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1E04: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1E0C: 38EBD9E0  addi r7, r11, -0x2620
	ctx.r[7].s64 = ctx.r[11].s64 + -9760;
	// 826E1E10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E1E14: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826E1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1E28: 386A172C  addi r3, r10, 0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + 5932;
	// 826E1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1E50: 4BD84FD1  bl 0x82466e20
	ctx.lr = 0x826E1E54;
	sub_82466E20(ctx, base);
	// 826E1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1E68 size=112
    let mut pc: u32 = 0x826E1E68;
    'dispatch: loop {
        match pc {
            0x826E1E68 => {
    //   block [0x826E1E68..0x826E1ED8)
	// 826E1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1E74: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1E78: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E1E7C: 38EADA88  addi r7, r10, -0x2578
	ctx.r[7].s64 = ctx.r[10].s64 + -9592;
	// 826E1E80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1E84: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1E88: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826E1E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1E90: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1E94: 396B6FF0  addi r11, r11, 0x6ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 28656;
	// 826E1E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1E9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1EA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1EA4: 386A175C  addi r3, r10, 0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + 5980;
	// 826E1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1EAC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1EB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1EB4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1EB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1EBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1EC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1EC4: 4BD84F5D  bl 0x82466e20
	ctx.lr = 0x826E1EC8;
	sub_82466E20(ctx, base);
	// 826E1EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1ED8 size=112
    let mut pc: u32 = 0x826E1ED8;
    'dispatch: loop {
        match pc {
            0x826E1ED8 => {
    //   block [0x826E1ED8..0x826E1F48)
	// 826E1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1EE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1EE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1EEC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1EF4: 390BDB00  addi r8, r11, -0x2500
	ctx.r[8].s64 = ctx.r[11].s64 + -9472;
	// 826E1EF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E1EFC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826E1F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1F10: 386A178C  addi r3, r10, 0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + 6028;
	// 826E1F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1F34: 4BD84EED  bl 0x82466e20
	ctx.lr = 0x826E1F38;
	sub_82466E20(ctx, base);
	// 826E1F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1F48 size=116
    let mut pc: u32 = 0x826E1F48;
    'dispatch: loop {
        match pc {
            0x826E1F48 => {
    //   block [0x826E1F48..0x826E1FBC)
	// 826E1F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1F54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1F58: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E1F5C: 390ADB48  addi r8, r10, -0x24b8
	ctx.r[8].s64 = ctx.r[10].s64 + -9400;
	// 826E1F60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1F68: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1F6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1F70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1F7C: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 826E1F80: 396B7010  addi r11, r11, 0x7010
	ctx.r[11].s64 = ctx.r[11].s64 + 28688;
	// 826E1F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1F8C: 386A17BC  addi r3, r10, 0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6076;
	// 826E1F90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1F94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1F98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1FA8: 4BD84E79  bl 0x82466e20
	ctx.lr = 0x826E1FAC;
	sub_82466E20(ctx, base);
	// 826E1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1FC0 size=116
    let mut pc: u32 = 0x826E1FC0;
    'dispatch: loop {
        match pc {
            0x826E1FC0 => {
    //   block [0x826E1FC0..0x826E2034)
	// 826E1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1FCC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1FD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E1FD4: 390ADC50  addi r8, r10, -0x23b0
	ctx.r[8].s64 = ctx.r[10].s64 + -9136;
	// 826E1FD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1FDC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1FE0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1FE4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1FE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1FF4: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 826E1FF8: 396B7058  addi r11, r11, 0x7058
	ctx.r[11].s64 = ctx.r[11].s64 + 28760;
	// 826E1FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2000: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2004: 386A17EC  addi r3, r10, 0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6124;
	// 826E2008: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E200C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2010: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E201C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2020: 4BD84E01  bl 0x82466e20
	ctx.lr = 0x826E2024;
	sub_82466E20(ctx, base);
	// 826E2024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2038 size=116
    let mut pc: u32 = 0x826E2038;
    'dispatch: loop {
        match pc {
            0x826E2038 => {
    //   block [0x826E2038..0x826E20AC)
	// 826E2038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E203C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2044: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2048: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826E204C: 390ADC98  addi r8, r10, -0x2368
	ctx.r[8].s64 = ctx.r[10].s64 + -9064;
	// 826E2050: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2054: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2058: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E205C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2060: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E206C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826E2070: 396B7068  addi r11, r11, 0x7068
	ctx.r[11].s64 = ctx.r[11].s64 + 28776;
	// 826E2074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E207C: 386A181C  addi r3, r10, 0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + 6172;
	// 826E2080: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2088: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E208C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2098: 4BD84D89  bl 0x82466e20
	ctx.lr = 0x826E209C;
	sub_82466E20(ctx, base);
	// 826E209C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E20A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E20A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E20A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E20B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E20B0 size=100
    let mut pc: u32 = 0x826E20B0;
    'dispatch: loop {
        match pc {
            0x826E20B0 => {
    //   block [0x826E20B0..0x826E2114)
	// 826E20B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E20B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E20B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E20BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E20C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E20C4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E20C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E20CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E20D0: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826E20D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E20D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E20DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E20E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E20E4: 386A184C  addi r3, r10, 0x184c
	ctx.r[3].s64 = ctx.r[10].s64 + 6220;
	// 826E20E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E20EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E20F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E20F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E20F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E20FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2100: 4BD84D21  bl 0x82466e20
	ctx.lr = 0x826E2104;
	sub_82466E20(ctx, base);
	// 826E2104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E210C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2118 size=112
    let mut pc: u32 = 0x826E2118;
    'dispatch: loop {
        match pc {
            0x826E2118 => {
    //   block [0x826E2118..0x826E2188)
	// 826E2118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2124: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2128: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E212C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2134: 390BDDD0  addi r8, r11, -0x2230
	ctx.r[8].s64 = ctx.r[11].s64 + -8752;
	// 826E2138: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E213C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826E2140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E214C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2150: 386A187C  addi r3, r10, 0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + 6268;
	// 826E2154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E215C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E216C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2174: 4BD84CAD  bl 0x82466e20
	ctx.lr = 0x826E2178;
	sub_82466E20(ctx, base);
	// 826E2178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2188 size=112
    let mut pc: u32 = 0x826E2188;
    'dispatch: loop {
        match pc {
            0x826E2188 => {
    //   block [0x826E2188..0x826E21F8)
	// 826E2188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2198: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E219C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E21A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E21A4: 390BDDE8  addi r8, r11, -0x2218
	ctx.r[8].s64 = ctx.r[11].s64 + -8728;
	// 826E21A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E21AC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826E21B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E21B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E21B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E21BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E21C0: 386A18AC  addi r3, r10, 0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6316;
	// 826E21C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E21C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E21CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E21D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E21D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E21D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E21DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E21E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E21E4: 4BD84C3D  bl 0x82466e20
	ctx.lr = 0x826E21E8;
	sub_82466E20(ctx, base);
	// 826E21E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E21EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E21F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E21F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E21F8 size=112
    let mut pc: u32 = 0x826E21F8;
    'dispatch: loop {
        match pc {
            0x826E21F8 => {
    //   block [0x826E21F8..0x826E2268)
	// 826E21F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E21FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2204: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2208: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E220C: 38EADE18  addi r7, r10, -0x21e8
	ctx.r[7].s64 = ctx.r[10].s64 + -8680;
	// 826E2210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2214: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2218: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826E221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2220: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2224: 396B70E0  addi r11, r11, 0x70e0
	ctx.r[11].s64 = ctx.r[11].s64 + 28896;
	// 826E2228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E222C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2230: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2234: 386A18DC  addi r3, r10, 0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6364;
	// 826E2238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E223C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2240: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2244: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2248: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E224C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2250: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2254: 4BD84BCD  bl 0x82466e20
	ctx.lr = 0x826E2258;
	sub_82466E20(ctx, base);
	// 826E2258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2268 size=112
    let mut pc: u32 = 0x826E2268;
    'dispatch: loop {
        match pc {
            0x826E2268 => {
    //   block [0x826E2268..0x826E22D8)
	// 826E2268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2278: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E227C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2284: 390BDE90  addi r8, r11, -0x2170
	ctx.r[8].s64 = ctx.r[11].s64 + -8560;
	// 826E2288: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E228C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826E2290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E229C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E22A0: 386A190C  addi r3, r10, 0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + 6412;
	// 826E22A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E22A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E22AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E22B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E22B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E22B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E22BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E22C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E22C4: 4BD84B5D  bl 0x82466e20
	ctx.lr = 0x826E22C8;
	sub_82466E20(ctx, base);
	// 826E22C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E22CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E22D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E22D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E22D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E22D8 size=24
    let mut pc: u32 = 0x826E22D8;
    'dispatch: loop {
        match pc {
            0x826E22D8 => {
    //   block [0x826E22D8..0x826E22F0)
	// 826E22D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E22DC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E22E0: 394A5F70  addi r10, r10, 0x5f70
	ctx.r[10].s64 = ctx.r[10].s64 + 24432;
	// 826E22E4: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 826E22E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E22EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E22F0 size=116
    let mut pc: u32 = 0x826E22F0;
    'dispatch: loop {
        match pc {
            0x826E22F0 => {
    //   block [0x826E22F0..0x826E2364)
	// 826E22F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E22F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E22F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E22FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2300: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E2304: 390B5F70  addi r8, r11, 0x5f70
	ctx.r[8].s64 = ctx.r[11].s64 + 24432;
	// 826E2308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E230C: 392A7120  addi r9, r10, 0x7120
	ctx.r[9].s64 = ctx.r[10].s64 + 28960;
	// 826E2310: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2314: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E2318: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E231C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2324: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E232C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2334: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E2338: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826E233C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2340: 386B193C  addi r3, r11, 0x193c
	ctx.r[3].s64 = ctx.r[11].s64 + 6460;
	// 826E2344: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E2348: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E234C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2350: 4BD84AD1  bl 0x82466e20
	ctx.lr = 0x826E2354;
	sub_82466E20(ctx, base);
	// 826E2354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E235C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2368 size=116
    let mut pc: u32 = 0x826E2368;
    'dispatch: loop {
        match pc {
            0x826E2368 => {
    //   block [0x826E2368..0x826E23DC)
	// 826E2368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2374: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2378: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826E237C: 390ADEC8  addi r8, r10, -0x2138
	ctx.r[8].s64 = ctx.r[10].s64 + -8504;
	// 826E2380: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2388: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E238C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E239C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826E23A0: 396B7134  addi r11, r11, 0x7134
	ctx.r[11].s64 = ctx.r[11].s64 + 28980;
	// 826E23A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E23A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E23AC: 386A196C  addi r3, r10, 0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + 6508;
	// 826E23B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E23B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E23B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E23BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E23C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E23C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E23C8: 4BD84A59  bl 0x82466e20
	ctx.lr = 0x826E23CC;
	sub_82466E20(ctx, base);
	// 826E23CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E23D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E23D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E23D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E23E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E23E0 size=112
    let mut pc: u32 = 0x826E23E0;
    'dispatch: loop {
        match pc {
            0x826E23E0 => {
    //   block [0x826E23E0..0x826E2450)
	// 826E23E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E23E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E23E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E23EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E23F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E23F4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E23F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E23FC: 390BDF88  addi r8, r11, -0x2078
	ctx.r[8].s64 = ctx.r[11].s64 + -8312;
	// 826E2400: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E2404: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826E2408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E240C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2418: 386A199C  addi r3, r10, 0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + 6556;
	// 826E241C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E242C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E243C: 4BD849E5  bl 0x82466e20
	ctx.lr = 0x826E2440;
	sub_82466E20(ctx, base);
	// 826E2440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E244C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2450 size=112
    let mut pc: u32 = 0x826E2450;
    'dispatch: loop {
        match pc {
            0x826E2450 => {
    //   block [0x826E2450..0x826E24C0)
	// 826E2450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E245C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2464: 38EADFA0  addi r7, r10, -0x2060
	ctx.r[7].s64 = ctx.r[10].s64 + -8288;
	// 826E2468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E246C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2470: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826E2474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2478: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E247C: 396B7158  addi r11, r11, 0x7158
	ctx.r[11].s64 = ctx.r[11].s64 + 29016;
	// 826E2480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E248C: 386A19CC  addi r3, r10, 0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6604;
	// 826E2490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2494: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2498: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E249C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E24A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E24A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E24A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E24AC: 4BD84975  bl 0x82466e20
	ctx.lr = 0x826E24B0;
	sub_82466E20(ctx, base);
	// 826E24B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E24B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E24B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E24BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E24C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E24C0 size=112
    let mut pc: u32 = 0x826E24C0;
    'dispatch: loop {
        match pc {
            0x826E24C0 => {
    //   block [0x826E24C0..0x826E2530)
	// 826E24C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E24C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E24C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E24CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E24D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E24D4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E24D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E24DC: 390BDFD0  addi r8, r11, -0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + -8240;
	// 826E24E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E24E4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826E24E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E24EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E24F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E24F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E24F8: 386A19FC  addi r3, r10, 0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6652;
	// 826E24FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E250C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E251C: 4BD84905  bl 0x82466e20
	ctx.lr = 0x826E2520;
	sub_82466E20(ctx, base);
	// 826E2520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2530 size=116
    let mut pc: u32 = 0x826E2530;
    'dispatch: loop {
        match pc {
            0x826E2530 => {
    //   block [0x826E2530..0x826E25A4)
	// 826E2530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E253C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2540: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E2544: 390ADFE8  addi r8, r10, -0x2018
	ctx.r[8].s64 = ctx.r[10].s64 + -8216;
	// 826E2548: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E254C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2550: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2558: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E255C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2564: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826E2568: 396B7164  addi r11, r11, 0x7164
	ctx.r[11].s64 = ctx.r[11].s64 + 29028;
	// 826E256C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2570: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2574: 386A1A2C  addi r3, r10, 0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6700;
	// 826E2578: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E257C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2580: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E258C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2590: 4BD84891  bl 0x82466e20
	ctx.lr = 0x826E2594;
	sub_82466E20(ctx, base);
	// 826E2594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E259C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E25A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E25A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E25A8 size=112
    let mut pc: u32 = 0x826E25A8;
    'dispatch: loop {
        match pc {
            0x826E25A8 => {
    //   block [0x826E25A8..0x826E2618)
	// 826E25A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E25AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E25B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E25B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E25B8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E25BC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E25C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E25C4: 390BE090  addi r8, r11, -0x1f70
	ctx.r[8].s64 = ctx.r[11].s64 + -8048;
	// 826E25C8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826E25CC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826E25D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E25D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E25D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E25DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E25E0: 386A1A5C  addi r3, r10, 0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 6748;
	// 826E25E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E25E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E25EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E25F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E25F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E25F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E25FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2604: 4BD8481D  bl 0x82466e20
	ctx.lr = 0x826E2608;
	sub_82466E20(ctx, base);
	// 826E2608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E260C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2618 size=112
    let mut pc: u32 = 0x826E2618;
    'dispatch: loop {
        match pc {
            0x826E2618 => {
    //   block [0x826E2618..0x826E2688)
	// 826E2618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E261C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2628: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E262C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2634: 390BE1C8  addi r8, r11, -0x1e38
	ctx.r[8].s64 = ctx.r[11].s64 + -7736;
	// 826E2638: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E263C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826E2640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2650: 386A1A8C  addi r3, r10, 0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 6796;
	// 826E2654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E266C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2674: 4BD847AD  bl 0x82466e20
	ctx.lr = 0x826E2678;
	sub_82466E20(ctx, base);
	// 826E2678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E267C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2688 size=116
    let mut pc: u32 = 0x826E2688;
    'dispatch: loop {
        match pc {
            0x826E2688 => {
    //   block [0x826E2688..0x826E26FC)
	// 826E2688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E268C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2694: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2698: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E269C: 390BE1E0  addi r8, r11, -0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + -7712;
	// 826E26A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E26A4: 392A719C  addi r9, r10, 0x719c
	ctx.r[9].s64 = ctx.r[10].s64 + 29084;
	// 826E26A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E26AC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E26B0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E26B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E26B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E26BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E26C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E26C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E26C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E26CC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E26D0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826E26D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E26D8: 386B1ABC  addi r3, r11, 0x1abc
	ctx.r[3].s64 = ctx.r[11].s64 + 6844;
	// 826E26DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E26E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E26E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E26E8: 4BD84739  bl 0x82466e20
	ctx.lr = 0x826E26EC;
	sub_82466E20(ctx, base);
	// 826E26EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E26F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E26F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E26F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2700 size=100
    let mut pc: u32 = 0x826E2700;
    'dispatch: loop {
        match pc {
            0x826E2700 => {
    //   block [0x826E2700..0x826E2764)
	// 826E2700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E270C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2714: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2720: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826E2724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E272C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2734: 386A1AEC  addi r3, r10, 0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + 6892;
	// 826E2738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E273C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E2744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E274C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2750: 4BD846D1  bl 0x82466e20
	ctx.lr = 0x826E2754;
	sub_82466E20(ctx, base);
	// 826E2754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E275C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2768 size=112
    let mut pc: u32 = 0x826E2768;
    'dispatch: loop {
        match pc {
            0x826E2768 => {
    //   block [0x826E2768..0x826E27D8)
	// 826E2768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E276C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2774: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E277C: 38AA1AEC  addi r5, r10, 0x1aec
	ctx.r[5].s64 = ctx.r[10].s64 + 6892;
	// 826E2780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2784: 390BE210  addi r8, r11, -0x1df0
	ctx.r[8].s64 = ctx.r[11].s64 + -7664;
	// 826E2788: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E278C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826E2790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E279C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E27A0: 386A1B1C  addi r3, r10, 0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 6940;
	// 826E27A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E27A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E27AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E27B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E27B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E27B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E27BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E27C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E27C4: 4BD8465D  bl 0x82466e20
	ctx.lr = 0x826E27C8;
	sub_82466E20(ctx, base);
	// 826E27C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E27CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E27D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E27D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E27D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E27D8 size=112
    let mut pc: u32 = 0x826E27D8;
    'dispatch: loop {
        match pc {
            0x826E27D8 => {
    //   block [0x826E27D8..0x826E2848)
	// 826E27D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E27DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E27E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E27E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E27E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E27EC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E27F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E27F4: 390BE228  addi r8, r11, -0x1dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -7640;
	// 826E27F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E27FC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826E2800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E280C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2810: 386A1B4C  addi r3, r10, 0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 6988;
	// 826E2814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E281C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E282C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2834: 4BD845ED  bl 0x82466e20
	ctx.lr = 0x826E2838;
	sub_82466E20(ctx, base);
	// 826E2838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E283C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2848 size=112
    let mut pc: u32 = 0x826E2848;
    'dispatch: loop {
        match pc {
            0x826E2848 => {
    //   block [0x826E2848..0x826E28B8)
	// 826E2848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E284C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2858: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E285C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2864: 390BE270  addi r8, r11, -0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + -7568;
	// 826E2868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E286C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826E2870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E287C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2880: 386A1B7C  addi r3, r10, 0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7036;
	// 826E2884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E288C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E289C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E28A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E28A4: 4BD8457D  bl 0x82466e20
	ctx.lr = 0x826E28A8;
	sub_82466E20(ctx, base);
	// 826E28A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E28AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E28B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E28B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E28B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E28B8 size=116
    let mut pc: u32 = 0x826E28B8;
    'dispatch: loop {
        match pc {
            0x826E28B8 => {
    //   block [0x826E28B8..0x826E292C)
	// 826E28B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E28BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E28C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E28C4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E28C8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826E28CC: 390AE318  addi r8, r10, -0x1ce8
	ctx.r[8].s64 = ctx.r[10].s64 + -7400;
	// 826E28D0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E28D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E28D8: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E28DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E28E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E28E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E28E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E28EC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826E28F0: 396B71B0  addi r11, r11, 0x71b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29104;
	// 826E28F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E28F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E28FC: 386A1BAC  addi r3, r10, 0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + 7084;
	// 826E2900: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2908: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E290C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2918: 4BD84509  bl 0x82466e20
	ctx.lr = 0x826E291C;
	sub_82466E20(ctx, base);
	// 826E291C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2930 size=112
    let mut pc: u32 = 0x826E2930;
    'dispatch: loop {
        match pc {
            0x826E2930 => {
    //   block [0x826E2930..0x826E29A0)
	// 826E2930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E293C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2940: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2944: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E294C: 390BE3D8  addi r8, r11, -0x1c28
	ctx.r[8].s64 = ctx.r[11].s64 + -7208;
	// 826E2950: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E2954: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826E2958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E295C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2968: 386A1BDC  addi r3, r10, 0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 7132;
	// 826E296C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E297C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E298C: 4BD84495  bl 0x82466e20
	ctx.lr = 0x826E2990;
	sub_82466E20(ctx, base);
	// 826E2990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E299C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E29A0 size=100
    let mut pc: u32 = 0x826E29A0;
    'dispatch: loop {
        match pc {
            0x826E29A0 => {
    //   block [0x826E29A0..0x826E2A04)
	// 826E29A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E29A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E29A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E29AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E29B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E29B4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E29B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E29C0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826E29C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E29C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E29CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E29D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E29D4: 386A1C0C  addi r3, r10, 0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7180;
	// 826E29D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E29DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E29E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E29E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E29E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E29EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E29F0: 4BD84431  bl 0x82466e20
	ctx.lr = 0x826E29F4;
	sub_82466E20(ctx, base);
	// 826E29F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E29F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E29FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2A08 size=108
    let mut pc: u32 = 0x826E2A08;
    'dispatch: loop {
        match pc {
            0x826E2A08 => {
    //   block [0x826E2A08..0x826E2A74)
	// 826E2A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2A14: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2A1C: 38EBE408  addi r7, r11, -0x1bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -7160;
	// 826E2A20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2A24: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826E2A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2A38: 386A1C3C  addi r3, r10, 0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7228;
	// 826E2A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2A60: 4BD843C1  bl 0x82466e20
	ctx.lr = 0x826E2A64;
	sub_82466E20(ctx, base);
	// 826E2A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2A78 size=112
    let mut pc: u32 = 0x826E2A78;
    'dispatch: loop {
        match pc {
            0x826E2A78 => {
    //   block [0x826E2A78..0x826E2AE8)
	// 826E2A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2A88: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2A8C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2A94: 390BE438  addi r8, r11, -0x1bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -7112;
	// 826E2A98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E2A9C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826E2AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2AA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2AB0: 386A1C6C  addi r3, r10, 0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7276;
	// 826E2AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2AD4: 4BD8434D  bl 0x82466e20
	ctx.lr = 0x826E2AD8;
	sub_82466E20(ctx, base);
	// 826E2AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2AE8 size=108
    let mut pc: u32 = 0x826E2AE8;
    'dispatch: loop {
        match pc {
            0x826E2AE8 => {
    //   block [0x826E2AE8..0x826E2B54)
	// 826E2AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2AF4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2AF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2AFC: 38EBE468  addi r7, r11, -0x1b98
	ctx.r[7].s64 = ctx.r[11].s64 + -7064;
	// 826E2B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2B04: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 826E2B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2B0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2B18: 386A1C9C  addi r3, r10, 0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7324;
	// 826E2B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2B40: 4BD842E1  bl 0x82466e20
	ctx.lr = 0x826E2B44;
	sub_82466E20(ctx, base);
	// 826E2B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2B58 size=112
    let mut pc: u32 = 0x826E2B58;
    'dispatch: loop {
        match pc {
            0x826E2B58 => {
    //   block [0x826E2B58..0x826E2BC8)
	// 826E2B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2B64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B68: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2B6C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2B74: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 826E2B78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2B7C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826E2B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2B90: 386A1CCC  addi r3, r10, 0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 7372;
	// 826E2B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2BB4: 4BD8426D  bl 0x82466e20
	ctx.lr = 0x826E2BB8;
	sub_82466E20(ctx, base);
	// 826E2BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2BC8 size=108
    let mut pc: u32 = 0x826E2BC8;
    'dispatch: loop {
        match pc {
            0x826E2BC8 => {
    //   block [0x826E2BC8..0x826E2C34)
	// 826E2BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2BD4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2BD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2BDC: 38EBE4E0  addi r7, r11, -0x1b20
	ctx.r[7].s64 = ctx.r[11].s64 + -6944;
	// 826E2BE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2BE4: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 826E2BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2BF8: 386A1CFC  addi r3, r10, 0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7420;
	// 826E2BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2C20: 4BD84201  bl 0x82466e20
	ctx.lr = 0x826E2C24;
	sub_82466E20(ctx, base);
	// 826E2C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2C38 size=112
    let mut pc: u32 = 0x826E2C38;
    'dispatch: loop {
        match pc {
            0x826E2C38 => {
    //   block [0x826E2C38..0x826E2CA8)
	// 826E2C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2C44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2C48: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2C4C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2C54: 390BE510  addi r8, r11, -0x1af0
	ctx.r[8].s64 = ctx.r[11].s64 + -6896;
	// 826E2C58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2C5C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826E2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2C70: 386A1D2C  addi r3, r10, 0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7468;
	// 826E2C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2C94: 4BD8418D  bl 0x82466e20
	ctx.lr = 0x826E2C98;
	sub_82466E20(ctx, base);
	// 826E2C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2CA8 size=108
    let mut pc: u32 = 0x826E2CA8;
    'dispatch: loop {
        match pc {
            0x826E2CA8 => {
    //   block [0x826E2CA8..0x826E2D14)
	// 826E2CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2CB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2CBC: 38EBE558  addi r7, r11, -0x1aa8
	ctx.r[7].s64 = ctx.r[11].s64 + -6824;
	// 826E2CC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2CC4: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 826E2CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2CD8: 386A1D5C  addi r3, r10, 0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7516;
	// 826E2CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2D00: 4BD84121  bl 0x82466e20
	ctx.lr = 0x826E2D04;
	sub_82466E20(ctx, base);
	// 826E2D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2D18 size=112
    let mut pc: u32 = 0x826E2D18;
    'dispatch: loop {
        match pc {
            0x826E2D18 => {
    //   block [0x826E2D18..0x826E2D88)
	// 826E2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2D24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2D28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2D2C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2D34: 390BE588  addi r8, r11, -0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + -6776;
	// 826E2D38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2D3C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826E2D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2D50: 386A1D8C  addi r3, r10, 0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7564;
	// 826E2D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2D74: 4BD840AD  bl 0x82466e20
	ctx.lr = 0x826E2D78;
	sub_82466E20(ctx, base);
	// 826E2D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2D88 size=108
    let mut pc: u32 = 0x826E2D88;
    'dispatch: loop {
        match pc {
            0x826E2D88 => {
    //   block [0x826E2D88..0x826E2DF4)
	// 826E2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2D94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2D9C: 38EBE5D8  addi r7, r11, -0x1a28
	ctx.r[7].s64 = ctx.r[11].s64 + -6696;
	// 826E2DA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E2DA4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826E2DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2DB8: 386A1DBC  addi r3, r10, 0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7612;
	// 826E2DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2DE0: 4BD84041  bl 0x82466e20
	ctx.lr = 0x826E2DE4;
	sub_82466E20(ctx, base);
	// 826E2DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E2DF8 size=24
    let mut pc: u32 = 0x826E2DF8;
    'dispatch: loop {
        match pc {
            0x826E2DF8 => {
    //   block [0x826E2DF8..0x826E2E10)
	// 826E2DF8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2DFC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2E00: 394A6078  addi r10, r10, 0x6078
	ctx.r[10].s64 = ctx.r[10].s64 + 24696;
	// 826E2E04: 816BE5D4  lwz r11, -0x1a2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6700 as u32) ) } as u64;
	// 826E2E08: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E2E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2E10 size=112
    let mut pc: u32 = 0x826E2E10;
    'dispatch: loop {
        match pc {
            0x826E2E10 => {
    //   block [0x826E2E10..0x826E2E80)
	// 826E2E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2E1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E2E20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2E24: 392A7298  addi r9, r10, 0x7298
	ctx.r[9].s64 = ctx.r[10].s64 + 29336;
	// 826E2E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2E2C: 390B6078  addi r8, r11, 0x6078
	ctx.r[8].s64 = ctx.r[11].s64 + 24696;
	// 826E2E30: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E2E34: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826E2E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2E48: 386A1DEC  addi r3, r10, 0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + 7660;
	// 826E2E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2E50: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E2E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2E6C: 4BD83FB5  bl 0x82466e20
	ctx.lr = 0x826E2E70;
	sub_82466E20(ctx, base);
	// 826E2E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2E80 size=108
    let mut pc: u32 = 0x826E2E80;
    'dispatch: loop {
        match pc {
            0x826E2E80 => {
    //   block [0x826E2E80..0x826E2EEC)
	// 826E2E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2E8C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2E94: 38EBE640  addi r7, r11, -0x19c0
	ctx.r[7].s64 = ctx.r[11].s64 + -6592;
	// 826E2E98: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E2E9C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826E2EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2EA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2EB0: 386A1E1C  addi r3, r10, 0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7708;
	// 826E2EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2ED8: 4BD83F49  bl 0x82466e20
	ctx.lr = 0x826E2EDC;
	sub_82466E20(ctx, base);
	// 826E2EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2EF0 size=108
    let mut pc: u32 = 0x826E2EF0;
    'dispatch: loop {
        match pc {
            0x826E2EF0 => {
    //   block [0x826E2EF0..0x826E2F5C)
	// 826E2EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2EFC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2F00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2F04: 38EBE6B8  addi r7, r11, -0x1948
	ctx.r[7].s64 = ctx.r[11].s64 + -6472;
	// 826E2F08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E2F0C: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 826E2F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2F20: 386A1E4C  addi r3, r10, 0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7756;
	// 826E2F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2F48: 4BD83ED9  bl 0x82466e20
	ctx.lr = 0x826E2F4C;
	sub_82466E20(ctx, base);
	// 826E2F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2F60 size=108
    let mut pc: u32 = 0x826E2F60;
    'dispatch: loop {
        match pc {
            0x826E2F60 => {
    //   block [0x826E2F60..0x826E2FCC)
	// 826E2F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2F6C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2F74: 38EBE718  addi r7, r11, -0x18e8
	ctx.r[7].s64 = ctx.r[11].s64 + -6376;
	// 826E2F78: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E2F7C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826E2F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2F90: 386A1E7C  addi r3, r10, 0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7804;
	// 826E2F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2FB8: 4BD83E69  bl 0x82466e20
	ctx.lr = 0x826E2FBC;
	sub_82466E20(ctx, base);
	// 826E2FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E2FD0 size=24
    let mut pc: u32 = 0x826E2FD0;
    'dispatch: loop {
        match pc {
            0x826E2FD0 => {
    //   block [0x826E2FD0..0x826E2FE8)
	// 826E2FD0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2FD4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2FD8: 394A6180  addi r10, r10, 0x6180
	ctx.r[10].s64 = ctx.r[10].s64 + 24960;
	// 826E2FDC: 816BE5D0  lwz r11, -0x1a30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6704 as u32) ) } as u64;
	// 826E2FE0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E2FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2FE8 size=116
    let mut pc: u32 = 0x826E2FE8;
    'dispatch: loop {
        match pc {
            0x826E2FE8 => {
    //   block [0x826E2FE8..0x826E305C)
	// 826E2FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2FF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2FF8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2FFC: 392B71F8  addi r9, r11, 0x71f8
	ctx.r[9].s64 = ctx.r[11].s64 + 29176;
	// 826E3000: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3008: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 826E300C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 826E3010: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3014: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826E3018: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E301C: 396B6180  addi r11, r11, 0x6180
	ctx.r[11].s64 = ctx.r[11].s64 + 24960;
	// 826E3020: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E3024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3028: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E302C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3030: 386A1EAC  addi r3, r10, 0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + 7852;
	// 826E3034: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3038: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E303C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3040: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E3044: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E3048: 4BD83DD9  bl 0x82466e20
	ctx.lr = 0x826E304C;
	sub_82466E20(ctx, base);
	// 826E304C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3060 size=36
    let mut pc: u32 = 0x826E3060;
    'dispatch: loop {
        match pc {
            0x826E3060 => {
    //   block [0x826E3060..0x826E3084)
	// 826E3060: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3064: 814BE63C  lwz r10, -0x19c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6596 as u32) ) } as u64;
	// 826E3068: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E306C: 396B6450  addi r11, r11, 0x6450
	ctx.r[11].s64 = ctx.r[11].s64 + 25680;
	// 826E3070: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E3074: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3078: 814AE7C0  lwz r10, -0x1840(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-6208 as u32) ) } as u64;
	// 826E307C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826E3080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3088 size=116
    let mut pc: u32 = 0x826E3088;
    'dispatch: loop {
        match pc {
            0x826E3088 => {
    //   block [0x826E3088..0x826E30FC)
	// 826E3088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3094: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3098: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E309C: 390B6450  addi r8, r11, 0x6450
	ctx.r[8].s64 = ctx.r[11].s64 + 25680;
	// 826E30A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E30A4: 392A7380  addi r9, r10, 0x7380
	ctx.r[9].s64 = ctx.r[10].s64 + 29568;
	// 826E30A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E30AC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E30B0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E30B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E30B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E30BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E30C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E30C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E30C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E30CC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E30D0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826E30D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E30D8: 386B1EDC  addi r3, r11, 0x1edc
	ctx.r[3].s64 = ctx.r[11].s64 + 7900;
	// 826E30DC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E30E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E30E8: 4BD83D39  bl 0x82466e20
	ctx.lr = 0x826E30EC;
	sub_82466E20(ctx, base);
	// 826E30EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E30F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E30F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E30F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3100 size=24
    let mut pc: u32 = 0x826E3100;
    'dispatch: loop {
        match pc {
            0x826E3100 => {
    //   block [0x826E3100..0x826E3118)
	// 826E3100: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3104: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3108: 394A6480  addi r10, r10, 0x6480
	ctx.r[10].s64 = ctx.r[10].s64 + 25728;
	// 826E310C: 816BE7C8  lwz r11, -0x1838(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6200 as u32) ) } as u64;
	// 826E3110: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E3114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3118 size=116
    let mut pc: u32 = 0x826E3118;
    'dispatch: loop {
        match pc {
            0x826E3118 => {
    //   block [0x826E3118..0x826E318C)
	// 826E3118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E311C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3124: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3128: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E312C: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 826E3130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3134: 392A73D8  addi r9, r10, 0x73d8
	ctx.r[9].s64 = ctx.r[10].s64 + 29656;
	// 826E3138: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E313C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E3140: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 826E3144: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E314C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E315C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3160: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826E3164: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3168: 386B1F0C  addi r3, r11, 0x1f0c
	ctx.r[3].s64 = ctx.r[11].s64 + 7948;
	// 826E316C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E3170: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3178: 4BD83CA9  bl 0x82466e20
	ctx.lr = 0x826E317C;
	sub_82466E20(ctx, base);
	// 826E317C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3190 size=116
    let mut pc: u32 = 0x826E3190;
    'dispatch: loop {
        match pc {
            0x826E3190 => {
    //   block [0x826E3190..0x826E3204)
	// 826E3190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E319C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E31A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E31A4: 390BE7D0  addi r8, r11, -0x1830
	ctx.r[8].s64 = ctx.r[11].s64 + -6192;
	// 826E31A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E31AC: 392A7420  addi r9, r10, 0x7420
	ctx.r[9].s64 = ctx.r[10].s64 + 29728;
	// 826E31B0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E31B4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E31B8: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 826E31BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E31C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E31C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E31C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E31CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E31D4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E31D8: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826E31DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E31E0: 386B1F3C  addi r3, r11, 0x1f3c
	ctx.r[3].s64 = ctx.r[11].s64 + 7996;
	// 826E31E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E31E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E31EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E31F0: 4BD83C31  bl 0x82466e20
	ctx.lr = 0x826E31F4;
	sub_82466E20(ctx, base);
	// 826E31F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E31F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E31FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3208 size=112
    let mut pc: u32 = 0x826E3208;
    'dispatch: loop {
        match pc {
            0x826E3208 => {
    //   block [0x826E3208..0x826E3278)
	// 826E3208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E320C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3214: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3218: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E321C: 38EAE8A8  addi r7, r10, -0x1758
	ctx.r[7].s64 = ctx.r[10].s64 + -5976;
	// 826E3220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3224: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3228: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826E322C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3230: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3234: 396B7434  addi r11, r11, 0x7434
	ctx.r[11].s64 = ctx.r[11].s64 + 29748;
	// 826E3238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E323C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3244: 386A1F6C  addi r3, r10, 0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 8044;
	// 826E3248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E324C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3250: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3254: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3258: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E325C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3260: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3264: 4BD83BBD  bl 0x82466e20
	ctx.lr = 0x826E3268;
	sub_82466E20(ctx, base);
	// 826E3268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E326C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3278 size=112
    let mut pc: u32 = 0x826E3278;
    'dispatch: loop {
        match pc {
            0x826E3278 => {
    //   block [0x826E3278..0x826E32E8)
	// 826E3278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3284: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3288: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E328C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E3290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3294: 390BE938  addi r8, r11, -0x16c8
	ctx.r[8].s64 = ctx.r[11].s64 + -5832;
	// 826E3298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E329C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826E32A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E32A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E32A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E32AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E32B0: 386A1F9C  addi r3, r10, 0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 8092;
	// 826E32B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E32B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E32BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E32C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E32C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E32C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E32CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E32D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E32D4: 4BD83B4D  bl 0x82466e20
	ctx.lr = 0x826E32D8;
	sub_82466E20(ctx, base);
	// 826E32D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E32DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E32E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E32E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E32E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E32E8 size=24
    let mut pc: u32 = 0x826E32E8;
    'dispatch: loop {
        match pc {
            0x826E32E8 => {
    //   block [0x826E32E8..0x826E3300)
	// 826E32E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E32EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E32F0: 394A65A0  addi r10, r10, 0x65a0
	ctx.r[10].s64 = ctx.r[10].s64 + 26016;
	// 826E32F4: 816BE950  lwz r11, -0x16b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5808 as u32) ) } as u64;
	// 826E32F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E32FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3300 size=112
    let mut pc: u32 = 0x826E3300;
    'dispatch: loop {
        match pc {
            0x826E3300 => {
    //   block [0x826E3300..0x826E3370)
	// 826E3300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E330C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3310: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3314: 392A7480  addi r9, r10, 0x7480
	ctx.r[9].s64 = ctx.r[10].s64 + 29824;
	// 826E3318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E331C: 390B65A0  addi r8, r11, 0x65a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26016;
	// 826E3320: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E3324: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826E3328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E332C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3338: 386A1FCC  addi r3, r10, 0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 8140;
	// 826E333C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3340: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E334C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E335C: 4BD83AC5  bl 0x82466e20
	ctx.lr = 0x826E3360;
	sub_82466E20(ctx, base);
	// 826E3360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3370 size=108
    let mut pc: u32 = 0x826E3370;
    'dispatch: loop {
        match pc {
            0x826E3370 => {
    //   block [0x826E3370..0x826E33DC)
	// 826E3370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E337C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3384: 38EBE958  addi r7, r11, -0x16a8
	ctx.r[7].s64 = ctx.r[11].s64 + -5800;
	// 826E3388: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E338C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826E3390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E33A0: 386A1FFC  addi r3, r10, 0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 8188;
	// 826E33A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E33A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E33AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E33B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E33B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E33B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E33BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E33C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E33C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E33C8: 4BD83A59  bl 0x82466e20
	ctx.lr = 0x826E33CC;
	sub_82466E20(ctx, base);
	// 826E33CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E33D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E33D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E33D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E33E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E33E0 size=108
    let mut pc: u32 = 0x826E33E0;
    'dispatch: loop {
        match pc {
            0x826E33E0 => {
    //   block [0x826E33E0..0x826E344C)
	// 826E33E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E33E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E33E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E33EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E33F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E33F4: 38EBE9B8  addi r7, r11, -0x1648
	ctx.r[7].s64 = ctx.r[11].s64 + -5704;
	// 826E33F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E33FC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826E3400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3410: 386A202C  addi r3, r10, 0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + 8236;
	// 826E3414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E341C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E342C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3438: 4BD839E9  bl 0x82466e20
	ctx.lr = 0x826E343C;
	sub_82466E20(ctx, base);
	// 826E343C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3450 size=116
    let mut pc: u32 = 0x826E3450;
    'dispatch: loop {
        match pc {
            0x826E3450 => {
    //   block [0x826E3450..0x826E34C4)
	// 826E3450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E345C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3460: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3464: 390BE9E8  addi r8, r11, -0x1618
	ctx.r[8].s64 = ctx.r[11].s64 + -5656;
	// 826E3468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E346C: 392A74AC  addi r9, r10, 0x74ac
	ctx.r[9].s64 = ctx.r[10].s64 + 29868;
	// 826E3470: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3474: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E3478: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E347C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E348C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3494: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3498: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826E349C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E34A0: 386B205C  addi r3, r11, 0x205c
	ctx.r[3].s64 = ctx.r[11].s64 + 8284;
	// 826E34A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E34A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E34AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E34B0: 4BD83971  bl 0x82466e20
	ctx.lr = 0x826E34B4;
	sub_82466E20(ctx, base);
	// 826E34B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E34B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E34BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E34C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E34C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E34C8 size=108
    let mut pc: u32 = 0x826E34C8;
    'dispatch: loop {
        match pc {
            0x826E34C8 => {
    //   block [0x826E34C8..0x826E3534)
	// 826E34C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E34CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E34D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E34D4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E34D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E34DC: 38EBEA00  addi r7, r11, -0x1600
	ctx.r[7].s64 = ctx.r[11].s64 + -5632;
	// 826E34E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E34E4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826E34E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E34EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E34F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E34F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E34F8: 386A208C  addi r3, r10, 0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + 8332;
	// 826E34FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E350C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E351C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3520: 4BD83901  bl 0x82466e20
	ctx.lr = 0x826E3524;
	sub_82466E20(ctx, base);
	// 826E3524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E352C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3538 size=112
    let mut pc: u32 = 0x826E3538;
    'dispatch: loop {
        match pc {
            0x826E3538 => {
    //   block [0x826E3538..0x826E35A8)
	// 826E3538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3548: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E354C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E3550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3554: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 826E3558: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826E355C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826E3560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E356C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3570: 386A20BC  addi r3, r10, 0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8380;
	// 826E3574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E357C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E358C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3594: 4BD8388D  bl 0x82466e20
	ctx.lr = 0x826E3598;
	sub_82466E20(ctx, base);
	// 826E3598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E359C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E35A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E35A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E35A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E35A8 size=108
    let mut pc: u32 = 0x826E35A8;
    'dispatch: loop {
        match pc {
            0x826E35A8 => {
    //   block [0x826E35A8..0x826E3614)
	// 826E35A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E35AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E35B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E35B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E35B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E35BC: 38EBEAF0  addi r7, r11, -0x1510
	ctx.r[7].s64 = ctx.r[11].s64 + -5392;
	// 826E35C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E35C4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826E35C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E35CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E35D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E35D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E35D8: 386A20EC  addi r3, r10, 0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8428;
	// 826E35DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E35E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E35E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E35E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E35EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E35F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E35F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E35F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E35FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3600: 4BD83821  bl 0x82466e20
	ctx.lr = 0x826E3604;
	sub_82466E20(ctx, base);
	// 826E3604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3618 size=108
    let mut pc: u32 = 0x826E3618;
    'dispatch: loop {
        match pc {
            0x826E3618 => {
    //   block [0x826E3618..0x826E3684)
	// 826E3618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3624: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E362C: 38EBEB68  addi r7, r11, -0x1498
	ctx.r[7].s64 = ctx.r[11].s64 + -5272;
	// 826E3630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E3634: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826E3638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E363C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3648: 386A211C  addi r3, r10, 0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + 8476;
	// 826E364C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E365C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E366C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3670: 4BD837B1  bl 0x82466e20
	ctx.lr = 0x826E3674;
	sub_82466E20(ctx, base);
	// 826E3674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E367C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3688 size=116
    let mut pc: u32 = 0x826E3688;
    'dispatch: loop {
        match pc {
            0x826E3688 => {
    //   block [0x826E3688..0x826E36FC)
	// 826E3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3694: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3698: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826E369C: 390AEBB0  addi r8, r10, -0x1450
	ctx.r[8].s64 = ctx.r[10].s64 + -5200;
	// 826E36A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E36A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E36A8: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E36AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E36B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E36B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E36B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E36BC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826E36C0: 396B74C0  addi r11, r11, 0x74c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29888;
	// 826E36C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E36C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E36CC: 386A214C  addi r3, r10, 0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + 8524;
	// 826E36D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E36D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E36D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E36DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E36E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E36E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E36E8: 4BD83739  bl 0x82466e20
	ctx.lr = 0x826E36EC;
	sub_82466E20(ctx, base);
	// 826E36EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E36F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E36F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E36F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3700 size=112
    let mut pc: u32 = 0x826E3700;
    'dispatch: loop {
        match pc {
            0x826E3700 => {
    //   block [0x826E3700..0x826E3770)
	// 826E3700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E370C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3710: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3714: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3718: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E371C: 390BEDC0  addi r8, r11, -0x1240
	ctx.r[8].s64 = ctx.r[11].s64 + -4672;
	// 826E3720: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E3724: 388AB374  addi r4, r10, -0x4c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -19596;
	// 826E3728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E372C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3738: 386A217C  addi r3, r10, 0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + 8572;
	// 826E373C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E374C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E375C: 4BD836C5  bl 0x82466e20
	ctx.lr = 0x826E3760;
	sub_82466E20(ctx, base);
	// 826E3760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3770 size=76
    let mut pc: u32 = 0x826E3770;
    'dispatch: loop {
        match pc {
            0x826E3770 => {
    //   block [0x826E3770..0x826E37BC)
	// 826E3770: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3774: 814BEDD8  lwz r10, -0x1228(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4648 as u32) ) } as u64;
	// 826E3778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E377C: 396B65D0  addi r11, r11, 0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + 26064;
	// 826E3780: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826E3784: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826E3788: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826E378C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826E3790: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826E3794: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826E3798: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E379C: 814AEDDC  lwz r10, -0x1224(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4644 as u32) ) } as u64;
	// 826E37A0: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826E37A4: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826E37A8: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826E37AC: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826E37B0: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826E37B4: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826E37B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E37C0 size=108
    let mut pc: u32 = 0x826E37C0;
    'dispatch: loop {
        match pc {
            0x826E37C0 => {
    //   block [0x826E37C0..0x826E382C)
	// 826E37C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E37C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E37C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E37CC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E37D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E37D4: 38EB65D0  addi r7, r11, 0x65d0
	ctx.r[7].s64 = ctx.r[11].s64 + 26064;
	// 826E37D8: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 826E37DC: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826E37E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E37E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E37E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E37EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E37F0: 386A21AC  addi r3, r10, 0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8620;
	// 826E37F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E37F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E37FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E380C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3818: 4BD83609  bl 0x82466e20
	ctx.lr = 0x826E381C;
	sub_82466E20(ctx, base);
	// 826E381C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3830 size=76
    let mut pc: u32 = 0x826E3830;
    'dispatch: loop {
        match pc {
            0x826E3830 => {
    //   block [0x826E3830..0x826E387C)
	// 826E3830: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3834: 814BEDD8  lwz r10, -0x1228(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4648 as u32) ) } as u64;
	// 826E3838: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E383C: 396B6840  addi r11, r11, 0x6840
	ctx.r[11].s64 = ctx.r[11].s64 + 26688;
	// 826E3840: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826E3844: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826E3848: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826E384C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826E3850: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826E3854: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826E3858: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E385C: 814AEDDC  lwz r10, -0x1224(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4644 as u32) ) } as u64;
	// 826E3860: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826E3864: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826E3868: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826E386C: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826E3870: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826E3874: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826E3878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3880 size=116
    let mut pc: u32 = 0x826E3880;
    'dispatch: loop {
        match pc {
            0x826E3880 => {
    //   block [0x826E3880..0x826E38F4)
	// 826E3880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E388C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3890: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3894: 390B6840  addi r8, r11, 0x6840
	ctx.r[8].s64 = ctx.r[11].s64 + 26688;
	// 826E3898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E389C: 392A7554  addi r9, r10, 0x7554
	ctx.r[9].s64 = ctx.r[10].s64 + 30036;
	// 826E38A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E38A4: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 826E38A8: 38AA14BC  addi r5, r10, 0x14bc
	ctx.r[5].s64 = ctx.r[10].s64 + 5308;
	// 826E38AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E38B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E38B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E38B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E38BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E38C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E38C4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E38C8: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826E38CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E38D0: 386B21DC  addi r3, r11, 0x21dc
	ctx.r[3].s64 = ctx.r[11].s64 + 8668;
	// 826E38D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E38D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E38DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E38E0: 4BD83541  bl 0x82466e20
	ctx.lr = 0x826E38E4;
	sub_82466E20(ctx, base);
	// 826E38E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E38E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E38EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E38F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E38F8 size=112
    let mut pc: u32 = 0x826E38F8;
    'dispatch: loop {
        match pc {
            0x826E38F8 => {
    //   block [0x826E38F8..0x826E3968)
	// 826E38F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E38FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3908: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E390C: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3910: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3914: 390BEDE0  addi r8, r11, -0x1220
	ctx.r[8].s64 = ctx.r[11].s64 + -4640;
	// 826E3918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E391C: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 826E3920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3924: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E392C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3930: 386A220C  addi r3, r10, 0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + 8716;
	// 826E3934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E393C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3954: 4BD834CD  bl 0x82466e20
	ctx.lr = 0x826E3958;
	sub_82466E20(ctx, base);
	// 826E3958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E395C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3968 size=108
    let mut pc: u32 = 0x826E3968;
    'dispatch: loop {
        match pc {
            0x826E3968 => {
    //   block [0x826E3968..0x826E39D4)
	// 826E3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3974: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E397C: 38EBEE28  addi r7, r11, -0x11d8
	ctx.r[7].s64 = ctx.r[11].s64 + -4568;
	// 826E3980: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E3984: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 826E3988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E398C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3998: 386A223C  addi r3, r10, 0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + 8764;
	// 826E399C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E39A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E39A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E39A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E39AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E39B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E39B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E39B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E39BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E39C0: 4BD83461  bl 0x82466e20
	ctx.lr = 0x826E39C4;
	sub_82466E20(ctx, base);
	// 826E39C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E39C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E39CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E39D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E39D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E39D8 size=108
    let mut pc: u32 = 0x826E39D8;
    'dispatch: loop {
        match pc {
            0x826E39D8 => {
    //   block [0x826E39D8..0x826E3A44)
	// 826E39D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E39DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E39E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E39E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E39E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E39EC: 38EBEE70  addi r7, r11, -0x1190
	ctx.r[7].s64 = ctx.r[11].s64 + -4496;
	// 826E39F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E39F4: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 826E39F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E39FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3A08: 386A226C  addi r3, r10, 0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + 8812;
	// 826E3A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3A30: 4BD833F1  bl 0x82466e20
	ctx.lr = 0x826E3A34;
	sub_82466E20(ctx, base);
	// 826E3A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3A48 size=116
    let mut pc: u32 = 0x826E3A48;
    'dispatch: loop {
        match pc {
            0x826E3A48 => {
    //   block [0x826E3A48..0x826E3ABC)
	// 826E3A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3A54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3A58: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E3A5C: 390AEEB8  addi r8, r10, -0x1148
	ctx.r[8].s64 = ctx.r[10].s64 + -4424;
	// 826E3A60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3A68: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3A6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3A70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3A7C: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 826E3A80: 396B757C  addi r11, r11, 0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + 30076;
	// 826E3A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3A8C: 386A229C  addi r3, r10, 0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + 8860;
	// 826E3A90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3A94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3A98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3AA8: 4BD83379  bl 0x82466e20
	ctx.lr = 0x826E3AAC;
	sub_82466E20(ctx, base);
	// 826E3AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3AC0 size=116
    let mut pc: u32 = 0x826E3AC0;
    'dispatch: loop {
        match pc {
            0x826E3AC0 => {
    //   block [0x826E3AC0..0x826E3B34)
	// 826E3AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3ACC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3AD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E3AD4: 390AEF60  addi r8, r10, -0x10a0
	ctx.r[8].s64 = ctx.r[10].s64 + -4256;
	// 826E3AD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3ADC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3AE0: 38AA229C  addi r5, r10, 0x229c
	ctx.r[5].s64 = ctx.r[10].s64 + 8860;
	// 826E3AE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3AE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3AF4: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 826E3AF8: 396B75A0  addi r11, r11, 0x75a0
	ctx.r[11].s64 = ctx.r[11].s64 + 30112;
	// 826E3AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3B00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3B04: 386A22CC  addi r3, r10, 0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8908;
	// 826E3B08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3B0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3B10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3B20: 4BD83301  bl 0x82466e20
	ctx.lr = 0x826E3B24;
	sub_82466E20(ctx, base);
	// 826E3B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3B38 size=36
    let mut pc: u32 = 0x826E3B38;
    'dispatch: loop {
        match pc {
            0x826E3B38 => {
    //   block [0x826E3B38..0x826E3B5C)
	// 826E3B38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B3C: 814BEFA8  lwz r10, -0x1058(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4184 as u32) ) } as u64;
	// 826E3B40: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B44: 396B6C00  addi r11, r11, 0x6c00
	ctx.r[11].s64 = ctx.r[11].s64 + 27648;
	// 826E3B48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E3B4C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3B50: 814AEFAC  lwz r10, -0x1054(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4180 as u32) ) } as u64;
	// 826E3B54: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826E3B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3B60 size=116
    let mut pc: u32 = 0x826E3B60;
    'dispatch: loop {
        match pc {
            0x826E3B60 => {
    //   block [0x826E3B60..0x826E3BD4)
	// 826E3B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3B6C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3B74: 390B6C00  addi r8, r11, 0x6c00
	ctx.r[8].s64 = ctx.r[11].s64 + 27648;
	// 826E3B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3B7C: 392A75E0  addi r9, r10, 0x75e0
	ctx.r[9].s64 = ctx.r[10].s64 + 30176;
	// 826E3B80: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3B84: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E3B88: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3B8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3B94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3BA4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3BA8: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 826E3BAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3BB0: 386B22FC  addi r3, r11, 0x22fc
	ctx.r[3].s64 = ctx.r[11].s64 + 8956;
	// 826E3BB4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E3BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3BC0: 4BD83261  bl 0x82466e20
	ctx.lr = 0x826E3BC4;
	sub_82466E20(ctx, base);
	// 826E3BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3BD8 size=112
    let mut pc: u32 = 0x826E3BD8;
    'dispatch: loop {
        match pc {
            0x826E3BD8 => {
    //   block [0x826E3BD8..0x826E3C48)
	// 826E3BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3BE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3BEC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3BF4: 390BEFB0  addi r8, r11, -0x1050
	ctx.r[8].s64 = ctx.r[11].s64 + -4176;
	// 826E3BF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826E3BFC: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 826E3C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3C04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3C10: 386A232C  addi r3, r10, 0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + 9004;
	// 826E3C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3C34: 4BD831ED  bl 0x82466e20
	ctx.lr = 0x826E3C38;
	sub_82466E20(ctx, base);
	// 826E3C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3C48 size=108
    let mut pc: u32 = 0x826E3C48;
    'dispatch: loop {
        match pc {
            0x826E3C48 => {
    //   block [0x826E3C48..0x826E3CB4)
	// 826E3C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3C54: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3C5C: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 826E3C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E3C64: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 826E3C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3C6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3C78: 386A235C  addi r3, r10, 0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + 9052;
	// 826E3C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3CA0: 4BD83181  bl 0x82466e20
	ctx.lr = 0x826E3CA4;
	sub_82466E20(ctx, base);
	// 826E3CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3CB8 size=112
    let mut pc: u32 = 0x826E3CB8;
    'dispatch: loop {
        match pc {
            0x826E3CB8 => {
    //   block [0x826E3CB8..0x826E3D28)
	// 826E3CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3CC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3CC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3CCC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3CD4: 390BF0A0  addi r8, r11, -0xf60
	ctx.r[8].s64 = ctx.r[11].s64 + -3936;
	// 826E3CD8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826E3CDC: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 826E3CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3CE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3CF0: 386A238C  addi r3, r10, 0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + 9100;
	// 826E3CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3D14: 4BD8310D  bl 0x82466e20
	ctx.lr = 0x826E3D18;
	sub_82466E20(ctx, base);
	// 826E3D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3D28 size=112
    let mut pc: u32 = 0x826E3D28;
    'dispatch: loop {
        match pc {
            0x826E3D28 => {
    //   block [0x826E3D28..0x826E3D98)
	// 826E3D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3D34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3D38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3D3C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3D44: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 826E3D48: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826E3D4C: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 826E3D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3D54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3D60: 386A23BC  addi r3, r10, 0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9148;
	// 826E3D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3D84: 4BD8309D  bl 0x82466e20
	ctx.lr = 0x826E3D88;
	sub_82466E20(ctx, base);
	// 826E3D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3D98 size=116
    let mut pc: u32 = 0x826E3D98;
    'dispatch: loop {
        match pc {
            0x826E3D98 => {
    //   block [0x826E3D98..0x826E3E0C)
	// 826E3D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3DA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3DA8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3DAC: 390BF2E8  addi r8, r11, -0xd18
	ctx.r[8].s64 = ctx.r[11].s64 + -3352;
	// 826E3DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3DB4: 392A7618  addi r9, r10, 0x7618
	ctx.r[9].s64 = ctx.r[10].s64 + 30232;
	// 826E3DB8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3DBC: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 826E3DC0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3DC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3DCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3DDC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3DE0: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 826E3DE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3DE8: 386B23EC  addi r3, r11, 0x23ec
	ctx.r[3].s64 = ctx.r[11].s64 + 9196;
	// 826E3DEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3DF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3DF8: 4BD83029  bl 0x82466e20
	ctx.lr = 0x826E3DFC;
	sub_82466E20(ctx, base);
	// 826E3DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3E10 size=112
    let mut pc: u32 = 0x826E3E10;
    'dispatch: loop {
        match pc {
            0x826E3E10 => {
    //   block [0x826E3E10..0x826E3E80)
	// 826E3E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3E24: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3E28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3E2C: 390BF5A0  addi r8, r11, -0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + -2656;
	// 826E3E30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E3E34: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 826E3E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3E48: 386A241C  addi r3, r10, 0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + 9244;
	// 826E3E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3E6C: 4BD82FB5  bl 0x82466e20
	ctx.lr = 0x826E3E70;
	sub_82466E20(ctx, base);
	// 826E3E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3E80 size=116
    let mut pc: u32 = 0x826E3E80;
    'dispatch: loop {
        match pc {
            0x826E3E80 => {
    //   block [0x826E3E80..0x826E3EF4)
	// 826E3E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3E8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3E90: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E3E94: 390AF5E8  addi r8, r10, -0xa18
	ctx.r[8].s64 = ctx.r[10].s64 + -2584;
	// 826E3E98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3EA0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3EA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3EA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3EB4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826E3EB8: 396B762C  addi r11, r11, 0x762c
	ctx.r[11].s64 = ctx.r[11].s64 + 30252;
	// 826E3EBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3EC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3EC4: 386A244C  addi r3, r10, 0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + 9292;
	// 826E3EC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3ED0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3EE0: 4BD82F41  bl 0x82466e20
	ctx.lr = 0x826E3EE4;
	sub_82466E20(ctx, base);
	// 826E3EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3EF8 size=112
    let mut pc: u32 = 0x826E3EF8;
    'dispatch: loop {
        match pc {
            0x826E3EF8 => {
    //   block [0x826E3EF8..0x826E3F68)
	// 826E3EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3F0C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3F10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3F14: 390BF630  addi r8, r11, -0x9d0
	ctx.r[8].s64 = ctx.r[11].s64 + -2512;
	// 826E3F18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E3F1C: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 826E3F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3F30: 386A247C  addi r3, r10, 0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + 9340;
	// 826E3F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3F54: 4BD82ECD  bl 0x82466e20
	ctx.lr = 0x826E3F58;
	sub_82466E20(ctx, base);
	// 826E3F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3F68 size=112
    let mut pc: u32 = 0x826E3F68;
    'dispatch: loop {
        match pc {
            0x826E3F68 => {
    //   block [0x826E3F68..0x826E3FD8)
	// 826E3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3F74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3F7C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3F80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3F84: 390BF6C0  addi r8, r11, -0x940
	ctx.r[8].s64 = ctx.r[11].s64 + -2368;
	// 826E3F88: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E3F8C: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 826E3F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3FA0: 386A24AC  addi r3, r10, 0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9388;
	// 826E3FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3FC4: 4BD82E5D  bl 0x82466e20
	ctx.lr = 0x826E3FC8;
	sub_82466E20(ctx, base);
	// 826E3FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3FD8 size=24
    let mut pc: u32 = 0x826E3FD8;
    'dispatch: loop {
        match pc {
            0x826E3FD8 => {
    //   block [0x826E3FD8..0x826E3FF0)
	// 826E3FD8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3FDC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3FE0: 394A6D08  addi r10, r10, 0x6d08
	ctx.r[10].s64 = ctx.r[10].s64 + 27912;
	// 826E3FE4: 816BF2E4  lwz r11, -0xd1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3356 as u32) ) } as u64;
	// 826E3FE8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826E3FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3FF0 size=116
    let mut pc: u32 = 0x826E3FF0;
    'dispatch: loop {
        match pc {
            0x826E3FF0 => {
    //   block [0x826E3FF0..0x826E4064)
	// 826E3FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3FFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E4000: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4004: 392B7658  addi r9, r11, 0x7658
	ctx.r[9].s64 = ctx.r[11].s64 + 30296;
	// 826E4008: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E400C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4010: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E4014: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826E4018: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E401C: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826E4020: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4024: 396B6D08  addi r11, r11, 0x6d08
	ctx.r[11].s64 = ctx.r[11].s64 + 27912;
	// 826E4028: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E402C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4030: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E4034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4038: 386A24DC  addi r3, r10, 0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9436;
	// 826E403C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4040: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E4044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4048: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E404C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E4050: 4BD82DD1  bl 0x82466e20
	ctx.lr = 0x826E4054;
	sub_82466E20(ctx, base);
	// 826E4054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E405C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4068 size=112
    let mut pc: u32 = 0x826E4068;
    'dispatch: loop {
        match pc {
            0x826E4068 => {
    //   block [0x826E4068..0x826E40D8)
	// 826E4068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4078: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E407C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E4080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4084: 390BF738  addi r8, r11, -0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + -2248;
	// 826E4088: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E408C: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 826E4090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E409C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E40A0: 386A250C  addi r3, r10, 0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + 9484;
	// 826E40A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E40A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E40AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E40B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E40B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E40B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E40BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E40C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E40C4: 4BD82D5D  bl 0x82466e20
	ctx.lr = 0x826E40C8;
	sub_82466E20(ctx, base);
	// 826E40C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E40CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E40D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E40D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E40D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E40D8 size=24
    let mut pc: u32 = 0x826E40D8;
    'dispatch: loop {
        match pc {
            0x826E40D8 => {
    //   block [0x826E40D8..0x826E40F0)
	// 826E40D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E40DC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E40E0: 394A6DB0  addi r10, r10, 0x6db0
	ctx.r[10].s64 = ctx.r[10].s64 + 28080;
	// 826E40E4: 816BF7B0  lwz r11, -0x850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2128 as u32) ) } as u64;
	// 826E40E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E40EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E40F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E40F0 size=116
    let mut pc: u32 = 0x826E40F0;
    'dispatch: loop {
        match pc {
            0x826E40F0 => {
    //   block [0x826E40F0..0x826E4164)
	// 826E40F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E40F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E40F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E40FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4100: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4104: 390B6DB0  addi r8, r11, 0x6db0
	ctx.r[8].s64 = ctx.r[11].s64 + 28080;
	// 826E4108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E410C: 392A76C0  addi r9, r10, 0x76c0
	ctx.r[9].s64 = ctx.r[10].s64 + 30400;
	// 826E4110: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4114: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E4118: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E411C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4124: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E412C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4134: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E4138: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826E413C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4140: 386B253C  addi r3, r11, 0x253c
	ctx.r[3].s64 = ctx.r[11].s64 + 9532;
	// 826E4144: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4148: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E414C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4150: 4BD82CD1  bl 0x82466e20
	ctx.lr = 0x826E4154;
	sub_82466E20(ctx, base);
	// 826E4154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E415C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4168 size=112
    let mut pc: u32 = 0x826E4168;
    'dispatch: loop {
        match pc {
            0x826E4168 => {
    //   block [0x826E4168..0x826E41D8)
	// 826E4168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E416C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4174: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4178: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E417C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E4180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4184: 390BF7B8  addi r8, r11, -0x848
	ctx.r[8].s64 = ctx.r[11].s64 + -2120;
	// 826E4188: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826E418C: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 826E4190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E419C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E41A0: 386A256C  addi r3, r10, 0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + 9580;
	// 826E41A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E41A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E41AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E41B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E41B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E41B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E41BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E41C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E41C4: 4BD82C5D  bl 0x82466e20
	ctx.lr = 0x826E41C8;
	sub_82466E20(ctx, base);
	// 826E41C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E41CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E41D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E41D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E41D8 size=112
    let mut pc: u32 = 0x826E41D8;
    'dispatch: loop {
        match pc {
            0x826E41D8 => {
    //   block [0x826E41D8..0x826E4248)
	// 826E41D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E41DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E41E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E41E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E41E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E41EC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E41F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E41F4: 390BF8A8  addi r8, r11, -0x758
	ctx.r[8].s64 = ctx.r[11].s64 + -1880;
	// 826E41F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E41FC: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 826E4200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E420C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4210: 386A259C  addi r3, r10, 0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + 9628;
	// 826E4214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E421C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E422C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4234: 4BD82BED  bl 0x82466e20
	ctx.lr = 0x826E4238;
	sub_82466E20(ctx, base);
	// 826E4238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E423C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E4248 size=24
    let mut pc: u32 = 0x826E4248;
    'dispatch: loop {
        match pc {
            0x826E4248 => {
    //   block [0x826E4248..0x826E4260)
	// 826E4248: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E424C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4250: 394A6E58  addi r10, r10, 0x6e58
	ctx.r[10].s64 = ctx.r[10].s64 + 28248;
	// 826E4254: 816BF7B4  lwz r11, -0x84c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2124 as u32) ) } as u64;
	// 826E4258: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826E425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4260 size=116
    let mut pc: u32 = 0x826E4260;
    'dispatch: loop {
        match pc {
            0x826E4260 => {
    //   block [0x826E4260..0x826E42D4)
	// 826E4260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E426C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4270: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4274: 390B6E58  addi r8, r11, 0x6e58
	ctx.r[8].s64 = ctx.r[11].s64 + 28248;
	// 826E4278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E427C: 392A770C  addi r9, r10, 0x770c
	ctx.r[9].s64 = ctx.r[10].s64 + 30476;
	// 826E4280: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4284: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 826E4288: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E428C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4294: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E429C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E42A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E42A4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E42A8: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826E42AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E42B0: 386B25CC  addi r3, r11, 0x25cc
	ctx.r[3].s64 = ctx.r[11].s64 + 9676;
	// 826E42B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E42B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E42BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E42C0: 4BD82B61  bl 0x82466e20
	ctx.lr = 0x826E42C4;
	sub_82466E20(ctx, base);
	// 826E42C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E42C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E42CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E42D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E42D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E42D8 size=116
    let mut pc: u32 = 0x826E42D8;
    'dispatch: loop {
        match pc {
            0x826E42D8 => {
    //   block [0x826E42D8..0x826E434C)
	// 826E42D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E42DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E42E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E42E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E42E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E42EC: 392B7744  addi r9, r11, 0x7744
	ctx.r[9].s64 = ctx.r[11].s64 + 30532;
	// 826E42F0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E42F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E42F8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E42FC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 826E4300: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4304: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 826E4308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E430C: 396BF910  addi r11, r11, -0x6f0
	ctx.r[11].s64 = ctx.r[11].s64 + -1776;
	// 826E4310: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E4314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4318: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E431C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4320: 386A25FC  addi r3, r10, 0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9724;
	// 826E4324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4328: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E432C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4330: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E4334: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E4338: 4BD82AE9  bl 0x82466e20
	ctx.lr = 0x826E433C;
	sub_82466E20(ctx, base);
	// 826E433C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4350 size=112
    let mut pc: u32 = 0x826E4350;
    'dispatch: loop {
        match pc {
            0x826E4350 => {
    //   block [0x826E4350..0x826E43C0)
	// 826E4350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E435C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4360: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4364: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E4368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E436C: 390BFAD8  addi r8, r11, -0x528
	ctx.r[8].s64 = ctx.r[11].s64 + -1320;
	// 826E4370: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E4374: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 826E4378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E437C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4388: 386A262C  addi r3, r10, 0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + 9772;
	// 826E438C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E439C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E43A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E43A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E43A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E43AC: 4BD82A75  bl 0x82466e20
	ctx.lr = 0x826E43B0;
	sub_82466E20(ctx, base);
	// 826E43B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E43B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E43B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E43BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


