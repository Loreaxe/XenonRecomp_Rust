pub fn sub_826ACBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACBC0 size=100
    let mut pc: u32 = 0x826ACBC0;
    'dispatch: loop {
        match pc {
            0x826ACBC0 => {
    //   block [0x826ACBC0..0x826ACC24)
	// 826ACBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACBCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACBD4: 38AAB0A8  addi r5, r10, -0x4f58
	ctx.r[5].s64 = ctx.r[10].s64 + -20312;
	// 826ACBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACBE0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826ACBE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACBF4: 386AB078  addi r3, r10, -0x4f88
	ctx.r[3].s64 = ctx.r[10].s64 + -20360;
	// 826ACBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACC10: 4BDBA211  bl 0x82466e20
	ctx.lr = 0x826ACC14;
	sub_82466E20(ctx, base);
	// 826ACC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACC28 size=100
    let mut pc: u32 = 0x826ACC28;
    'dispatch: loop {
        match pc {
            0x826ACC28 => {
    //   block [0x826ACC28..0x826ACC8C)
	// 826ACC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACC34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACC3C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACC48: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 826ACC4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACC5C: 386AB0A8  addi r3, r10, -0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + -20312;
	// 826ACC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACC78: 4BDBA1A9  bl 0x82466e20
	ctx.lr = 0x826ACC7C;
	sub_82466E20(ctx, base);
	// 826ACC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACC90 size=100
    let mut pc: u32 = 0x826ACC90;
    'dispatch: loop {
        match pc {
            0x826ACC90 => {
    //   block [0x826ACC90..0x826ACCF4)
	// 826ACC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACC9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACCA4: 38AAB0A8  addi r5, r10, -0x4f58
	ctx.r[5].s64 = ctx.r[10].s64 + -20312;
	// 826ACCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACCB0: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 826ACCB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACCC4: 386AB0D8  addi r3, r10, -0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + -20264;
	// 826ACCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACCE0: 4BDBA141  bl 0x82466e20
	ctx.lr = 0x826ACCE4;
	sub_82466E20(ctx, base);
	// 826ACCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACCF8 size=100
    let mut pc: u32 = 0x826ACCF8;
    'dispatch: loop {
        match pc {
            0x826ACCF8 => {
    //   block [0x826ACCF8..0x826ACD5C)
	// 826ACCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACD04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACD0C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACD10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACD18: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 826ACD1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACD2C: 386AB108  addi r3, r10, -0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -20216;
	// 826ACD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACD34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACD48: 4BDBA0D9  bl 0x82466e20
	ctx.lr = 0x826ACD4C;
	sub_82466E20(ctx, base);
	// 826ACD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACD60 size=100
    let mut pc: u32 = 0x826ACD60;
    'dispatch: loop {
        match pc {
            0x826ACD60 => {
    //   block [0x826ACD60..0x826ACDC4)
	// 826ACD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACD6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACD74: 38AAB018  addi r5, r10, -0x4fe8
	ctx.r[5].s64 = ctx.r[10].s64 + -20456;
	// 826ACD78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACD80: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 826ACD84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACD94: 386AB138  addi r3, r10, -0x4ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -20168;
	// 826ACD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACD9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACDA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACDA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACDB0: 4BDBA071  bl 0x82466e20
	ctx.lr = 0x826ACDB4;
	sub_82466E20(ctx, base);
	// 826ACDB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACDB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACDBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACDC8 size=100
    let mut pc: u32 = 0x826ACDC8;
    'dispatch: loop {
        match pc {
            0x826ACDC8 => {
    //   block [0x826ACDC8..0x826ACE2C)
	// 826ACDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACDD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACDDC: 38AAB108  addi r5, r10, -0x4ef8
	ctx.r[5].s64 = ctx.r[10].s64 + -20216;
	// 826ACDE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACDE8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 826ACDEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACDFC: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 826ACE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACE04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACE08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACE10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACE18: 4BDBA009  bl 0x82466e20
	ctx.lr = 0x826ACE1C;
	sub_82466E20(ctx, base);
	// 826ACE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACE30 size=100
    let mut pc: u32 = 0x826ACE30;
    'dispatch: loop {
        match pc {
            0x826ACE30 => {
    //   block [0x826ACE30..0x826ACE94)
	// 826ACE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACE44: 38AAB018  addi r5, r10, -0x4fe8
	ctx.r[5].s64 = ctx.r[10].s64 + -20456;
	// 826ACE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACE50: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 826ACE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACE64: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 826ACE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACE6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACE70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACE78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACE80: 4BDB9FA1  bl 0x82466e20
	ctx.lr = 0x826ACE84;
	sub_82466E20(ctx, base);
	// 826ACE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACE98 size=112
    let mut pc: u32 = 0x826ACE98;
    'dispatch: loop {
        match pc {
            0x826ACE98 => {
    //   block [0x826ACE98..0x826ACF08)
	// 826ACE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACEA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACEAC: 38AAB228  addi r5, r10, -0x4dd8
	ctx.r[5].s64 = ctx.r[10].s64 + -19928;
	// 826ACEB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACEB4: 390BF728  addi r8, r11, -0x8d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2264;
	// 826ACEB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ACEBC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 826ACEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACEC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACED0: 386AB1C8  addi r3, r10, -0x4e38
	ctx.r[3].s64 = ctx.r[10].s64 + -20024;
	// 826ACED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACEF4: 4BDB9F2D  bl 0x82466e20
	ctx.lr = 0x826ACEF8;
	sub_82466E20(ctx, base);
	// 826ACEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACF08 size=112
    let mut pc: u32 = 0x826ACF08;
    'dispatch: loop {
        match pc {
            0x826ACF08 => {
    //   block [0x826ACF08..0x826ACF78)
	// 826ACF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACF14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACF1C: 38AAB258  addi r5, r10, -0x4da8
	ctx.r[5].s64 = ctx.r[10].s64 + -19880;
	// 826ACF20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACF24: 390BF758  addi r8, r11, -0x8a8
	ctx.r[8].s64 = ctx.r[11].s64 + -2216;
	// 826ACF28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ACF2C: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 826ACF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACF34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACF40: 386AB1F8  addi r3, r10, -0x4e08
	ctx.r[3].s64 = ctx.r[10].s64 + -19976;
	// 826ACF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACF64: 4BDB9EBD  bl 0x82466e20
	ctx.lr = 0x826ACF68;
	sub_82466E20(ctx, base);
	// 826ACF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACF78 size=112
    let mut pc: u32 = 0x826ACF78;
    'dispatch: loop {
        match pc {
            0x826ACF78 => {
    //   block [0x826ACF78..0x826ACFE8)
	// 826ACF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACF84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACF8C: 38AAB378  addi r5, r10, -0x4c88
	ctx.r[5].s64 = ctx.r[10].s64 + -19592;
	// 826ACF90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACF94: 390BF770  addi r8, r11, -0x890
	ctx.r[8].s64 = ctx.r[11].s64 + -2192;
	// 826ACF98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ACF9C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 826ACFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACFA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACFB0: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 826ACFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACFD4: 4BDB9E4D  bl 0x82466e20
	ctx.lr = 0x826ACFD8;
	sub_82466E20(ctx, base);
	// 826ACFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACFE8 size=112
    let mut pc: u32 = 0x826ACFE8;
    'dispatch: loop {
        match pc {
            0x826ACFE8 => {
    //   block [0x826ACFE8..0x826AD058)
	// 826ACFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACFF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACFF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACFFC: 38AAB228  addi r5, r10, -0x4dd8
	ctx.r[5].s64 = ctx.r[10].s64 + -19928;
	// 826AD000: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD004: 390BF7A0  addi r8, r11, -0x860
	ctx.r[8].s64 = ctx.r[11].s64 + -2144;
	// 826AD008: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD00C: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 826AD010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD014: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD020: 386AB258  addi r3, r10, -0x4da8
	ctx.r[3].s64 = ctx.r[10].s64 + -19880;
	// 826AD024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD044: 4BDB9DDD  bl 0x82466e20
	ctx.lr = 0x826AD048;
	sub_82466E20(ctx, base);
	// 826AD048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD058 size=108
    let mut pc: u32 = 0x826AD058;
    'dispatch: loop {
        match pc {
            0x826AD058 => {
    //   block [0x826AD058..0x826AD0C4)
	// 826AD058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD064: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD06C: 38EBF7B8  addi r7, r11, -0x848
	ctx.r[7].s64 = ctx.r[11].s64 + -2120;
	// 826AD070: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD074: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 826AD078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD07C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD088: 386AB288  addi r3, r10, -0x4d78
	ctx.r[3].s64 = ctx.r[10].s64 + -19832;
	// 826AD08C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD0B0: 4BDB9D71  bl 0x82466e20
	ctx.lr = 0x826AD0B4;
	sub_82466E20(ctx, base);
	// 826AD0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD0C8 size=112
    let mut pc: u32 = 0x826AD0C8;
    'dispatch: loop {
        match pc {
            0x826AD0C8 => {
    //   block [0x826AD0C8..0x826AD138)
	// 826AD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD0D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD0DC: 38AAB258  addi r5, r10, -0x4da8
	ctx.r[5].s64 = ctx.r[10].s64 + -19880;
	// 826AD0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD0E4: 390BF7D0  addi r8, r11, -0x830
	ctx.r[8].s64 = ctx.r[11].s64 + -2096;
	// 826AD0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD0EC: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 826AD0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD0F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD100: 386AB2B8  addi r3, r10, -0x4d48
	ctx.r[3].s64 = ctx.r[10].s64 + -19784;
	// 826AD104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD124: 4BDB9CFD  bl 0x82466e20
	ctx.lr = 0x826AD128;
	sub_82466E20(ctx, base);
	// 826AD128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD138 size=116
    let mut pc: u32 = 0x826AD138;
    'dispatch: loop {
        match pc {
            0x826AD138 => {
    //   block [0x826AD138..0x826AD1AC)
	// 826AD138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD144: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD148: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AD14C: 390AF7E8  addi r8, r10, -0x818
	ctx.r[8].s64 = ctx.r[10].s64 + -2072;
	// 826AD150: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD154: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AD158: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD15C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD160: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AD164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD16C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 826AD170: 396BD63C  addi r11, r11, -0x29c4
	ctx.r[11].s64 = ctx.r[11].s64 + -10692;
	// 826AD174: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD17C: 386AB2E8  addi r3, r10, -0x4d18
	ctx.r[3].s64 = ctx.r[10].s64 + -19736;
	// 826AD180: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AD184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD188: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AD18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD198: 4BDB9C89  bl 0x82466e20
	ctx.lr = 0x826AD19C;
	sub_82466E20(ctx, base);
	// 826AD19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD1B0 size=116
    let mut pc: u32 = 0x826AD1B0;
    'dispatch: loop {
        match pc {
            0x826AD1B0 => {
    //   block [0x826AD1B0..0x826AD224)
	// 826AD1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD1BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD1C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AD1C4: 392AD764  addi r9, r10, -0x289c
	ctx.r[9].s64 = ctx.r[10].s64 + -10396;
	// 826AD1C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD1CC: 38C00045  li r6, 0x45
	ctx.r[6].s64 = 69;
	// 826AD1D0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD1D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD1D8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826AD1DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD1E0: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 826AD1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD1E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD1EC: 396BF8B0  addi r11, r11, -0x750
	ctx.r[11].s64 = ctx.r[11].s64 + -1872;
	// 826AD1F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD1F8: 386AB318  addi r3, r10, -0x4ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -19688;
	// 826AD1FC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826AD200: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD208: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826AD20C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD210: 4BDB9C11  bl 0x82466e20
	ctx.lr = 0x826AD214;
	sub_82466E20(ctx, base);
	// 826AD214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AD228 size=48
    let mut pc: u32 = 0x826AD228;
    'dispatch: loop {
        match pc {
            0x826AD228 => {
    //   block [0x826AD228..0x826AD258)
	// 826AD228: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD22C: 814BFF30  lwz r10, -0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-208 as u32) ) } as u64;
	// 826AD230: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD234: 396B4558  addi r11, r11, 0x4558
	ctx.r[11].s64 = ctx.r[11].s64 + 17752;
	// 826AD238: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826AD23C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD240: 814AFF2C  lwz r10, -0xd4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-212 as u32) ) } as u64;
	// 826AD244: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826AD248: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD24C: 814AFF28  lwz r10, -0xd8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-216 as u32) ) } as u64;
	// 826AD250: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 826AD254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD258 size=116
    let mut pc: u32 = 0x826AD258;
    'dispatch: loop {
        match pc {
            0x826AD258 => {
    //   block [0x826AD258..0x826AD2CC)
	// 826AD258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD264: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AD268: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD26C: 392BD840  addi r9, r11, -0x27c0
	ctx.r[9].s64 = ctx.r[11].s64 + -10176;
	// 826AD270: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD274: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD278: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826AD27C: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 826AD280: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD284: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 826AD288: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD28C: 396B4558  addi r11, r11, 0x4558
	ctx.r[11].s64 = ctx.r[11].s64 + 17752;
	// 826AD290: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AD294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD298: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD2A0: 386AB348  addi r3, r10, -0x4cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -19640;
	// 826AD2A4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826AD2A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AD2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD2B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD2B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD2B8: 4BDB9B69  bl 0x82466e20
	ctx.lr = 0x826AD2BC;
	sub_82466E20(ctx, base);
	// 826AD2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD2D0 size=116
    let mut pc: u32 = 0x826AD2D0;
    'dispatch: loop {
        match pc {
            0x826AD2D0 => {
    //   block [0x826AD2D0..0x826AD344)
	// 826AD2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD2DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD2E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD2E4: 390BFF40  addi r8, r11, -0xc0
	ctx.r[8].s64 = ctx.r[11].s64 + -192;
	// 826AD2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD2EC: 392AD9F0  addi r9, r10, -0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + -9744;
	// 826AD2F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD2F4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AD2F8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD2FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD304: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD314: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AD318: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 826AD31C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AD320: 386BB378  addi r3, r11, -0x4c88
	ctx.r[3].s64 = ctx.r[11].s64 + -19592;
	// 826AD324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AD328: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD330: 4BDB9AF1  bl 0x82466e20
	ctx.lr = 0x826AD334;
	sub_82466E20(ctx, base);
	// 826AD334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD348 size=112
    let mut pc: u32 = 0x826AD348;
    'dispatch: loop {
        match pc {
            0x826AD348 => {
    //   block [0x826AD348..0x826AD3B8)
	// 826AD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD358: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD35C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD364: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 826AD368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD36C: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 826AD370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD380: 386AB3A8  addi r3, r10, -0x4c58
	ctx.r[3].s64 = ctx.r[10].s64 + -19544;
	// 826AD384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD3A4: 4BDB9A7D  bl 0x82466e20
	ctx.lr = 0x826AD3A8;
	sub_82466E20(ctx, base);
	// 826AD3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AD3B8 size=36
    let mut pc: u32 = 0x826AD3B8;
    'dispatch: loop {
        match pc {
            0x826AD3B8 => {
    //   block [0x826AD3B8..0x826AD3DC)
	// 826AD3B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD3BC: 814BFFEC  lwz r10, -0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 826AD3C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD3C4: 396B4948  addi r11, r11, 0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + 18760;
	// 826AD3C8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826AD3CC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD3D0: 814AFF3C  lwz r10, -0xc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-196 as u32) ) } as u64;
	// 826AD3D4: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826AD3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD3E0 size=116
    let mut pc: u32 = 0x826AD3E0;
    'dispatch: loop {
        match pc {
            0x826AD3E0 => {
    //   block [0x826AD3E0..0x826AD454)
	// 826AD3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD3EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD3F0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AD3F4: 392ADA58  addi r9, r10, -0x25a8
	ctx.r[9].s64 = ctx.r[10].s64 + -9640;
	// 826AD3F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD3FC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826AD400: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD404: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD408: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826AD40C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD410: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 826AD414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD418: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD41C: 396B4948  addi r11, r11, 0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + 18760;
	// 826AD420: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD428: 386AB3D8  addi r3, r10, -0x4c28
	ctx.r[3].s64 = ctx.r[10].s64 + -19496;
	// 826AD42C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826AD430: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD438: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826AD43C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD440: 4BDB99E1  bl 0x82466e20
	ctx.lr = 0x826AD444;
	sub_82466E20(ctx, base);
	// 826AD444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD458 size=108
    let mut pc: u32 = 0x826AD458;
    'dispatch: loop {
        match pc {
            0x826AD458 => {
    //   block [0x826AD458..0x826AD4C4)
	// 826AD458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD464: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD46C: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 826AD470: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AD474: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 826AD478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD47C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD488: 386AB408  addi r3, r10, -0x4bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -19448;
	// 826AD48C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD4B0: 4BDB9971  bl 0x82466e20
	ctx.lr = 0x826AD4B4;
	sub_82466E20(ctx, base);
	// 826AD4B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD4C8 size=112
    let mut pc: u32 = 0x826AD4C8;
    'dispatch: loop {
        match pc {
            0x826AD4C8 => {
    //   block [0x826AD4C8..0x826AD538)
	// 826AD4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD4D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD4D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD4DC: 38AA9188  addi r5, r10, -0x6e78
	ctx.r[5].s64 = ctx.r[10].s64 + -28280;
	// 826AD4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD4E4: 390B0068  addi r8, r11, 0x68
	ctx.r[8].s64 = ctx.r[11].s64 + 104;
	// 826AD4E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD4EC: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826AD4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD500: 386AB438  addi r3, r10, -0x4bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -19400;
	// 826AD504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD524: 4BDB98FD  bl 0x82466e20
	ctx.lr = 0x826AD528;
	sub_82466E20(ctx, base);
	// 826AD528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD538 size=108
    let mut pc: u32 = 0x826AD538;
    'dispatch: loop {
        match pc {
            0x826AD538 => {
    //   block [0x826AD538..0x826AD5A4)
	// 826AD538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD544: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD54C: 38EB0080  addi r7, r11, 0x80
	ctx.r[7].s64 = ctx.r[11].s64 + 128;
	// 826AD550: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD554: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 826AD558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD55C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD568: 386AB468  addi r3, r10, -0x4b98
	ctx.r[3].s64 = ctx.r[10].s64 + -19352;
	// 826AD56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD590: 4BDB9891  bl 0x82466e20
	ctx.lr = 0x826AD594;
	sub_82466E20(ctx, base);
	// 826AD594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD5A8 size=112
    let mut pc: u32 = 0x826AD5A8;
    'dispatch: loop {
        match pc {
            0x826AD5A8 => {
    //   block [0x826AD5A8..0x826AD618)
	// 826AD5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD5B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD5BC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD5C4: 390B0098  addi r8, r11, 0x98
	ctx.r[8].s64 = ctx.r[11].s64 + 152;
	// 826AD5C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AD5CC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826AD5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD5E0: 386AB498  addi r3, r10, -0x4b68
	ctx.r[3].s64 = ctx.r[10].s64 + -19304;
	// 826AD5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD604: 4BDB981D  bl 0x82466e20
	ctx.lr = 0x826AD608;
	sub_82466E20(ctx, base);
	// 826AD608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD618 size=108
    let mut pc: u32 = 0x826AD618;
    'dispatch: loop {
        match pc {
            0x826AD618 => {
    //   block [0x826AD618..0x826AD684)
	// 826AD618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD624: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD62C: 38EB00E0  addi r7, r11, 0xe0
	ctx.r[7].s64 = ctx.r[11].s64 + 224;
	// 826AD630: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AD634: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 826AD638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD648: 386AB4C8  addi r3, r10, -0x4b38
	ctx.r[3].s64 = ctx.r[10].s64 + -19256;
	// 826AD64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD670: 4BDB97B1  bl 0x82466e20
	ctx.lr = 0x826AD674;
	sub_82466E20(ctx, base);
	// 826AD674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD688 size=108
    let mut pc: u32 = 0x826AD688;
    'dispatch: loop {
        match pc {
            0x826AD688 => {
    //   block [0x826AD688..0x826AD6F4)
	// 826AD688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD694: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD69C: 38EB0110  addi r7, r11, 0x110
	ctx.r[7].s64 = ctx.r[11].s64 + 272;
	// 826AD6A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD6A4: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 826AD6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD6B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD6B8: 386AB4F8  addi r3, r10, -0x4b08
	ctx.r[3].s64 = ctx.r[10].s64 + -19208;
	// 826AD6BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD6E0: 4BDB9741  bl 0x82466e20
	ctx.lr = 0x826AD6E4;
	sub_82466E20(ctx, base);
	// 826AD6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD6F8 size=112
    let mut pc: u32 = 0x826AD6F8;
    'dispatch: loop {
        match pc {
            0x826AD6F8 => {
    //   block [0x826AD6F8..0x826AD768)
	// 826AD6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD708: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD70C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD714: 390B0128  addi r8, r11, 0x128
	ctx.r[8].s64 = ctx.r[11].s64 + 296;
	// 826AD718: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AD71C: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 826AD720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD730: 386AB528  addi r3, r10, -0x4ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -19160;
	// 826AD734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD754: 4BDB96CD  bl 0x82466e20
	ctx.lr = 0x826AD758;
	sub_82466E20(ctx, base);
	// 826AD758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD768 size=112
    let mut pc: u32 = 0x826AD768;
    'dispatch: loop {
        match pc {
            0x826AD768 => {
    //   block [0x826AD768..0x826AD7D8)
	// 826AD768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD778: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD77C: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD780: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD784: 390B0158  addi r8, r11, 0x158
	ctx.r[8].s64 = ctx.r[11].s64 + 344;
	// 826AD788: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AD78C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 826AD790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD7A0: 386AB558  addi r3, r10, -0x4aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -19112;
	// 826AD7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD7C4: 4BDB965D  bl 0x82466e20
	ctx.lr = 0x826AD7C8;
	sub_82466E20(ctx, base);
	// 826AD7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD7D8 size=112
    let mut pc: u32 = 0x826AD7D8;
    'dispatch: loop {
        match pc {
            0x826AD7D8 => {
    //   block [0x826AD7D8..0x826AD848)
	// 826AD7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD7E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD7E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD7EC: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD7F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD7F4: 390B01A0  addi r8, r11, 0x1a0
	ctx.r[8].s64 = ctx.r[11].s64 + 416;
	// 826AD7F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD7FC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826AD800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD810: 386AB588  addi r3, r10, -0x4a78
	ctx.r[3].s64 = ctx.r[10].s64 + -19064;
	// 826AD814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD834: 4BDB95ED  bl 0x82466e20
	ctx.lr = 0x826AD838;
	sub_82466E20(ctx, base);
	// 826AD838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD848 size=112
    let mut pc: u32 = 0x826AD848;
    'dispatch: loop {
        match pc {
            0x826AD848 => {
    //   block [0x826AD848..0x826AD8B8)
	// 826AD848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD85C: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD864: 390B0200  addi r8, r11, 0x200
	ctx.r[8].s64 = ctx.r[11].s64 + 512;
	// 826AD868: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD86C: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 826AD870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD880: 386AB5B8  addi r3, r10, -0x4a48
	ctx.r[3].s64 = ctx.r[10].s64 + -19016;
	// 826AD884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD8A4: 4BDB957D  bl 0x82466e20
	ctx.lr = 0x826AD8A8;
	sub_82466E20(ctx, base);
	// 826AD8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD8B8 size=112
    let mut pc: u32 = 0x826AD8B8;
    'dispatch: loop {
        match pc {
            0x826AD8B8 => {
    //   block [0x826AD8B8..0x826AD928)
	// 826AD8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD8C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD8C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD8CC: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD8D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD8D4: 390B0260  addi r8, r11, 0x260
	ctx.r[8].s64 = ctx.r[11].s64 + 608;
	// 826AD8D8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AD8DC: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826AD8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD8F0: 386AB5E8  addi r3, r10, -0x4a18
	ctx.r[3].s64 = ctx.r[10].s64 + -18968;
	// 826AD8F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD914: 4BDB950D  bl 0x82466e20
	ctx.lr = 0x826AD918;
	sub_82466E20(ctx, base);
	// 826AD918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD928 size=112
    let mut pc: u32 = 0x826AD928;
    'dispatch: loop {
        match pc {
            0x826AD928 => {
    //   block [0x826AD928..0x826AD998)
	// 826AD928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD93C: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD944: 390B0320  addi r8, r11, 0x320
	ctx.r[8].s64 = ctx.r[11].s64 + 800;
	// 826AD948: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD94C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 826AD950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD960: 386AB618  addi r3, r10, -0x49e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18920;
	// 826AD964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD984: 4BDB949D  bl 0x82466e20
	ctx.lr = 0x826AD988;
	sub_82466E20(ctx, base);
	// 826AD988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD998 size=112
    let mut pc: u32 = 0x826AD998;
    'dispatch: loop {
        match pc {
            0x826AD998 => {
    //   block [0x826AD998..0x826ADA08)
	// 826AD998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD9A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD9A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD9AC: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD9B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD9B4: 390B0380  addi r8, r11, 0x380
	ctx.r[8].s64 = ctx.r[11].s64 + 896;
	// 826AD9B8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AD9BC: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826AD9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD9C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD9C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD9D0: 386AB648  addi r3, r10, -0x49b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18872;
	// 826AD9D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD9F4: 4BDB942D  bl 0x82466e20
	ctx.lr = 0x826AD9F8;
	sub_82466E20(ctx, base);
	// 826AD9F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADA08 size=112
    let mut pc: u32 = 0x826ADA08;
    'dispatch: loop {
        match pc {
            0x826ADA08 => {
    //   block [0x826ADA08..0x826ADA78)
	// 826ADA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADA14: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ADA18: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826ADA1C: 38EA0440  addi r7, r10, 0x440
	ctx.r[7].s64 = ctx.r[10].s64 + 1088;
	// 826ADA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADA24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ADA28: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 826ADA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADA30: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADA34: 396BDA80  addi r11, r11, -0x2580
	ctx.r[11].s64 = ctx.r[11].s64 + -9600;
	// 826ADA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADA3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADA40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADA44: 386AB678  addi r3, r10, -0x4988
	ctx.r[3].s64 = ctx.r[10].s64 + -18824;
	// 826ADA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADA4C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826ADA50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADA54: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826ADA58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADA5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADA60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADA64: 4BDB93BD  bl 0x82466e20
	ctx.lr = 0x826ADA68;
	sub_82466E20(ctx, base);
	// 826ADA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADA78 size=112
    let mut pc: u32 = 0x826ADA78;
    'dispatch: loop {
        match pc {
            0x826ADA78 => {
    //   block [0x826ADA78..0x826ADAE8)
	// 826ADA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADA84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADA88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADA8C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADA90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADA94: 390B0608  addi r8, r11, 0x608
	ctx.r[8].s64 = ctx.r[11].s64 + 1544;
	// 826ADA98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADA9C: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 826ADAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADAA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADAB0: 386AB6A8  addi r3, r10, -0x4958
	ctx.r[3].s64 = ctx.r[10].s64 + -18776;
	// 826ADAB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADAC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADAD4: 4BDB934D  bl 0x82466e20
	ctx.lr = 0x826ADAD8;
	sub_82466E20(ctx, base);
	// 826ADAD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADAE8 size=108
    let mut pc: u32 = 0x826ADAE8;
    'dispatch: loop {
        match pc {
            0x826ADAE8 => {
    //   block [0x826ADAE8..0x826ADB54)
	// 826ADAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADAF4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADAF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADAFC: 38EB0620  addi r7, r11, 0x620
	ctx.r[7].s64 = ctx.r[11].s64 + 1568;
	// 826ADB00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADB04: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826ADB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADB18: 386AB6D8  addi r3, r10, -0x4928
	ctx.r[3].s64 = ctx.r[10].s64 + -18728;
	// 826ADB1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADB40: 4BDB92E1  bl 0x82466e20
	ctx.lr = 0x826ADB44;
	sub_82466E20(ctx, base);
	// 826ADB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADB58 size=112
    let mut pc: u32 = 0x826ADB58;
    'dispatch: loop {
        match pc {
            0x826ADB58 => {
    //   block [0x826ADB58..0x826ADBC8)
	// 826ADB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADB64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADB6C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADB70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADB74: 390B0650  addi r8, r11, 0x650
	ctx.r[8].s64 = ctx.r[11].s64 + 1616;
	// 826ADB78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADB7C: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826ADB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADB84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADB90: 386AB708  addi r3, r10, -0x48f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18680;
	// 826ADB94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADB98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADBA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADBA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADBA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADBB4: 4BDB926D  bl 0x82466e20
	ctx.lr = 0x826ADBB8;
	sub_82466E20(ctx, base);
	// 826ADBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADBC8 size=108
    let mut pc: u32 = 0x826ADBC8;
    'dispatch: loop {
        match pc {
            0x826ADBC8 => {
    //   block [0x826ADBC8..0x826ADC34)
	// 826ADBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADBD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADBDC: 38EB0668  addi r7, r11, 0x668
	ctx.r[7].s64 = ctx.r[11].s64 + 1640;
	// 826ADBE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADBE4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826ADBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADBEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADBF8: 386AB738  addi r3, r10, -0x48c8
	ctx.r[3].s64 = ctx.r[10].s64 + -18632;
	// 826ADBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADC20: 4BDB9201  bl 0x82466e20
	ctx.lr = 0x826ADC24;
	sub_82466E20(ctx, base);
	// 826ADC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADC38 size=108
    let mut pc: u32 = 0x826ADC38;
    'dispatch: loop {
        match pc {
            0x826ADC38 => {
    //   block [0x826ADC38..0x826ADCA4)
	// 826ADC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADC44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADC48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADC4C: 38EB0698  addi r7, r11, 0x698
	ctx.r[7].s64 = ctx.r[11].s64 + 1688;
	// 826ADC50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ADC54: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826ADC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADC5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADC60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADC68: 386AB768  addi r3, r10, -0x4898
	ctx.r[3].s64 = ctx.r[10].s64 + -18584;
	// 826ADC6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADC8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADC90: 4BDB9191  bl 0x82466e20
	ctx.lr = 0x826ADC94;
	sub_82466E20(ctx, base);
	// 826ADC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADCA8 size=112
    let mut pc: u32 = 0x826ADCA8;
    'dispatch: loop {
        match pc {
            0x826ADCA8 => {
    //   block [0x826ADCA8..0x826ADD18)
	// 826ADCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADCB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADCB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADCBC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ADCC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADCC4: 390B06E0  addi r8, r11, 0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1760;
	// 826ADCC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ADCCC: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826ADCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADCD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADCD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADCDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADCE0: 386AB798  addi r3, r10, -0x4868
	ctx.r[3].s64 = ctx.r[10].s64 + -18536;
	// 826ADCE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADD04: 4BDB911D  bl 0x82466e20
	ctx.lr = 0x826ADD08;
	sub_82466E20(ctx, base);
	// 826ADD08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADD18 size=112
    let mut pc: u32 = 0x826ADD18;
    'dispatch: loop {
        match pc {
            0x826ADD18 => {
    //   block [0x826ADD18..0x826ADD88)
	// 826ADD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD28: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADD2C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADD30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADD34: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 826ADD38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADD3C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826ADD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADD44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADD4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADD50: 386AB7C8  addi r3, r10, -0x4838
	ctx.r[3].s64 = ctx.r[10].s64 + -18488;
	// 826ADD54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADD64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADD74: 4BDB90AD  bl 0x82466e20
	ctx.lr = 0x826ADD78;
	sub_82466E20(ctx, base);
	// 826ADD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADD88 size=112
    let mut pc: u32 = 0x826ADD88;
    'dispatch: loop {
        match pc {
            0x826ADD88 => {
    //   block [0x826ADD88..0x826ADDF8)
	// 826ADD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADD94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADD9C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADDA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADDA4: 390B0740  addi r8, r11, 0x740
	ctx.r[8].s64 = ctx.r[11].s64 + 1856;
	// 826ADDA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ADDAC: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826ADDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADDB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADDC0: 386AB7F8  addi r3, r10, -0x4808
	ctx.r[3].s64 = ctx.r[10].s64 + -18440;
	// 826ADDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADDE4: 4BDB903D  bl 0x82466e20
	ctx.lr = 0x826ADDE8;
	sub_82466E20(ctx, base);
	// 826ADDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADDF8 size=112
    let mut pc: u32 = 0x826ADDF8;
    'dispatch: loop {
        match pc {
            0x826ADDF8 => {
    //   block [0x826ADDF8..0x826ADE68)
	// 826ADDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADE04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADE0C: 38AAB2E8  addi r5, r10, -0x4d18
	ctx.r[5].s64 = ctx.r[10].s64 + -19736;
	// 826ADE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADE14: 390B0770  addi r8, r11, 0x770
	ctx.r[8].s64 = ctx.r[11].s64 + 1904;
	// 826ADE18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADE1C: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826ADE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADE24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADE30: 386AB828  addi r3, r10, -0x47d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18392;
	// 826ADE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADE54: 4BDB8FCD  bl 0x82466e20
	ctx.lr = 0x826ADE58;
	sub_82466E20(ctx, base);
	// 826ADE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADE68 size=108
    let mut pc: u32 = 0x826ADE68;
    'dispatch: loop {
        match pc {
            0x826ADE68 => {
    //   block [0x826ADE68..0x826ADED4)
	// 826ADE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADE74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADE78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADE7C: 38EB0788  addi r7, r11, 0x788
	ctx.r[7].s64 = ctx.r[11].s64 + 1928;
	// 826ADE80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADE84: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826ADE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADE8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADE98: 386AB858  addi r3, r10, -0x47a8
	ctx.r[3].s64 = ctx.r[10].s64 + -18344;
	// 826ADE9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADEBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADEC0: 4BDB8F61  bl 0x82466e20
	ctx.lr = 0x826ADEC4;
	sub_82466E20(ctx, base);
	// 826ADEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADED8 size=112
    let mut pc: u32 = 0x826ADED8;
    'dispatch: loop {
        match pc {
            0x826ADED8 => {
    //   block [0x826ADED8..0x826ADF48)
	// 826ADED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADEE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADEE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADEEC: 38AAB858  addi r5, r10, -0x47a8
	ctx.r[5].s64 = ctx.r[10].s64 + -18344;
	// 826ADEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADEF4: 390B07B8  addi r8, r11, 0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1976;
	// 826ADEF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ADEFC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826ADF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADF04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADF10: 386AB888  addi r3, r10, -0x4778
	ctx.r[3].s64 = ctx.r[10].s64 + -18296;
	// 826ADF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADF34: 4BDB8EED  bl 0x82466e20
	ctx.lr = 0x826ADF38;
	sub_82466E20(ctx, base);
	// 826ADF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ADF48 size=24
    let mut pc: u32 = 0x826ADF48;
    'dispatch: loop {
        match pc {
            0x826ADF48 => {
    //   block [0x826ADF48..0x826ADF60)
	// 826ADF48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADF4C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ADF50: 394A4A08  addi r10, r10, 0x4a08
	ctx.r[10].s64 = ctx.r[10].s64 + 18952;
	// 826ADF54: 816B07E8  lwz r11, 0x7e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2024 as u32) ) } as u64;
	// 826ADF58: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826ADF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADF60 size=116
    let mut pc: u32 = 0x826ADF60;
    'dispatch: loop {
        match pc {
            0x826ADF60 => {
    //   block [0x826ADF60..0x826ADFD4)
	// 826ADF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADF6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADF70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ADF74: 390B4A08  addi r8, r11, 0x4a08
	ctx.r[8].s64 = ctx.r[11].s64 + 18952;
	// 826ADF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADF7C: 392ADB20  addi r9, r10, -0x24e0
	ctx.r[9].s64 = ctx.r[10].s64 + -9440;
	// 826ADF80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADF84: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826ADF88: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ADF8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADF94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADFA4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826ADFA8: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826ADFAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ADFB0: 386BB8B8  addi r3, r11, -0x4748
	ctx.r[3].s64 = ctx.r[11].s64 + -18248;
	// 826ADFB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ADFB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADFC0: 4BDB8E61  bl 0x82466e20
	ctx.lr = 0x826ADFC4;
	sub_82466E20(ctx, base);
	// 826ADFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADFD8 size=108
    let mut pc: u32 = 0x826ADFD8;
    'dispatch: loop {
        match pc {
            0x826ADFD8 => {
    //   block [0x826ADFD8..0x826AE044)
	// 826ADFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADFE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADFE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADFEC: 38EB07F0  addi r7, r11, 0x7f0
	ctx.r[7].s64 = ctx.r[11].s64 + 2032;
	// 826ADFF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ADFF4: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826ADFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADFFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE008: 386AB8E8  addi r3, r10, -0x4718
	ctx.r[3].s64 = ctx.r[10].s64 + -18200;
	// 826AE00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE030: 4BDB8DF1  bl 0x82466e20
	ctx.lr = 0x826AE034;
	sub_82466E20(ctx, base);
	// 826AE034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE048 size=108
    let mut pc: u32 = 0x826AE048;
    'dispatch: loop {
        match pc {
            0x826AE048 => {
    //   block [0x826AE048..0x826AE0B4)
	// 826AE048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE054: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE058: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE05C: 38EB0838  addi r7, r11, 0x838
	ctx.r[7].s64 = ctx.r[11].s64 + 2104;
	// 826AE060: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE064: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826AE068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE078: 386AB918  addi r3, r10, -0x46e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18152;
	// 826AE07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE0A0: 4BDB8D81  bl 0x82466e20
	ctx.lr = 0x826AE0A4;
	sub_82466E20(ctx, base);
	// 826AE0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE0B8 size=108
    let mut pc: u32 = 0x826AE0B8;
    'dispatch: loop {
        match pc {
            0x826AE0B8 => {
    //   block [0x826AE0B8..0x826AE124)
	// 826AE0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE0C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE0C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE0CC: 38EB0868  addi r7, r11, 0x868
	ctx.r[7].s64 = ctx.r[11].s64 + 2152;
	// 826AE0D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE0D4: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826AE0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE0E8: 386AB948  addi r3, r10, -0x46b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18104;
	// 826AE0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE110: 4BDB8D11  bl 0x82466e20
	ctx.lr = 0x826AE114;
	sub_82466E20(ctx, base);
	// 826AE114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE128 size=112
    let mut pc: u32 = 0x826AE128;
    'dispatch: loop {
        match pc {
            0x826AE128 => {
    //   block [0x826AE128..0x826AE198)
	// 826AE128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE138: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE13C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE140: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE144: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 826AE148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AE14C: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826AE150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE160: 386AB978  addi r3, r10, -0x4688
	ctx.r[3].s64 = ctx.r[10].s64 + -18056;
	// 826AE164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE184: 4BDB8C9D  bl 0x82466e20
	ctx.lr = 0x826AE188;
	sub_82466E20(ctx, base);
	// 826AE188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE198 size=112
    let mut pc: u32 = 0x826AE198;
    'dispatch: loop {
        match pc {
            0x826AE198 => {
    //   block [0x826AE198..0x826AE208)
	// 826AE198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE1A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE1AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE1B4: 390B08C8  addi r8, r11, 0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + 2248;
	// 826AE1B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE1BC: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826AE1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE1D0: 386AB9A8  addi r3, r10, -0x4658
	ctx.r[3].s64 = ctx.r[10].s64 + -18008;
	// 826AE1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE1F4: 4BDB8C2D  bl 0x82466e20
	ctx.lr = 0x826AE1F8;
	sub_82466E20(ctx, base);
	// 826AE1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE208 size=112
    let mut pc: u32 = 0x826AE208;
    'dispatch: loop {
        match pc {
            0x826AE208 => {
    //   block [0x826AE208..0x826AE278)
	// 826AE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE218: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE21C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE224: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 826AE228: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE22C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826AE230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE240: 386AB9D8  addi r3, r10, -0x4628
	ctx.r[3].s64 = ctx.r[10].s64 + -17960;
	// 826AE244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE264: 4BDB8BBD  bl 0x82466e20
	ctx.lr = 0x826AE268;
	sub_82466E20(ctx, base);
	// 826AE268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE278 size=108
    let mut pc: u32 = 0x826AE278;
    'dispatch: loop {
        match pc {
            0x826AE278 => {
    //   block [0x826AE278..0x826AE2E4)
	// 826AE278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE284: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE28C: 38EB08F8  addi r7, r11, 0x8f8
	ctx.r[7].s64 = ctx.r[11].s64 + 2296;
	// 826AE290: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE294: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826AE298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE29C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE2A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE2A8: 386ABA08  addi r3, r10, -0x45f8
	ctx.r[3].s64 = ctx.r[10].s64 + -17912;
	// 826AE2AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE2D0: 4BDB8B51  bl 0x82466e20
	ctx.lr = 0x826AE2D4;
	sub_82466E20(ctx, base);
	// 826AE2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE2E8 size=112
    let mut pc: u32 = 0x826AE2E8;
    'dispatch: loop {
        match pc {
            0x826AE2E8 => {
    //   block [0x826AE2E8..0x826AE358)
	// 826AE2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE2FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE304: 390B0928  addi r8, r11, 0x928
	ctx.r[8].s64 = ctx.r[11].s64 + 2344;
	// 826AE308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE30C: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826AE310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE320: 386ABA38  addi r3, r10, -0x45c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17864;
	// 826AE324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE344: 4BDB8ADD  bl 0x82466e20
	ctx.lr = 0x826AE348;
	sub_82466E20(ctx, base);
	// 826AE348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE358 size=108
    let mut pc: u32 = 0x826AE358;
    'dispatch: loop {
        match pc {
            0x826AE358 => {
    //   block [0x826AE358..0x826AE3C4)
	// 826AE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE364: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE36C: 38EB0940  addi r7, r11, 0x940
	ctx.r[7].s64 = ctx.r[11].s64 + 2368;
	// 826AE370: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826AE374: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826AE378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE37C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE388: 386ABA68  addi r3, r10, -0x4598
	ctx.r[3].s64 = ctx.r[10].s64 + -17816;
	// 826AE38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE3B0: 4BDB8A71  bl 0x82466e20
	ctx.lr = 0x826AE3B4;
	sub_82466E20(ctx, base);
	// 826AE3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE3C8 size=116
    let mut pc: u32 = 0x826AE3C8;
    'dispatch: loop {
        match pc {
            0x826AE3C8 => {
    //   block [0x826AE3C8..0x826AE43C)
	// 826AE3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE3D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AE3D8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826AE3DC: 390A0A30  addi r8, r10, 0xa30
	ctx.r[8].s64 = ctx.r[10].s64 + 2608;
	// 826AE3E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE3E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AE3E8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE3EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE3F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AE3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE3FC: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826AE400: 396BDB38  addi r11, r11, -0x24c8
	ctx.r[11].s64 = ctx.r[11].s64 + -9416;
	// 826AE404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE40C: 386ABA98  addi r3, r10, -0x4568
	ctx.r[3].s64 = ctx.r[10].s64 + -17768;
	// 826AE410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AE414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AE41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE428: 4BDB89F9  bl 0x82466e20
	ctx.lr = 0x826AE42C;
	sub_82466E20(ctx, base);
	// 826AE42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE440 size=108
    let mut pc: u32 = 0x826AE440;
    'dispatch: loop {
        match pc {
            0x826AE440 => {
    //   block [0x826AE440..0x826AE4AC)
	// 826AE440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE44C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE454: 38EB0BF8  addi r7, r11, 0xbf8
	ctx.r[7].s64 = ctx.r[11].s64 + 3064;
	// 826AE458: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826AE45C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826AE460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE470: 386ABAC8  addi r3, r10, -0x4538
	ctx.r[3].s64 = ctx.r[10].s64 + -17720;
	// 826AE474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE498: 4BDB8989  bl 0x82466e20
	ctx.lr = 0x826AE49C;
	sub_82466E20(ctx, base);
	// 826AE49C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE4A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE4A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE4B0 size=112
    let mut pc: u32 = 0x826AE4B0;
    'dispatch: loop {
        match pc {
            0x826AE4B0 => {
    //   block [0x826AE4B0..0x826AE520)
	// 826AE4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE4BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE4C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE4C4: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AE4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE4CC: 390B0D90  addi r8, r11, 0xd90
	ctx.r[8].s64 = ctx.r[11].s64 + 3472;
	// 826AE4D0: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826AE4D4: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826AE4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE4DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE4E8: 386ABAF8  addi r3, r10, -0x4508
	ctx.r[3].s64 = ctx.r[10].s64 + -17672;
	// 826AE4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE50C: 4BDB8915  bl 0x82466e20
	ctx.lr = 0x826AE510;
	sub_82466E20(ctx, base);
	// 826AE510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE520 size=100
    let mut pc: u32 = 0x826AE520;
    'dispatch: loop {
        match pc {
            0x826AE520 => {
    //   block [0x826AE520..0x826AE584)
	// 826AE520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE52C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE534: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE540: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826AE544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE554: 386ABB28  addi r3, r10, -0x44d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17624;
	// 826AE558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE55C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE570: 4BDB88B1  bl 0x82466e20
	ctx.lr = 0x826AE574;
	sub_82466E20(ctx, base);
	// 826AE574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE588 size=112
    let mut pc: u32 = 0x826AE588;
    'dispatch: loop {
        match pc {
            0x826AE588 => {
    //   block [0x826AE588..0x826AE5F8)
	// 826AE588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE598: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE59C: 38AABB28  addi r5, r10, -0x44d8
	ctx.r[5].s64 = ctx.r[10].s64 + -17624;
	// 826AE5A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE5A4: 390B0FE8  addi r8, r11, 0xfe8
	ctx.r[8].s64 = ctx.r[11].s64 + 4072;
	// 826AE5A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AE5AC: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826AE5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE5C0: 386ABB58  addi r3, r10, -0x44a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17576;
	// 826AE5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE5E4: 4BDB883D  bl 0x82466e20
	ctx.lr = 0x826AE5E8;
	sub_82466E20(ctx, base);
	// 826AE5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE5F8 size=100
    let mut pc: u32 = 0x826AE5F8;
    'dispatch: loop {
        match pc {
            0x826AE5F8 => {
    //   block [0x826AE5F8..0x826AE65C)
	// 826AE5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE60C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE610: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE618: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826AE61C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE62C: 386ABB88  addi r3, r10, -0x4478
	ctx.r[3].s64 = ctx.r[10].s64 + -17528;
	// 826AE630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE634: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE638: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE640: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE648: 4BDB87D9  bl 0x82466e20
	ctx.lr = 0x826AE64C;
	sub_82466E20(ctx, base);
	// 826AE64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE660 size=108
    let mut pc: u32 = 0x826AE660;
    'dispatch: loop {
        match pc {
            0x826AE660 => {
    //   block [0x826AE660..0x826AE6CC)
	// 826AE660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE66C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE674: 38EB1060  addi r7, r11, 0x1060
	ctx.r[7].s64 = ctx.r[11].s64 + 4192;
	// 826AE678: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AE67C: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826AE680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE690: 386ABBB8  addi r3, r10, -0x4448
	ctx.r[3].s64 = ctx.r[10].s64 + -17480;
	// 826AE694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE6B8: 4BDB8769  bl 0x82466e20
	ctx.lr = 0x826AE6BC;
	sub_82466E20(ctx, base);
	// 826AE6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE6D0 size=112
    let mut pc: u32 = 0x826AE6D0;
    'dispatch: loop {
        match pc {
            0x826AE6D0 => {
    //   block [0x826AE6D0..0x826AE740)
	// 826AE6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE6DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE6E0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE6E4: 38AABB88  addi r5, r10, -0x4478
	ctx.r[5].s64 = ctx.r[10].s64 + -17528;
	// 826AE6E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE6EC: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 826AE6F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AE6F4: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826AE6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE6FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE708: 386ABBE8  addi r3, r10, -0x4418
	ctx.r[3].s64 = ctx.r[10].s64 + -17432;
	// 826AE70C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE72C: 4BDB86F5  bl 0x82466e20
	ctx.lr = 0x826AE730;
	sub_82466E20(ctx, base);
	// 826AE730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE740 size=100
    let mut pc: u32 = 0x826AE740;
    'dispatch: loop {
        match pc {
            0x826AE740 => {
    //   block [0x826AE740..0x826AE7A4)
	// 826AE740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE74C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE754: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE758: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE760: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826AE764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE774: 386ABC18  addi r3, r10, -0x43e8
	ctx.r[3].s64 = ctx.r[10].s64 + -17384;
	// 826AE778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE780: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE788: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE790: 4BDB8691  bl 0x82466e20
	ctx.lr = 0x826AE794;
	sub_82466E20(ctx, base);
	// 826AE794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE7A8 size=100
    let mut pc: u32 = 0x826AE7A8;
    'dispatch: loop {
        match pc {
            0x826AE7A8 => {
    //   block [0x826AE7A8..0x826AE80C)
	// 826AE7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE7B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE7BC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE7C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE7C8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826AE7CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE7DC: 386ABC48  addi r3, r10, -0x43b8
	ctx.r[3].s64 = ctx.r[10].s64 + -17336;
	// 826AE7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE7E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE7F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE7F8: 4BDB8629  bl 0x82466e20
	ctx.lr = 0x826AE7FC;
	sub_82466E20(ctx, base);
	// 826AE7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE810 size=112
    let mut pc: u32 = 0x826AE810;
    'dispatch: loop {
        match pc {
            0x826AE810 => {
    //   block [0x826AE810..0x826AE880)
	// 826AE810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE81C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE820: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE824: 38AABC18  addi r5, r10, -0x43e8
	ctx.r[5].s64 = ctx.r[10].s64 + -17384;
	// 826AE828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE82C: 390B10D8  addi r8, r11, 0x10d8
	ctx.r[8].s64 = ctx.r[11].s64 + 4312;
	// 826AE830: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AE834: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826AE838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE83C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE848: 386ABC78  addi r3, r10, -0x4388
	ctx.r[3].s64 = ctx.r[10].s64 + -17288;
	// 826AE84C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE86C: 4BDB85B5  bl 0x82466e20
	ctx.lr = 0x826AE870;
	sub_82466E20(ctx, base);
	// 826AE870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE880 size=112
    let mut pc: u32 = 0x826AE880;
    'dispatch: loop {
        match pc {
            0x826AE880 => {
    //   block [0x826AE880..0x826AE8F0)
	// 826AE880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE88C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE890: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE894: 38AABC48  addi r5, r10, -0x43b8
	ctx.r[5].s64 = ctx.r[10].s64 + -17336;
	// 826AE898: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE89C: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 826AE8A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AE8A4: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826AE8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE8AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE8B8: 386ABCA8  addi r3, r10, -0x4358
	ctx.r[3].s64 = ctx.r[10].s64 + -17240;
	// 826AE8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE8DC: 4BDB8545  bl 0x82466e20
	ctx.lr = 0x826AE8E0;
	sub_82466E20(ctx, base);
	// 826AE8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE8F0 size=100
    let mut pc: u32 = 0x826AE8F0;
    'dispatch: loop {
        match pc {
            0x826AE8F0 => {
    //   block [0x826AE8F0..0x826AE954)
	// 826AE8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE8FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE904: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE910: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826AE914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE924: 386ABCD8  addi r3, r10, -0x4328
	ctx.r[3].s64 = ctx.r[10].s64 + -17192;
	// 826AE928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE940: 4BDB84E1  bl 0x82466e20
	ctx.lr = 0x826AE944;
	sub_82466E20(ctx, base);
	// 826AE944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE958 size=112
    let mut pc: u32 = 0x826AE958;
    'dispatch: loop {
        match pc {
            0x826AE958 => {
    //   block [0x826AE958..0x826AE9C8)
	// 826AE958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE968: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE96C: 38AABCD8  addi r5, r10, -0x4328
	ctx.r[5].s64 = ctx.r[10].s64 + -17192;
	// 826AE970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE974: 390B1198  addi r8, r11, 0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + 4504;
	// 826AE978: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826AE97C: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826AE980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE990: 386ABD08  addi r3, r10, -0x42f8
	ctx.r[3].s64 = ctx.r[10].s64 + -17144;
	// 826AE994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE9B4: 4BDB846D  bl 0x82466e20
	ctx.lr = 0x826AE9B8;
	sub_82466E20(ctx, base);
	// 826AE9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE9C8 size=108
    let mut pc: u32 = 0x826AE9C8;
    'dispatch: loop {
        match pc {
            0x826AE9C8 => {
    //   block [0x826AE9C8..0x826AEA34)
	// 826AE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE9D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE9D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE9DC: 38EB1288  addi r7, r11, 0x1288
	ctx.r[7].s64 = ctx.r[11].s64 + 4744;
	// 826AE9E0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826AE9E4: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826AE9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE9F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE9F8: 386ABD38  addi r3, r10, -0x42c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17096;
	// 826AE9FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEA20: 4BDB8401  bl 0x82466e20
	ctx.lr = 0x826AEA24;
	sub_82466E20(ctx, base);
	// 826AEA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEA38 size=108
    let mut pc: u32 = 0x826AEA38;
    'dispatch: loop {
        match pc {
            0x826AEA38 => {
    //   block [0x826AEA38..0x826AEAA4)
	// 826AEA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEA44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEA48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEA4C: 38EB1378  addi r7, r11, 0x1378
	ctx.r[7].s64 = ctx.r[11].s64 + 4984;
	// 826AEA50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AEA54: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826AEA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEA5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEA68: 386ABD68  addi r3, r10, -0x4298
	ctx.r[3].s64 = ctx.r[10].s64 + -17048;
	// 826AEA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEA90: 4BDB8391  bl 0x82466e20
	ctx.lr = 0x826AEA94;
	sub_82466E20(ctx, base);
	// 826AEA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEAA8 size=108
    let mut pc: u32 = 0x826AEAA8;
    'dispatch: loop {
        match pc {
            0x826AEAA8 => {
    //   block [0x826AEAA8..0x826AEB14)
	// 826AEAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEAB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEAB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEABC: 38EB13C0  addi r7, r11, 0x13c0
	ctx.r[7].s64 = ctx.r[11].s64 + 5056;
	// 826AEAC0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826AEAC4: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826AEAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEACC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEAD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEAD8: 386ABD98  addi r3, r10, -0x4268
	ctx.r[3].s64 = ctx.r[10].s64 + -17000;
	// 826AEADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEAFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEB00: 4BDB8321  bl 0x82466e20
	ctx.lr = 0x826AEB04;
	sub_82466E20(ctx, base);
	// 826AEB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEB18 size=108
    let mut pc: u32 = 0x826AEB18;
    'dispatch: loop {
        match pc {
            0x826AEB18 => {
    //   block [0x826AEB18..0x826AEB84)
	// 826AEB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEB24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEB2C: 38EB1498  addi r7, r11, 0x1498
	ctx.r[7].s64 = ctx.r[11].s64 + 5272;
	// 826AEB30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AEB34: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826AEB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEB3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEB40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEB48: 386ABDC8  addi r3, r10, -0x4238
	ctx.r[3].s64 = ctx.r[10].s64 + -16952;
	// 826AEB4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEB70: 4BDB82B1  bl 0x82466e20
	ctx.lr = 0x826AEB74;
	sub_82466E20(ctx, base);
	// 826AEB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEB88 size=100
    let mut pc: u32 = 0x826AEB88;
    'dispatch: loop {
        match pc {
            0x826AEB88 => {
    //   block [0x826AEB88..0x826AEBEC)
	// 826AEB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEB94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEB9C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEBA8: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826AEBAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEBBC: 386ABDF8  addi r3, r10, -0x4208
	ctx.r[3].s64 = ctx.r[10].s64 + -16904;
	// 826AEBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEBC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEBC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEBD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AEBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEBD8: 4BDB8249  bl 0x82466e20
	ctx.lr = 0x826AEBDC;
	sub_82466E20(ctx, base);
	// 826AEBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEBF0 size=112
    let mut pc: u32 = 0x826AEBF0;
    'dispatch: loop {
        match pc {
            0x826AEBF0 => {
    //   block [0x826AEBF0..0x826AEC60)
	// 826AEBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEBFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEC04: 38AABDF8  addi r5, r10, -0x4208
	ctx.r[5].s64 = ctx.r[10].s64 + -16904;
	// 826AEC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEC0C: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 826AEC10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEC14: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826AEC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEC1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEC28: 386ABE28  addi r3, r10, -0x41d8
	ctx.r[3].s64 = ctx.r[10].s64 + -16856;
	// 826AEC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEC4C: 4BDB81D5  bl 0x82466e20
	ctx.lr = 0x826AEC50;
	sub_82466E20(ctx, base);
	// 826AEC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEC60 size=108
    let mut pc: u32 = 0x826AEC60;
    'dispatch: loop {
        match pc {
            0x826AEC60 => {
    //   block [0x826AEC60..0x826AECCC)
	// 826AEC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEC6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEC70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEC74: 38EB14F8  addi r7, r11, 0x14f8
	ctx.r[7].s64 = ctx.r[11].s64 + 5368;
	// 826AEC78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AEC7C: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826AEC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEC84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEC90: 386ABE58  addi r3, r10, -0x41a8
	ctx.r[3].s64 = ctx.r[10].s64 + -16808;
	// 826AEC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AECA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AECA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AECA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AECAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AECB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AECB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AECB8: 4BDB8169  bl 0x82466e20
	ctx.lr = 0x826AECBC;
	sub_82466E20(ctx, base);
	// 826AECBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AECC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AECC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AECC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AECD0 size=112
    let mut pc: u32 = 0x826AECD0;
    'dispatch: loop {
        match pc {
            0x826AECD0 => {
    //   block [0x826AECD0..0x826AED40)
	// 826AECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AECD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AECDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AECE0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AECE4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AECE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AECEC: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 826AECF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AECF4: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826AECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AECFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AED00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AED04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AED08: 386ABE88  addi r3, r10, -0x4178
	ctx.r[3].s64 = ctx.r[10].s64 + -16760;
	// 826AED0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AED10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AED14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AED18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AED1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AED20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AED24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AED28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AED2C: 4BDB80F5  bl 0x82466e20
	ctx.lr = 0x826AED30;
	sub_82466E20(ctx, base);
	// 826AED30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AED40 size=108
    let mut pc: u32 = 0x826AED40;
    'dispatch: loop {
        match pc {
            0x826AED40 => {
    //   block [0x826AED40..0x826AEDAC)
	// 826AED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AED48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AED4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AED50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AED54: 38EB1558  addi r7, r11, 0x1558
	ctx.r[7].s64 = ctx.r[11].s64 + 5464;
	// 826AED58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AED5C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826AED60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AED64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AED68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AED70: 386ABEB8  addi r3, r10, -0x4148
	ctx.r[3].s64 = ctx.r[10].s64 + -16712;
	// 826AED74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AED78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AED7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AED80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AED84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AED88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AED8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AED90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AED94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AED98: 4BDB8089  bl 0x82466e20
	ctx.lr = 0x826AED9C;
	sub_82466E20(ctx, base);
	// 826AED9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEDA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEDA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEDB0 size=112
    let mut pc: u32 = 0x826AEDB0;
    'dispatch: loop {
        match pc {
            0x826AEDB0 => {
    //   block [0x826AEDB0..0x826AEE20)
	// 826AEDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEDBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEDC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEDC4: 38AABE88  addi r5, r10, -0x4178
	ctx.r[5].s64 = ctx.r[10].s64 + -16760;
	// 826AEDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEDCC: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826AEDD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AEDD4: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826AEDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEDDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEDE8: 386ABEE8  addi r3, r10, -0x4118
	ctx.r[3].s64 = ctx.r[10].s64 + -16664;
	// 826AEDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEE0C: 4BDB8015  bl 0x82466e20
	ctx.lr = 0x826AEE10;
	sub_82466E20(ctx, base);
	// 826AEE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEE20 size=100
    let mut pc: u32 = 0x826AEE20;
    'dispatch: loop {
        match pc {
            0x826AEE20 => {
    //   block [0x826AEE20..0x826AEE84)
	// 826AEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEE2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEE34: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEE40: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826AEE44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEE54: 386ABF18  addi r3, r10, -0x40e8
	ctx.r[3].s64 = ctx.r[10].s64 + -16616;
	// 826AEE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEE5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AEE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AEE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEE70: 4BDB7FB1  bl 0x82466e20
	ctx.lr = 0x826AEE74;
	sub_82466E20(ctx, base);
	// 826AEE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEE88 size=112
    let mut pc: u32 = 0x826AEE88;
    'dispatch: loop {
        match pc {
            0x826AEE88 => {
    //   block [0x826AEE88..0x826AEEF8)
	// 826AEE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEE94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEE9C: 38AABF18  addi r5, r10, -0x40e8
	ctx.r[5].s64 = ctx.r[10].s64 + -16616;
	// 826AEEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEEA4: 390B15B8  addi r8, r11, 0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5560;
	// 826AEEA8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AEEAC: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826AEEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEEB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEEC0: 386ABF48  addi r3, r10, -0x40b8
	ctx.r[3].s64 = ctx.r[10].s64 + -16568;
	// 826AEEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEEE4: 4BDB7F3D  bl 0x82466e20
	ctx.lr = 0x826AEEE8;
	sub_82466E20(ctx, base);
	// 826AEEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEEF8 size=108
    let mut pc: u32 = 0x826AEEF8;
    'dispatch: loop {
        match pc {
            0x826AEEF8 => {
    //   block [0x826AEEF8..0x826AEF64)
	// 826AEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEF04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEF08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEF0C: 38EB1660  addi r7, r11, 0x1660
	ctx.r[7].s64 = ctx.r[11].s64 + 5728;
	// 826AEF10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AEF14: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826AEF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEF1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEF28: 386ABF78  addi r3, r10, -0x4088
	ctx.r[3].s64 = ctx.r[10].s64 + -16520;
	// 826AEF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEF50: 4BDB7ED1  bl 0x82466e20
	ctx.lr = 0x826AEF54;
	sub_82466E20(ctx, base);
	// 826AEF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEF68 size=112
    let mut pc: u32 = 0x826AEF68;
    'dispatch: loop {
        match pc {
            0x826AEF68 => {
    //   block [0x826AEF68..0x826AEFD8)
	// 826AEF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEF74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEF7C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEF80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEF84: 390B1690  addi r8, r11, 0x1690
	ctx.r[8].s64 = ctx.r[11].s64 + 5776;
	// 826AEF88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEF8C: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826AEF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEF94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEFA0: 386ABFA8  addi r3, r10, -0x4058
	ctx.r[3].s64 = ctx.r[10].s64 + -16472;
	// 826AEFA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEFC4: 4BDB7E5D  bl 0x82466e20
	ctx.lr = 0x826AEFC8;
	sub_82466E20(ctx, base);
	// 826AEFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEFD8 size=112
    let mut pc: u32 = 0x826AEFD8;
    'dispatch: loop {
        match pc {
            0x826AEFD8 => {
    //   block [0x826AEFD8..0x826AF048)
	// 826AEFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEFE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEFE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEFEC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AEFF4: 390B16D8  addi r8, r11, 0x16d8
	ctx.r[8].s64 = ctx.r[11].s64 + 5848;
	// 826AEFF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEFFC: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826AF000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF010: 386ABFD8  addi r3, r10, -0x4028
	ctx.r[3].s64 = ctx.r[10].s64 + -16424;
	// 826AF014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF034: 4BDB7DED  bl 0x82466e20
	ctx.lr = 0x826AF038;
	sub_82466E20(ctx, base);
	// 826AF038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF048 size=100
    let mut pc: u32 = 0x826AF048;
    'dispatch: loop {
        match pc {
            0x826AF048 => {
    //   block [0x826AF048..0x826AF0AC)
	// 826AF048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF05C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF060: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF068: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826AF06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF07C: 386AC008  addi r3, r10, -0x3ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -16376;
	// 826AF080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AF08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AF094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF098: 4BDB7D89  bl 0x82466e20
	ctx.lr = 0x826AF09C;
	sub_82466E20(ctx, base);
	// 826AF09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF0B0 size=112
    let mut pc: u32 = 0x826AF0B0;
    'dispatch: loop {
        match pc {
            0x826AF0B0 => {
    //   block [0x826AF0B0..0x826AF120)
	// 826AF0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF0BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF0C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF0C4: 38AAC008  addi r5, r10, -0x3ff8
	ctx.r[5].s64 = ctx.r[10].s64 + -16376;
	// 826AF0C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF0CC: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 826AF0D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AF0D4: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826AF0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF0E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF0E8: 386AC038  addi r3, r10, -0x3fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -16328;
	// 826AF0EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF0F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF0FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF10C: 4BDB7D15  bl 0x82466e20
	ctx.lr = 0x826AF110;
	sub_82466E20(ctx, base);
	// 826AF110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF120 size=112
    let mut pc: u32 = 0x826AF120;
    'dispatch: loop {
        match pc {
            0x826AF120 => {
    //   block [0x826AF120..0x826AF190)
	// 826AF120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF130: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF134: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF13C: 390B1768  addi r8, r11, 0x1768
	ctx.r[8].s64 = ctx.r[11].s64 + 5992;
	// 826AF140: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF144: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826AF148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF158: 386AC068  addi r3, r10, -0x3f98
	ctx.r[3].s64 = ctx.r[10].s64 + -16280;
	// 826AF15C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF17C: 4BDB7CA5  bl 0x82466e20
	ctx.lr = 0x826AF180;
	sub_82466E20(ctx, base);
	// 826AF180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF190 size=112
    let mut pc: u32 = 0x826AF190;
    'dispatch: loop {
        match pc {
            0x826AF190 => {
    //   block [0x826AF190..0x826AF200)
	// 826AF190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF1A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF1A4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF1A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF1AC: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 826AF1B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF1B4: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826AF1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF1BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF1C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF1C8: 386AC098  addi r3, r10, -0x3f68
	ctx.r[3].s64 = ctx.r[10].s64 + -16232;
	// 826AF1CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF1DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AF1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF1EC: 4BDB7C35  bl 0x82466e20
	ctx.lr = 0x826AF1F0;
	sub_82466E20(ctx, base);
	// 826AF1F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF200 size=112
    let mut pc: u32 = 0x826AF200;
    'dispatch: loop {
        match pc {
            0x826AF200 => {
    //   block [0x826AF200..0x826AF270)
	// 826AF200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF210: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF214: 38AAC068  addi r5, r10, -0x3f98
	ctx.r[5].s64 = ctx.r[10].s64 + -16280;
	// 826AF218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF21C: 390B1798  addi r8, r11, 0x1798
	ctx.r[8].s64 = ctx.r[11].s64 + 6040;
	// 826AF220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AF224: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826AF228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF22C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF238: 386AC0C8  addi r3, r10, -0x3f38
	ctx.r[3].s64 = ctx.r[10].s64 + -16184;
	// 826AF23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF25C: 4BDB7BC5  bl 0x82466e20
	ctx.lr = 0x826AF260;
	sub_82466E20(ctx, base);
	// 826AF260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF270 size=72
    let mut pc: u32 = 0x826AF270;
    'dispatch: loop {
        match pc {
            0x826AF270 => {
    //   block [0x826AF270..0x826AF2B8)
	// 826AF270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF27C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF280: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826AF284: 38CBB410  addi r6, r11, -0x4bf0
	ctx.r[6].s64 = ctx.r[11].s64 + -19440;
	// 826AF288: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF28C: 388BDB88  addi r4, r11, -0x2478
	ctx.r[4].s64 = ctx.r[11].s64 + -9336;
	// 826AF290: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AF294: 386BC0F8  addi r3, r11, -0x3f08
	ctx.r[3].s64 = ctx.r[11].s64 + -16136;
	// 826AF298: 4BDCC7F1  bl 0x8247ba88
	ctx.lr = 0x826AF29C;
	sub_8247BA88(ctx, base);
	// 826AF29C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826AF2A0: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 826AF2A4: 4BE83895  bl 0x82532b38
	ctx.lr = 0x826AF2A8;
	sub_82532B38(ctx, base);
	// 826AF2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826AF2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF2B8 size=108
    let mut pc: u32 = 0x826AF2B8;
    'dispatch: loop {
        match pc {
            0x826AF2B8 => {
    //   block [0x826AF2B8..0x826AF324)
	// 826AF2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF2C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF2CC: 38EB5498  addi r7, r11, 0x5498
	ctx.r[7].s64 = ctx.r[11].s64 + 21656;
	// 826AF2D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF2D4: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826AF2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF2DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF2E8: 386AC114  addi r3, r10, -0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + -16108;
	// 826AF2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF310: 4BDB7B11  bl 0x82466e20
	ctx.lr = 0x826AF314;
	sub_82466E20(ctx, base);
	// 826AF314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AF328 size=24
    let mut pc: u32 = 0x826AF328;
    'dispatch: loop {
        match pc {
            0x826AF328 => {
    //   block [0x826AF328..0x826AF340)
	// 826AF328: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF32C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826AF330: 394AB670  addi r10, r10, -0x4990
	ctx.r[10].s64 = ctx.r[10].s64 + -18832;
	// 826AF334: 816B5510  lwz r11, 0x5510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21776 as u32) ) } as u64;
	// 826AF338: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826AF33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF340 size=112
    let mut pc: u32 = 0x826AF340;
    'dispatch: loop {
        match pc {
            0x826AF340 => {
    //   block [0x826AF340..0x826AF3B0)
	// 826AF340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF34C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF354: 392BECA4  addi r9, r11, -0x135c
	ctx.r[9].s64 = ctx.r[11].s64 + -4956;
	// 826AF358: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826AF35C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826AF360: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826AF364: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826AF368: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF36C: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 826AF370: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AF374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF378: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AF37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF380: 386AC144  addi r3, r10, -0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -16060;
	// 826AF384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AF388: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AF38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF390: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AF394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF398: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AF39C: 4BDB7A85  bl 0x82466e20
	ctx.lr = 0x826AF3A0;
	sub_82466E20(ctx, base);
	// 826AF3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF3B0 size=108
    let mut pc: u32 = 0x826AF3B0;
    'dispatch: loop {
        match pc {
            0x826AF3B0 => {
    //   block [0x826AF3B0..0x826AF41C)
	// 826AF3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF3BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF3C4: 38EB5514  addi r7, r11, 0x5514
	ctx.r[7].s64 = ctx.r[11].s64 + 21780;
	// 826AF3C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AF3CC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826AF3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF3E0: 386AC174  addi r3, r10, -0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16012;
	// 826AF3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF408: 4BDB7A19  bl 0x82466e20
	ctx.lr = 0x826AF40C;
	sub_82466E20(ctx, base);
	// 826AF40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF420 size=108
    let mut pc: u32 = 0x826AF420;
    'dispatch: loop {
        match pc {
            0x826AF420 => {
    //   block [0x826AF420..0x826AF48C)
	// 826AF420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF42C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF434: 38EB5544  addi r7, r11, 0x5544
	ctx.r[7].s64 = ctx.r[11].s64 + 21828;
	// 826AF438: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AF43C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826AF440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF450: 386AC1A4  addi r3, r10, -0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15964;
	// 826AF454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF478: 4BDB79A9  bl 0x82466e20
	ctx.lr = 0x826AF47C;
	sub_82466E20(ctx, base);
	// 826AF47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF490 size=112
    let mut pc: u32 = 0x826AF490;
    'dispatch: loop {
        match pc {
            0x826AF490 => {
    //   block [0x826AF490..0x826AF500)
	// 826AF490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF4A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF4A4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF4AC: 390B5578  addi r8, r11, 0x5578
	ctx.r[8].s64 = ctx.r[11].s64 + 21880;
	// 826AF4B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AF4B4: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826AF4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF4BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF4C8: 386AC1D4  addi r3, r10, -0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15916;
	// 826AF4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF4EC: 4BDB7935  bl 0x82466e20
	ctx.lr = 0x826AF4F0;
	sub_82466E20(ctx, base);
	// 826AF4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF500 size=108
    let mut pc: u32 = 0x826AF500;
    'dispatch: loop {
        match pc {
            0x826AF500 => {
    //   block [0x826AF500..0x826AF56C)
	// 826AF500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF50C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF514: 38EB55D8  addi r7, r11, 0x55d8
	ctx.r[7].s64 = ctx.r[11].s64 + 21976;
	// 826AF518: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF51C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826AF520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF530: 386AC204  addi r3, r10, -0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15868;
	// 826AF534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF558: 4BDB78C9  bl 0x82466e20
	ctx.lr = 0x826AF55C;
	sub_82466E20(ctx, base);
	// 826AF55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF570 size=112
    let mut pc: u32 = 0x826AF570;
    'dispatch: loop {
        match pc {
            0x826AF570 => {
    //   block [0x826AF570..0x826AF5E0)
	// 826AF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF57C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF580: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF584: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF58C: 390B5650  addi r8, r11, 0x5650
	ctx.r[8].s64 = ctx.r[11].s64 + 22096;
	// 826AF590: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AF594: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826AF598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF59C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF5A8: 386AC234  addi r3, r10, -0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -15820;
	// 826AF5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF5CC: 4BDB7855  bl 0x82466e20
	ctx.lr = 0x826AF5D0;
	sub_82466E20(ctx, base);
	// 826AF5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF5E0 size=112
    let mut pc: u32 = 0x826AF5E0;
    'dispatch: loop {
        match pc {
            0x826AF5E0 => {
    //   block [0x826AF5E0..0x826AF650)
	// 826AF5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF5EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF5F0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF5F4: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF5FC: 390B56F8  addi r8, r11, 0x56f8
	ctx.r[8].s64 = ctx.r[11].s64 + 22264;
	// 826AF600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF604: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826AF608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF60C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF618: 386AC264  addi r3, r10, -0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15772;
	// 826AF61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF63C: 4BDB77E5  bl 0x82466e20
	ctx.lr = 0x826AF640;
	sub_82466E20(ctx, base);
	// 826AF640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF650 size=108
    let mut pc: u32 = 0x826AF650;
    'dispatch: loop {
        match pc {
            0x826AF650 => {
    //   block [0x826AF650..0x826AF6BC)
	// 826AF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF65C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF664: 38EB5710  addi r7, r11, 0x5710
	ctx.r[7].s64 = ctx.r[11].s64 + 22288;
	// 826AF668: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF66C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826AF670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF680: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 826AF684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF6A8: 4BDB7779  bl 0x82466e20
	ctx.lr = 0x826AF6AC;
	sub_82466E20(ctx, base);
	// 826AF6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF6C0 size=112
    let mut pc: u32 = 0x826AF6C0;
    'dispatch: loop {
        match pc {
            0x826AF6C0 => {
    //   block [0x826AF6C0..0x826AF730)
	// 826AF6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF6CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF6D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF6D4: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF6DC: 390B5788  addi r8, r11, 0x5788
	ctx.r[8].s64 = ctx.r[11].s64 + 22408;
	// 826AF6E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AF6E4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826AF6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF6EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF6F8: 386AC2C4  addi r3, r10, -0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15676;
	// 826AF6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF71C: 4BDB7705  bl 0x82466e20
	ctx.lr = 0x826AF720;
	sub_82466E20(ctx, base);
	// 826AF720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF730 size=108
    let mut pc: u32 = 0x826AF730;
    'dispatch: loop {
        match pc {
            0x826AF730 => {
    //   block [0x826AF730..0x826AF79C)
	// 826AF730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF73C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF744: 38EB5830  addi r7, r11, 0x5830
	ctx.r[7].s64 = ctx.r[11].s64 + 22576;
	// 826AF748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AF74C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826AF750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF760: 386AC2F4  addi r3, r10, -0x3d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15628;
	// 826AF764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF788: 4BDB7699  bl 0x82466e20
	ctx.lr = 0x826AF78C;
	sub_82466E20(ctx, base);
	// 826AF78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF7A0 size=108
    let mut pc: u32 = 0x826AF7A0;
    'dispatch: loop {
        match pc {
            0x826AF7A0 => {
    //   block [0x826AF7A0..0x826AF80C)
	// 826AF7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF7AC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF7B4: 38EB5848  addi r7, r11, 0x5848
	ctx.r[7].s64 = ctx.r[11].s64 + 22600;
	// 826AF7B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AF7BC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826AF7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF7C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF7D0: 386AC324  addi r3, r10, -0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -15580;
	// 826AF7D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF7F8: 4BDB7629  bl 0x82466e20
	ctx.lr = 0x826AF7FC;
	sub_82466E20(ctx, base);
	// 826AF7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF810 size=116
    let mut pc: u32 = 0x826AF810;
    'dispatch: loop {
        match pc {
            0x826AF810 => {
    //   block [0x826AF810..0x826AF884)
	// 826AF810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF81C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF820: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AF824: 390B58A8  addi r8, r11, 0x58a8
	ctx.r[8].s64 = ctx.r[11].s64 + 22696;
	// 826AF828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF82C: 392AECE0  addi r9, r10, -0x1320
	ctx.r[9].s64 = ctx.r[10].s64 + -4896;
	// 826AF830: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AF838: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF83C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF844: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF854: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AF858: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826AF85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AF860: 386BC354  addi r3, r11, -0x3cac
	ctx.r[3].s64 = ctx.r[11].s64 + -15532;
	// 826AF864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AF868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF870: 4BDB75B1  bl 0x82466e20
	ctx.lr = 0x826AF874;
	sub_82466E20(ctx, base);
	// 826AF874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF888 size=108
    let mut pc: u32 = 0x826AF888;
    'dispatch: loop {
        match pc {
            0x826AF888 => {
    //   block [0x826AF888..0x826AF8F4)
	// 826AF888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF894: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF89C: 38EB58C0  addi r7, r11, 0x58c0
	ctx.r[7].s64 = ctx.r[11].s64 + 22720;
	// 826AF8A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AF8A4: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826AF8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF8AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF8B8: 386AC384  addi r3, r10, -0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15484;
	// 826AF8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF8E0: 4BDB7541  bl 0x82466e20
	ctx.lr = 0x826AF8E4;
	sub_82466E20(ctx, base);
	// 826AF8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF8F8 size=108
    let mut pc: u32 = 0x826AF8F8;
    'dispatch: loop {
        match pc {
            0x826AF8F8 => {
    //   block [0x826AF8F8..0x826AF964)
	// 826AF8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF904: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF90C: 38EB5908  addi r7, r11, 0x5908
	ctx.r[7].s64 = ctx.r[11].s64 + 22792;
	// 826AF910: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826AF914: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826AF918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF91C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF928: 386AC3B4  addi r3, r10, -0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15436;
	// 826AF92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF950: 4BDB74D1  bl 0x82466e20
	ctx.lr = 0x826AF954;
	sub_82466E20(ctx, base);
	// 826AF954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF968 size=108
    let mut pc: u32 = 0x826AF968;
    'dispatch: loop {
        match pc {
            0x826AF968 => {
    //   block [0x826AF968..0x826AF9D4)
	// 826AF968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF974: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF97C: 38EB5998  addi r7, r11, 0x5998
	ctx.r[7].s64 = ctx.r[11].s64 + 22936;
	// 826AF980: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826AF984: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826AF988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF98C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF998: 386AC3E4  addi r3, r10, -0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15388;
	// 826AF99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF9C0: 4BDB7461  bl 0x82466e20
	ctx.lr = 0x826AF9C4;
	sub_82466E20(ctx, base);
	// 826AF9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF9D8 size=100
    let mut pc: u32 = 0x826AF9D8;
    'dispatch: loop {
        match pc {
            0x826AF9D8 => {
    //   block [0x826AF9D8..0x826AFA3C)
	// 826AF9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF9E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF9EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF9F8: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826AF9FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFA0C: 386AC414  addi r3, r10, -0x3bec
	ctx.r[3].s64 = ctx.r[10].s64 + -15340;
	// 826AFA10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFA14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFA18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFA20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFA28: 4BDB73F9  bl 0x82466e20
	ctx.lr = 0x826AFA2C;
	sub_82466E20(ctx, base);
	// 826AFA2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFA40 size=112
    let mut pc: u32 = 0x826AFA40;
    'dispatch: loop {
        match pc {
            0x826AFA40 => {
    //   block [0x826AFA40..0x826AFAB0)
	// 826AFA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFA4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA50: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFA54: 38AAC414  addi r5, r10, -0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + -15340;
	// 826AFA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFA5C: 390B5A28  addi r8, r11, 0x5a28
	ctx.r[8].s64 = ctx.r[11].s64 + 23080;
	// 826AFA60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AFA64: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826AFA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFA6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFA78: 386AC444  addi r3, r10, -0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15292;
	// 826AFA7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AFA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFA9C: 4BDB7385  bl 0x82466e20
	ctx.lr = 0x826AFAA0;
	sub_82466E20(ctx, base);
	// 826AFAA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFAB0 size=108
    let mut pc: u32 = 0x826AFAB0;
    'dispatch: loop {
        match pc {
            0x826AFAB0 => {
    //   block [0x826AFAB0..0x826AFB1C)
	// 826AFAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFABC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFAC4: 38EB5A88  addi r7, r11, 0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + 23176;
	// 826AFAC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AFACC: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826AFAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFAD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFAE0: 386AC474  addi r3, r10, -0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15244;
	// 826AFAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFB08: 4BDB7319  bl 0x82466e20
	ctx.lr = 0x826AFB0C;
	sub_82466E20(ctx, base);
	// 826AFB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFB20 size=108
    let mut pc: u32 = 0x826AFB20;
    'dispatch: loop {
        match pc {
            0x826AFB20 => {
    //   block [0x826AFB20..0x826AFB8C)
	// 826AFB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFB2C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFB34: 38EB5AB8  addi r7, r11, 0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + 23224;
	// 826AFB38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AFB3C: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826AFB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFB44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFB50: 386AC4A4  addi r3, r10, -0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15196;
	// 826AFB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFB78: 4BDB72A9  bl 0x82466e20
	ctx.lr = 0x826AFB7C;
	sub_82466E20(ctx, base);
	// 826AFB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFB90 size=108
    let mut pc: u32 = 0x826AFB90;
    'dispatch: loop {
        match pc {
            0x826AFB90 => {
    //   block [0x826AFB90..0x826AFBFC)
	// 826AFB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFB9C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFBA4: 38EB5B18  addi r7, r11, 0x5b18
	ctx.r[7].s64 = ctx.r[11].s64 + 23320;
	// 826AFBA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AFBAC: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826AFBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFBB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFBC0: 386AC4D4  addi r3, r10, -0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15148;
	// 826AFBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFBE8: 4BDB7239  bl 0x82466e20
	ctx.lr = 0x826AFBEC;
	sub_82466E20(ctx, base);
	// 826AFBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFC00 size=112
    let mut pc: u32 = 0x826AFC00;
    'dispatch: loop {
        match pc {
            0x826AFC00 => {
    //   block [0x826AFC00..0x826AFC70)
	// 826AFC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFC0C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AFC10: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFC14: 392AED14  addi r9, r10, -0x12ec
	ctx.r[9].s64 = ctx.r[10].s64 + -4844;
	// 826AFC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFC1C: 390B5B80  addi r8, r11, 0x5b80
	ctx.r[8].s64 = ctx.r[11].s64 + 23424;
	// 826AFC20: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826AFC24: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826AFC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFC30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFC38: 386AC504  addi r3, r10, -0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + -15100;
	// 826AFC3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFC40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AFC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFC5C: 4BDB71C5  bl 0x82466e20
	ctx.lr = 0x826AFC60;
	sub_82466E20(ctx, base);
	// 826AFC60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFC70 size=108
    let mut pc: u32 = 0x826AFC70;
    'dispatch: loop {
        match pc {
            0x826AFC70 => {
    //   block [0x826AFC70..0x826AFCDC)
	// 826AFC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFC7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFC84: 38EB5C88  addi r7, r11, 0x5c88
	ctx.r[7].s64 = ctx.r[11].s64 + 23688;
	// 826AFC88: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826AFC8C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826AFC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFC94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFCA0: 386AC534  addi r3, r10, -0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + -15052;
	// 826AFCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFCC8: 4BDB7159  bl 0x82466e20
	ctx.lr = 0x826AFCCC;
	sub_82466E20(ctx, base);
	// 826AFCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFCE0 size=112
    let mut pc: u32 = 0x826AFCE0;
    'dispatch: loop {
        match pc {
            0x826AFCE0 => {
    //   block [0x826AFCE0..0x826AFD50)
	// 826AFCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFCEC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AFCF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFCF4: 392AED30  addi r9, r10, -0x12d0
	ctx.r[9].s64 = ctx.r[10].s64 + -4816;
	// 826AFCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFCFC: 390B5DA8  addi r8, r11, 0x5da8
	ctx.r[8].s64 = ctx.r[11].s64 + 23976;
	// 826AFD00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AFD04: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826AFD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFD0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFD18: 386AC564  addi r3, r10, -0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15004;
	// 826AFD1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFD20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AFD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFD3C: 4BDB70E5  bl 0x82466e20
	ctx.lr = 0x826AFD40;
	sub_82466E20(ctx, base);
	// 826AFD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFD50 size=100
    let mut pc: u32 = 0x826AFD50;
    'dispatch: loop {
        match pc {
            0x826AFD50 => {
    //   block [0x826AFD50..0x826AFDB4)
	// 826AFD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFD5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFD64: 38AACAA4  addi r5, r10, -0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + -13660;
	// 826AFD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFD70: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826AFD74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFD84: 386AC594  addi r3, r10, -0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -14956;
	// 826AFD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFDA0: 4BDB7081  bl 0x82466e20
	ctx.lr = 0x826AFDA4;
	sub_82466E20(ctx, base);
	// 826AFDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFDB8 size=116
    let mut pc: u32 = 0x826AFDB8;
    'dispatch: loop {
        match pc {
            0x826AFDB8 => {
    //   block [0x826AFDB8..0x826AFE2C)
	// 826AFDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFDC4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AFDC8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AFDCC: 390A5DC0  addi r8, r10, 0x5dc0
	ctx.r[8].s64 = ctx.r[10].s64 + 24000;
	// 826AFDD0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFDD4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AFDD8: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826AFDDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFDE0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFDEC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826AFDF0: 396BED44  addi r11, r11, -0x12bc
	ctx.r[11].s64 = ctx.r[11].s64 + -4796;
	// 826AFDF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFDF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFDFC: 386AC5C4  addi r3, r10, -0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -14908;
	// 826AFE00: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AFE04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFE08: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AFE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFE18: 4BDB7009  bl 0x82466e20
	ctx.lr = 0x826AFE1C;
	sub_82466E20(ctx, base);
	// 826AFE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFE30 size=100
    let mut pc: u32 = 0x826AFE30;
    'dispatch: loop {
        match pc {
            0x826AFE30 => {
    //   block [0x826AFE30..0x826AFE94)
	// 826AFE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFE44: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 826AFE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFE50: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826AFE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFE64: 386AC5F4  addi r3, r10, -0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -14860;
	// 826AFE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFE6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFE70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFE78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFE80: 4BDB6FA1  bl 0x82466e20
	ctx.lr = 0x826AFE84;
	sub_82466E20(ctx, base);
	// 826AFE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFE98 size=112
    let mut pc: u32 = 0x826AFE98;
    'dispatch: loop {
        match pc {
            0x826AFE98 => {
    //   block [0x826AFE98..0x826AFF08)
	// 826AFE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFEA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFEAC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AFEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFEB4: 390B5E50  addi r8, r11, 0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + 24144;
	// 826AFEB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AFEBC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826AFEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFEC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFED0: 386AC624  addi r3, r10, -0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14812;
	// 826AFED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AFED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFEF4: 4BDB6F2D  bl 0x82466e20
	ctx.lr = 0x826AFEF8;
	sub_82466E20(ctx, base);
	// 826AFEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFF08 size=116
    let mut pc: u32 = 0x826AFF08;
    'dispatch: loop {
        match pc {
            0x826AFF08 => {
    //   block [0x826AFF08..0x826AFF7C)
	// 826AFF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFF14: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AFF18: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AFF1C: 390A5E98  addi r8, r10, 0x5e98
	ctx.r[8].s64 = ctx.r[10].s64 + 24216;
	// 826AFF20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFF24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AFF28: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826AFF2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFF30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFF3C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826AFF40: 396BED64  addi r11, r11, -0x129c
	ctx.r[11].s64 = ctx.r[11].s64 + -4764;
	// 826AFF44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFF48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFF4C: 386AC654  addi r3, r10, -0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + -14764;
	// 826AFF50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AFF54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFF58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AFF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFF68: 4BDB6EB9  bl 0x82466e20
	ctx.lr = 0x826AFF6C;
	sub_82466E20(ctx, base);
	// 826AFF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFF80 size=108
    let mut pc: u32 = 0x826AFF80;
    'dispatch: loop {
        match pc {
            0x826AFF80 => {
    //   block [0x826AFF80..0x826AFFEC)
	// 826AFF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFF8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFF94: 38EB5F28  addi r7, r11, 0x5f28
	ctx.r[7].s64 = ctx.r[11].s64 + 24360;
	// 826AFF98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AFF9C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826AFFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFFA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFFA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFFB0: 386AC684  addi r3, r10, -0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + -14716;
	// 826AFFB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFFD8: 4BDB6E49  bl 0x82466e20
	ctx.lr = 0x826AFFDC;
	sub_82466E20(ctx, base);
	// 826AFFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AFFF0 size=24
    let mut pc: u32 = 0x826AFFF0;
    'dispatch: loop {
        match pc {
            0x826AFFF0 => {
    //   block [0x826AFFF0..0x826B0008)
	// 826AFFF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFFF4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826AFFF8: 394AB6B8  addi r10, r10, -0x4948
	ctx.r[10].s64 = ctx.r[10].s64 + -18760;
	// 826AFFFC: 816B5F70  lwz r11, 0x5f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24432 as u32) ) } as u64;
	// 826B0000: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B0004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0008 size=116
    let mut pc: u32 = 0x826B0008;
    'dispatch: loop {
        match pc {
            0x826B0008 => {
    //   block [0x826B0008..0x826B007C)
	// 826B0008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0014: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0018: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B001C: 392BED98  addi r9, r11, -0x1268
	ctx.r[9].s64 = ctx.r[11].s64 + -4712;
	// 826B0020: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0024: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0028: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B002C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 826B0030: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B0034: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826B0038: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B003C: 396BB6B8  addi r11, r11, -0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + -18760;
	// 826B0040: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B0044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0048: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B004C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0050: 386AC6B4  addi r3, r10, -0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + -14668;
	// 826B0054: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0058: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0060: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B0064: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0068: 4BDB6DB9  bl 0x82466e20
	ctx.lr = 0x826B006C;
	sub_82466E20(ctx, base);
	// 826B006C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0080 size=112
    let mut pc: u32 = 0x826B0080;
    'dispatch: loop {
        match pc {
            0x826B0080 => {
    //   block [0x826B0080..0x826B00F0)
	// 826B0080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B008C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0090: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0094: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B009C: 390B5F74  addi r8, r11, 0x5f74
	ctx.r[8].s64 = ctx.r[11].s64 + 24436;
	// 826B00A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B00A4: 388A596C  addi r4, r10, 0x596c
	ctx.r[4].s64 = ctx.r[10].s64 + 22892;
	// 826B00A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B00AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B00B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B00B8: 386AC6E4  addi r3, r10, -0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + -14620;
	// 826B00BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B00C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B00C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B00C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B00CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B00D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B00D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B00D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B00DC: 4BDB6D45  bl 0x82466e20
	ctx.lr = 0x826B00E0;
	sub_82466E20(ctx, base);
	// 826B00E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B00E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B00E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B00EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B00F0 size=112
    let mut pc: u32 = 0x826B00F0;
    'dispatch: loop {
        match pc {
            0x826B00F0 => {
    //   block [0x826B00F0..0x826B0160)
	// 826B00F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B00F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B00F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B00FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0100: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0104: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B010C: 390B5FA4  addi r8, r11, 0x5fa4
	ctx.r[8].s64 = ctx.r[11].s64 + 24484;
	// 826B0110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B0114: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826B0118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B011C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0128: 386AC714  addi r3, r10, -0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14572;
	// 826B012C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B013C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B014C: 4BDB6CD5  bl 0x82466e20
	ctx.lr = 0x826B0150;
	sub_82466E20(ctx, base);
	// 826B0150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0160 size=100
    let mut pc: u32 = 0x826B0160;
    'dispatch: loop {
        match pc {
            0x826B0160 => {
    //   block [0x826B0160..0x826B01C4)
	// 826B0160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B016C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0174: 38AACAA4  addi r5, r10, -0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + -13660;
	// 826B0178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B017C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0180: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826B0184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B018C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0194: 386AC744  addi r3, r10, -0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14524;
	// 826B0198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B019C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B01A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B01A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B01A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B01AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B01B0: 4BDB6C71  bl 0x82466e20
	ctx.lr = 0x826B01B4;
	sub_82466E20(ctx, base);
	// 826B01B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B01B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B01BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B01C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B01C8 size=116
    let mut pc: u32 = 0x826B01C8;
    'dispatch: loop {
        match pc {
            0x826B01C8 => {
    //   block [0x826B01C8..0x826B023C)
	// 826B01C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B01CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B01D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B01D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B01D8: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826B01DC: 390A5FC0  addi r8, r10, 0x5fc0
	ctx.r[8].s64 = ctx.r[10].s64 + 24512;
	// 826B01E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B01E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B01E8: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B01EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B01F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B01F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B01F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B01FC: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826B0200: 396BEDE0  addi r11, r11, -0x1220
	ctx.r[11].s64 = ctx.r[11].s64 + -4640;
	// 826B0204: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0208: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B020C: 386AC774  addi r3, r10, -0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + -14476;
	// 826B0210: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0218: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B021C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0228: 4BDB6BF9  bl 0x82466e20
	ctx.lr = 0x826B022C;
	sub_82466E20(ctx, base);
	// 826B022C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0240 size=112
    let mut pc: u32 = 0x826B0240;
    'dispatch: loop {
        match pc {
            0x826B0240 => {
    //   block [0x826B0240..0x826B02B0)
	// 826B0240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B024C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0250: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0254: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B025C: 390B6158  addi r8, r11, 0x6158
	ctx.r[8].s64 = ctx.r[11].s64 + 24920;
	// 826B0260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B0264: 388A5998  addi r4, r10, 0x5998
	ctx.r[4].s64 = ctx.r[10].s64 + 22936;
	// 826B0268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B026C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0278: 386AC7A4  addi r3, r10, -0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + -14428;
	// 826B027C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B028C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B029C: 4BDB6B85  bl 0x82466e20
	ctx.lr = 0x826B02A0;
	sub_82466E20(ctx, base);
	// 826B02A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B02A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B02A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B02AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B02B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B02B0 size=112
    let mut pc: u32 = 0x826B02B0;
    'dispatch: loop {
        match pc {
            0x826B02B0 => {
    //   block [0x826B02B0..0x826B0320)
	// 826B02B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B02B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B02B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B02BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B02C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B02C4: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B02C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B02CC: 390B6170  addi r8, r11, 0x6170
	ctx.r[8].s64 = ctx.r[11].s64 + 24944;
	// 826B02D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B02D4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826B02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B02DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B02E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B02E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B02E8: 386AC7D4  addi r3, r10, -0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + -14380;
	// 826B02EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B02F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B02F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B02F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B02FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B030C: 4BDB6B15  bl 0x82466e20
	ctx.lr = 0x826B0310;
	sub_82466E20(ctx, base);
	// 826B0310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0320 size=112
    let mut pc: u32 = 0x826B0320;
    'dispatch: loop {
        match pc {
            0x826B0320 => {
    //   block [0x826B0320..0x826B0390)
	// 826B0320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B032C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0330: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0334: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B033C: 390B6218  addi r8, r11, 0x6218
	ctx.r[8].s64 = ctx.r[11].s64 + 25112;
	// 826B0340: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B0344: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826B0348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B034C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0358: 386AC804  addi r3, r10, -0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14332;
	// 826B035C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B037C: 4BDB6AA5  bl 0x82466e20
	ctx.lr = 0x826B0380;
	sub_82466E20(ctx, base);
	// 826B0380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0390 size=112
    let mut pc: u32 = 0x826B0390;
    'dispatch: loop {
        match pc {
            0x826B0390 => {
    //   block [0x826B0390..0x826B0400)
	// 826B0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B039C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B03A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B03A4: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B03A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B03AC: 390B62F0  addi r8, r11, 0x62f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25328;
	// 826B03B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826B03B4: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826B03B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B03BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B03C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B03C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B03C8: 386AC834  addi r3, r10, -0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14284;
	// 826B03CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B03D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B03D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B03D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B03DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B03E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B03E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B03E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B03EC: 4BDB6A35  bl 0x82466e20
	ctx.lr = 0x826B03F0;
	sub_82466E20(ctx, base);
	// 826B03F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B03F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B03F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B03FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0400 size=112
    let mut pc: u32 = 0x826B0400;
    'dispatch: loop {
        match pc {
            0x826B0400 => {
    //   block [0x826B0400..0x826B0470)
	// 826B0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B040C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0410: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0414: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B041C: 390B63F8  addi r8, r11, 0x63f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25592;
	// 826B0420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B0424: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826B0428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B042C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0438: 386AC864  addi r3, r10, -0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + -14236;
	// 826B043C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B044C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B045C: 4BDB69C5  bl 0x82466e20
	ctx.lr = 0x826B0460;
	sub_82466E20(ctx, base);
	// 826B0460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B0470 size=24
    let mut pc: u32 = 0x826B0470;
    'dispatch: loop {
        match pc {
            0x826B0470 => {
    //   block [0x826B0470..0x826B0488)
	// 826B0470: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0474: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B0478: 394AB7D8  addi r10, r10, -0x4828
	ctx.r[10].s64 = ctx.r[10].s64 + -18472;
	// 826B047C: 816B5FBC  lwz r11, 0x5fbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24508 as u32) ) } as u64;
	// 826B0480: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B0484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0488 size=116
    let mut pc: u32 = 0x826B0488;
    'dispatch: loop {
        match pc {
            0x826B0488 => {
    //   block [0x826B0488..0x826B04FC)
	// 826B0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0494: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0498: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B049C: 392BEE3C  addi r9, r11, -0x11c4
	ctx.r[9].s64 = ctx.r[11].s64 + -4548;
	// 826B04A0: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B04A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B04A8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B04AC: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826B04B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B04B4: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826B04B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B04BC: 396BB7D8  addi r11, r11, -0x4828
	ctx.r[11].s64 = ctx.r[11].s64 + -18472;
	// 826B04C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B04C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B04C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B04CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B04D0: 386AC894  addi r3, r10, -0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + -14188;
	// 826B04D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B04D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B04DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B04E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B04E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B04E8: 4BDB6939  bl 0x82466e20
	ctx.lr = 0x826B04EC;
	sub_82466E20(ctx, base);
	// 826B04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0500 size=116
    let mut pc: u32 = 0x826B0500;
    'dispatch: loop {
        match pc {
            0x826B0500 => {
    //   block [0x826B0500..0x826B0574)
	// 826B0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B050C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B0510: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B0514: 390A6428  addi r8, r10, 0x6428
	ctx.r[8].s64 = ctx.r[10].s64 + 25640;
	// 826B0518: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B051C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0520: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B0524: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0528: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B052C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0534: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826B0538: 396BEEAC  addi r11, r11, -0x1154
	ctx.r[11].s64 = ctx.r[11].s64 + -4436;
	// 826B053C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0544: 386AC8C4  addi r3, r10, -0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + -14140;
	// 826B0548: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B054C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0550: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B055C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0560: 4BDB68C1  bl 0x82466e20
	ctx.lr = 0x826B0564;
	sub_82466E20(ctx, base);
	// 826B0564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B056C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0578 size=108
    let mut pc: u32 = 0x826B0578;
    'dispatch: loop {
        match pc {
            0x826B0578 => {
    //   block [0x826B0578..0x826B05E4)
	// 826B0578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0584: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B058C: 38EB6458  addi r7, r11, 0x6458
	ctx.r[7].s64 = ctx.r[11].s64 + 25688;
	// 826B0590: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B0594: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826B0598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B059C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B05A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B05A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B05A8: 386AC8F4  addi r3, r10, -0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + -14092;
	// 826B05AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B05B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B05B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B05B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B05BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B05C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B05C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B05C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B05CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B05D0: 4BDB6851  bl 0x82466e20
	ctx.lr = 0x826B05D4;
	sub_82466E20(ctx, base);
	// 826B05D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B05D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B05DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B05E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B05E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B05E8 size=24
    let mut pc: u32 = 0x826B05E8;
    'dispatch: loop {
        match pc {
            0x826B05E8 => {
    //   block [0x826B05E8..0x826B0600)
	// 826B05E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B05EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B05F0: 394AB970  addi r10, r10, -0x4690
	ctx.r[10].s64 = ctx.r[10].s64 + -18064;
	// 826B05F4: 816B64E8  lwz r11, 0x64e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25832 as u32) ) } as u64;
	// 826B05F8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B05FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0600 size=116
    let mut pc: u32 = 0x826B0600;
    'dispatch: loop {
        match pc {
            0x826B0600 => {
    //   block [0x826B0600..0x826B0674)
	// 826B0600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B060C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0610: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0614: 392BEED0  addi r9, r11, -0x1130
	ctx.r[9].s64 = ctx.r[11].s64 + -4400;
	// 826B0618: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B061C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0620: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826B0624: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826B0628: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B062C: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826B0630: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0634: 396BB970  addi r11, r11, -0x4690
	ctx.r[11].s64 = ctx.r[11].s64 + -18064;
	// 826B0638: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B063C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0640: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B0644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0648: 386AC924  addi r3, r10, -0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14044;
	// 826B064C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0650: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B0654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0658: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B065C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0660: 4BDB67C1  bl 0x82466e20
	ctx.lr = 0x826B0664;
	sub_82466E20(ctx, base);
	// 826B0664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B066C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0678 size=112
    let mut pc: u32 = 0x826B0678;
    'dispatch: loop {
        match pc {
            0x826B0678 => {
    //   block [0x826B0678..0x826B06E8)
	// 826B0678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B067C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0688: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B068C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0694: 390B64EC  addi r8, r11, 0x64ec
	ctx.r[8].s64 = ctx.r[11].s64 + 25836;
	// 826B0698: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B069C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826B06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B06A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B06A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B06AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B06B0: 386AC954  addi r3, r10, -0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13996;
	// 826B06B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B06B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B06BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B06C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B06CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B06D4: 4BDB674D  bl 0x82466e20
	ctx.lr = 0x826B06D8;
	sub_82466E20(ctx, base);
	// 826B06D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B06DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B06E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B06E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B06E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B06E8 size=116
    let mut pc: u32 = 0x826B06E8;
    'dispatch: loop {
        match pc {
            0x826B06E8 => {
    //   block [0x826B06E8..0x826B075C)
	// 826B06E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B06EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B06F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B06F4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B06F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B06FC: 390A6520  addi r8, r10, 0x6520
	ctx.r[8].s64 = ctx.r[10].s64 + 25888;
	// 826B0700: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0704: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0708: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B070C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0710: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B071C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826B0720: 396BEF28  addi r11, r11, -0x10d8
	ctx.r[11].s64 = ctx.r[11].s64 + -4312;
	// 826B0724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0728: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B072C: 386AC984  addi r3, r10, -0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + -13948;
	// 826B0730: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0738: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0748: 4BDB66D9  bl 0x82466e20
	ctx.lr = 0x826B074C;
	sub_82466E20(ctx, base);
	// 826B074C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0760 size=108
    let mut pc: u32 = 0x826B0760;
    'dispatch: loop {
        match pc {
            0x826B0760 => {
    //   block [0x826B0760..0x826B07CC)
	// 826B0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B076C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0774: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 826B0778: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826B077C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826B0780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B078C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0790: 386AC9B4  addi r3, r10, -0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + -13900;
	// 826B0794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B079C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B07A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B07A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B07A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B07AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B07B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B07B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B07B8: 4BDB6669  bl 0x82466e20
	ctx.lr = 0x826B07BC;
	sub_82466E20(ctx, base);
	// 826B07BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B07C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B07C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B07C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B07D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B07D0 size=116
    let mut pc: u32 = 0x826B07D0;
    'dispatch: loop {
        match pc {
            0x826B07D0 => {
    //   block [0x826B07D0..0x826B0844)
	// 826B07D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B07D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B07D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B07DC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B07E0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826B07E4: 390A66D0  addi r8, r10, 0x66d0
	ctx.r[8].s64 = ctx.r[10].s64 + 26320;
	// 826B07E8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B07EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B07F0: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B07F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B07F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B07FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0804: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826B0808: 396BEF40  addi r11, r11, -0x10c0
	ctx.r[11].s64 = ctx.r[11].s64 + -4288;
	// 826B080C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0814: 386AC9E4  addi r3, r10, -0x361c
	ctx.r[3].s64 = ctx.r[10].s64 + -13852;
	// 826B0818: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B081C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0820: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B082C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0830: 4BDB65F1  bl 0x82466e20
	ctx.lr = 0x826B0834;
	sub_82466E20(ctx, base);
	// 826B0834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B083C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0848 size=112
    let mut pc: u32 = 0x826B0848;
    'dispatch: loop {
        match pc {
            0x826B0848 => {
    //   block [0x826B0848..0x826B08B8)
	// 826B0848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B085C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0864: 390B67C0  addi r8, r11, 0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26560;
	// 826B0868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B086C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826B0870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0880: 386ACA14  addi r3, r10, -0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13804;
	// 826B0884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B088C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B089C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B08A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B08A4: 4BDB657D  bl 0x82466e20
	ctx.lr = 0x826B08A8;
	sub_82466E20(ctx, base);
	// 826B08A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B08AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B08B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B08B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B08B8 size=112
    let mut pc: u32 = 0x826B08B8;
    'dispatch: loop {
        match pc {
            0x826B08B8 => {
    //   block [0x826B08B8..0x826B0928)
	// 826B08B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B08BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B08C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B08C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B08C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B08CC: 38AAC804  addi r5, r10, -0x37fc
	ctx.r[5].s64 = ctx.r[10].s64 + -14332;
	// 826B08D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B08D4: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 826B08D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B08DC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826B08E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B08E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B08E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B08EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B08F0: 386ACA44  addi r3, r10, -0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13756;
	// 826B08F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B08F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B08FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B090C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0914: 4BDB650D  bl 0x82466e20
	ctx.lr = 0x826B0918;
	sub_82466E20(ctx, base);
	// 826B0918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B091C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0928 size=112
    let mut pc: u32 = 0x826B0928;
    'dispatch: loop {
        match pc {
            0x826B0928 => {
    //   block [0x826B0928..0x826B0998)
	// 826B0928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B093C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0944: 390B6868  addi r8, r11, 0x6868
	ctx.r[8].s64 = ctx.r[11].s64 + 26728;
	// 826B0948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B094C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826B0950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B095C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0960: 386ACA74  addi r3, r10, -0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + -13708;
	// 826B0964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B096C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B097C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0984: 4BDB649D  bl 0x82466e20
	ctx.lr = 0x826B0988;
	sub_82466E20(ctx, base);
	// 826B0988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B098C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0998 size=116
    let mut pc: u32 = 0x826B0998;
    'dispatch: loop {
        match pc {
            0x826B0998 => {
    //   block [0x826B0998..0x826B0A0C)
	// 826B0998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B099C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B09A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B09A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B09A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B09AC: 390B6880  addi r8, r11, 0x6880
	ctx.r[8].s64 = ctx.r[11].s64 + 26752;
	// 826B09B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B09B4: 392AEF8C  addi r9, r10, -0x1074
	ctx.r[9].s64 = ctx.r[10].s64 + -4212;
	// 826B09B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B09BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B09C0: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B09C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B09C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B09CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B09D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B09D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B09D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B09DC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B09E0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826B09E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B09E8: 386BCAA4  addi r3, r11, -0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + -13660;
	// 826B09EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B09F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B09F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B09F8: 4BDB6429  bl 0x82466e20
	ctx.lr = 0x826B09FC;
	sub_82466E20(ctx, base);
	// 826B09FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0A10 size=108
    let mut pc: u32 = 0x826B0A10;
    'dispatch: loop {
        match pc {
            0x826B0A10 => {
    //   block [0x826B0A10..0x826B0A7C)
	// 826B0A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0A1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0A24: 38EB6898  addi r7, r11, 0x6898
	ctx.r[7].s64 = ctx.r[11].s64 + 26776;
	// 826B0A28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B0A2C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826B0A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0A40: 386ACAD4  addi r3, r10, -0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + -13612;
	// 826B0A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0A68: 4BDB63B9  bl 0x82466e20
	ctx.lr = 0x826B0A6C;
	sub_82466E20(ctx, base);
	// 826B0A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0A80 size=108
    let mut pc: u32 = 0x826B0A80;
    'dispatch: loop {
        match pc {
            0x826B0A80 => {
    //   block [0x826B0A80..0x826B0AEC)
	// 826B0A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0A8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0A94: 38EB68E0  addi r7, r11, 0x68e0
	ctx.r[7].s64 = ctx.r[11].s64 + 26848;
	// 826B0A98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B0A9C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826B0AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0AB0: 386ACB04  addi r3, r10, -0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13564;
	// 826B0AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0AD8: 4BDB6349  bl 0x82466e20
	ctx.lr = 0x826B0ADC;
	sub_82466E20(ctx, base);
	// 826B0ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0AF0 size=116
    let mut pc: u32 = 0x826B0AF0;
    'dispatch: loop {
        match pc {
            0x826B0AF0 => {
    //   block [0x826B0AF0..0x826B0B64)
	// 826B0AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0AFC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B0B00: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826B0B04: 390A6928  addi r8, r10, 0x6928
	ctx.r[8].s64 = ctx.r[10].s64 + 26920;
	// 826B0B08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0B0C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0B10: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0B14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0B18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0B24: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826B0B28: 396BEFA0  addi r11, r11, -0x1060
	ctx.r[11].s64 = ctx.r[11].s64 + -4192;
	// 826B0B2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0B30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0B34: 386ACB34  addi r3, r10, -0x34cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13516;
	// 826B0B38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0B40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0B50: 4BDB62D1  bl 0x82466e20
	ctx.lr = 0x826B0B54;
	sub_82466E20(ctx, base);
	// 826B0B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0B68 size=108
    let mut pc: u32 = 0x826B0B68;
    'dispatch: loop {
        match pc {
            0x826B0B68 => {
    //   block [0x826B0B68..0x826B0BD4)
	// 826B0B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0B74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B0B7C: 38EB6A48  addi r7, r11, 0x6a48
	ctx.r[7].s64 = ctx.r[11].s64 + 27208;
	// 826B0B80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B0B84: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 826B0B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0B8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0B98: 386ACB64  addi r3, r10, -0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + -13468;
	// 826B0B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0BC0: 4BDB6261  bl 0x82466e20
	ctx.lr = 0x826B0BC4;
	sub_82466E20(ctx, base);
	// 826B0BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0BD8 size=104
    let mut pc: u32 = 0x826B0BD8;
    'dispatch: loop {
        match pc {
            0x826B0BD8 => {
    //   block [0x826B0BD8..0x826B0C40)
	// 826B0BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0BE4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B0BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0BEC: 392AEFF4  addi r9, r10, -0x100c
	ctx.r[9].s64 = ctx.r[10].s64 + -4108;
	// 826B0BF0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0BF8: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B0BFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0C0C: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826B0C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B0C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0C24: 386ACB94  addi r3, r10, -0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + -13420;
	// 826B0C28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0C2C: 4BDB61F5  bl 0x82466e20
	ctx.lr = 0x826B0C30;
	sub_82466E20(ctx, base);
	// 826B0C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B0C40 size=24
    let mut pc: u32 = 0x826B0C40;
    'dispatch: loop {
        match pc {
            0x826B0C40 => {
    //   block [0x826B0C40..0x826B0C58)
	// 826B0C40: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0C44: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B0C48: 394ABAC0  addi r10, r10, -0x4540
	ctx.r[10].s64 = ctx.r[10].s64 + -17728;
	// 826B0C4C: 816B6A94  lwz r11, 0x6a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27284 as u32) ) } as u64;
	// 826B0C50: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826B0C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0C58 size=116
    let mut pc: u32 = 0x826B0C58;
    'dispatch: loop {
        match pc {
            0x826B0C58 => {
    //   block [0x826B0C58..0x826B0CCC)
	// 826B0C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0C64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B0C68: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B0C6C: 390BBAC0  addi r8, r11, -0x4540
	ctx.r[8].s64 = ctx.r[11].s64 + -17728;
	// 826B0C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0C74: 392AF018  addi r9, r10, -0xfe8
	ctx.r[9].s64 = ctx.r[10].s64 + -4072;
	// 826B0C78: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0C7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B0C80: 38AACB94  addi r5, r10, -0x346c
	ctx.r[5].s64 = ctx.r[10].s64 + -13420;
	// 826B0C84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0C9C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B0CA0: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826B0CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0CA8: 386BCBC4  addi r3, r11, -0x343c
	ctx.r[3].s64 = ctx.r[11].s64 + -13372;
	// 826B0CAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0CB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0CB8: 4BDB6169  bl 0x82466e20
	ctx.lr = 0x826B0CBC;
	sub_82466E20(ctx, base);
	// 826B0CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0CD0 size=108
    let mut pc: u32 = 0x826B0CD0;
    'dispatch: loop {
        match pc {
            0x826B0CD0 => {
    //   block [0x826B0CD0..0x826B0D3C)
	// 826B0CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0CDC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0CE4: 38EB6A98  addi r7, r11, 0x6a98
	ctx.r[7].s64 = ctx.r[11].s64 + 27288;
	// 826B0CE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B0CEC: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826B0CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0D00: 386ACBF4  addi r3, r10, -0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + -13324;
	// 826B0D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0D28: 4BDB60F9  bl 0x82466e20
	ctx.lr = 0x826B0D2C;
	sub_82466E20(ctx, base);
	// 826B0D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0D40 size=108
    let mut pc: u32 = 0x826B0D40;
    'dispatch: loop {
        match pc {
            0x826B0D40 => {
    //   block [0x826B0D40..0x826B0DAC)
	// 826B0D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0D4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0D54: 38EB6AF8  addi r7, r11, 0x6af8
	ctx.r[7].s64 = ctx.r[11].s64 + 27384;
	// 826B0D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B0D5C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826B0D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0D64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0D70: 386ACC24  addi r3, r10, -0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + -13276;
	// 826B0D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0D98: 4BDB6089  bl 0x82466e20
	ctx.lr = 0x826B0D9C;
	sub_82466E20(ctx, base);
	// 826B0D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0DB0 size=112
    let mut pc: u32 = 0x826B0DB0;
    'dispatch: loop {
        match pc {
            0x826B0DB0 => {
    //   block [0x826B0DB0..0x826B0E20)
	// 826B0DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0DBC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B0DC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B0DC4: 38EA6B28  addi r7, r10, 0x6b28
	ctx.r[7].s64 = ctx.r[10].s64 + 27432;
	// 826B0DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0DCC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0DD0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826B0DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0DD8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0DDC: 396BF02C  addi r11, r11, -0xfd4
	ctx.r[11].s64 = ctx.r[11].s64 + -4052;
	// 826B0DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0DE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0DEC: 386ACC54  addi r3, r10, -0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13228;
	// 826B0DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0DF4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0DF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0DFC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0E00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0E04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0E08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0E0C: 4BDB6015  bl 0x82466e20
	ctx.lr = 0x826B0E10;
	sub_82466E20(ctx, base);
	// 826B0E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0E20 size=96
    let mut pc: u32 = 0x826B0E20;
    'dispatch: loop {
        match pc {
            0x826B0E20 => {
    //   block [0x826B0E20..0x826B0E80)
	// 826B0E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0E2C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B0E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0E34: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826B0E38: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0E40: 386ACC84  addi r3, r10, -0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + -13180;
	// 826B0E44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0E4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B0E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B0E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0E6C: 4BDB5FB5  bl 0x82466e20
	ctx.lr = 0x826B0E70;
	sub_82466E20(ctx, base);
	// 826B0E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0E80 size=112
    let mut pc: u32 = 0x826B0E80;
    'dispatch: loop {
        match pc {
            0x826B0E80 => {
    //   block [0x826B0E80..0x826B0EF0)
	// 826B0E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0E8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0E90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0E94: 38AACC84  addi r5, r10, -0x337c
	ctx.r[5].s64 = ctx.r[10].s64 + -13180;
	// 826B0E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B0E9C: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 826B0EA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B0EA4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826B0EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0EAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0EB8: 386ACCB4  addi r3, r10, -0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + -13132;
	// 826B0EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0EDC: 4BDB5F45  bl 0x82466e20
	ctx.lr = 0x826B0EE0;
	sub_82466E20(ctx, base);
	// 826B0EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0EF0 size=112
    let mut pc: u32 = 0x826B0EF0;
    'dispatch: loop {
        match pc {
            0x826B0EF0 => {
    //   block [0x826B0EF0..0x826B0F60)
	// 826B0EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0EFC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B0F00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0F04: 392AF040  addi r9, r10, -0xfc0
	ctx.r[9].s64 = ctx.r[10].s64 + -4032;
	// 826B0F08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B0F0C: 390B6B90  addi r8, r11, 0x6b90
	ctx.r[8].s64 = ctx.r[11].s64 + 27536;
	// 826B0F10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B0F14: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826B0F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0F1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0F28: 386ACCE4  addi r3, r10, -0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + -13084;
	// 826B0F2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0F30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0F4C: 4BDB5ED5  bl 0x82466e20
	ctx.lr = 0x826B0F50;
	sub_82466E20(ctx, base);
	// 826B0F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0F60 size=108
    let mut pc: u32 = 0x826B0F60;
    'dispatch: loop {
        match pc {
            0x826B0F60 => {
    //   block [0x826B0F60..0x826B0FCC)
	// 826B0F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0F6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0F70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B0F74: 38EB6C38  addi r7, r11, 0x6c38
	ctx.r[7].s64 = ctx.r[11].s64 + 27704;
	// 826B0F78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B0F7C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826B0F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0F84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0F90: 386ACD14  addi r3, r10, -0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13036;
	// 826B0F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0FB8: 4BDB5E69  bl 0x82466e20
	ctx.lr = 0x826B0FBC;
	sub_82466E20(ctx, base);
	// 826B0FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0FD0 size=108
    let mut pc: u32 = 0x826B0FD0;
    'dispatch: loop {
        match pc {
            0x826B0FD0 => {
    //   block [0x826B0FD0..0x826B103C)
	// 826B0FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0FDC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0FE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B0FE4: 38EB6C68  addi r7, r11, 0x6c68
	ctx.r[7].s64 = ctx.r[11].s64 + 27752;
	// 826B0FE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B0FEC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826B0FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0FF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1000: 386ACD44  addi r3, r10, -0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12988;
	// 826B1004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B100C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B101C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1028: 4BDB5DF9  bl 0x82466e20
	ctx.lr = 0x826B102C;
	sub_82466E20(ctx, base);
	// 826B102C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B1040 size=28
    let mut pc: u32 = 0x826B1040;
    'dispatch: loop {
        match pc {
            0x826B1040 => {
    //   block [0x826B1040..0x826B105C)
	// 826B1040: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1044: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B1048: 394ABAF0  addi r10, r10, -0x4510
	ctx.r[10].s64 = ctx.r[10].s64 + -17680;
	// 826B104C: 816B6B8C  lwz r11, 0x6b8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27532 as u32) ) } as u64;
	// 826B1050: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B1054: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B1058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1060 size=112
    let mut pc: u32 = 0x826B1060;
    'dispatch: loop {
        match pc {
            0x826B1060 => {
    //   block [0x826B1060..0x826B10D0)
	// 826B1060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B106C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1070: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B1074: 392AF1B8  addi r9, r10, -0xe48
	ctx.r[9].s64 = ctx.r[10].s64 + -3656;
	// 826B1078: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B107C: 390BBAF0  addi r8, r11, -0x4510
	ctx.r[8].s64 = ctx.r[11].s64 + -17680;
	// 826B1080: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826B1084: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826B1088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B108C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1098: 386ACD74  addi r3, r10, -0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + -12940;
	// 826B109C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B10A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826B10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B10A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B10AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B10B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B10B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B10B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B10BC: 4BDB5D65  bl 0x82466e20
	ctx.lr = 0x826B10C0;
	sub_82466E20(ctx, base);
	// 826B10C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B10C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B10C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B10CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B10D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B10D0 size=108
    let mut pc: u32 = 0x826B10D0;
    'dispatch: loop {
        match pc {
            0x826B10D0 => {
    //   block [0x826B10D0..0x826B113C)
	// 826B10D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B10D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B10D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B10DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B10E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B10E4: 38EB6CA0  addi r7, r11, 0x6ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 27808;
	// 826B10E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B10EC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826B10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B10F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B10F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B10FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1100: 386ACDA4  addi r3, r10, -0x325c
	ctx.r[3].s64 = ctx.r[10].s64 + -12892;
	// 826B1104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B110C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B111C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1128: 4BDB5CF9  bl 0x82466e20
	ctx.lr = 0x826B112C;
	sub_82466E20(ctx, base);
	// 826B112C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1140 size=108
    let mut pc: u32 = 0x826B1140;
    'dispatch: loop {
        match pc {
            0x826B1140 => {
    //   block [0x826B1140..0x826B11AC)
	// 826B1140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B114C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1150: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B1154: 38EB6CD0  addi r7, r11, 0x6cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 27856;
	// 826B1158: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B115C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826B1160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B116C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1170: 386ACDD4  addi r3, r10, -0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + -12844;
	// 826B1174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B117C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1198: 4BDB5C89  bl 0x82466e20
	ctx.lr = 0x826B119C;
	sub_82466E20(ctx, base);
	// 826B119C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B11A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B11A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B11A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B11B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B11B0 size=24
    let mut pc: u32 = 0x826B11B0;
    'dispatch: loop {
        match pc {
            0x826B11B0 => {
    //   block [0x826B11B0..0x826B11C8)
	// 826B11B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B11B4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B11B8: 394ABBB0  addi r10, r10, -0x4450
	ctx.r[10].s64 = ctx.r[10].s64 + -17488;
	// 826B11BC: 816B6CE8  lwz r11, 0x6ce8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27880 as u32) ) } as u64;
	// 826B11C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B11C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B11C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B11C8 size=112
    let mut pc: u32 = 0x826B11C8;
    'dispatch: loop {
        match pc {
            0x826B11C8 => {
    //   block [0x826B11C8..0x826B1238)
	// 826B11C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B11CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B11D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B11D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B11D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B11DC: 392AF20C  addi r9, r10, -0xdf4
	ctx.r[9].s64 = ctx.r[10].s64 + -3572;
	// 826B11E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B11E4: 390BBBB0  addi r8, r11, -0x4450
	ctx.r[8].s64 = ctx.r[11].s64 + -17488;
	// 826B11E8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B11EC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826B11F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B11F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B11F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B11FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1200: 386ACE04  addi r3, r10, -0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + -12796;
	// 826B1204: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B120C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B121C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1224: 4BDB5BFD  bl 0x82466e20
	ctx.lr = 0x826B1228;
	sub_82466E20(ctx, base);
	// 826B1228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B122C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1238 size=112
    let mut pc: u32 = 0x826B1238;
    'dispatch: loop {
        match pc {
            0x826B1238 => {
    //   block [0x826B1238..0x826B12A8)
	// 826B1238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B123C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1244: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1248: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B124C: 392AF248  addi r9, r10, -0xdb8
	ctx.r[9].s64 = ctx.r[10].s64 + -3512;
	// 826B1250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1254: 390B6CF8  addi r8, r11, 0x6cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 27896;
	// 826B1258: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B125C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826B1260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B126C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1270: 386ACE34  addi r3, r10, -0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + -12748;
	// 826B1274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1278: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826B127C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B128C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1294: 4BDB5B8D  bl 0x82466e20
	ctx.lr = 0x826B1298;
	sub_82466E20(ctx, base);
	// 826B1298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B129C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B12A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B12A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B12A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B12A8 size=108
    let mut pc: u32 = 0x826B12A8;
    'dispatch: loop {
        match pc {
            0x826B12A8 => {
    //   block [0x826B12A8..0x826B1314)
	// 826B12A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B12AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B12B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B12B4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B12B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B12BC: 38EB6D40  addi r7, r11, 0x6d40
	ctx.r[7].s64 = ctx.r[11].s64 + 27968;
	// 826B12C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B12C4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826B12C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B12CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B12D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B12D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B12D8: 386ACE64  addi r3, r10, -0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + -12700;
	// 826B12DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B12E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B12E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B12E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B12EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B12F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B12F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B12F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B12FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1300: 4BDB5B21  bl 0x82466e20
	ctx.lr = 0x826B1304;
	sub_82466E20(ctx, base);
	// 826B1304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B130C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1318 size=108
    let mut pc: u32 = 0x826B1318;
    'dispatch: loop {
        match pc {
            0x826B1318 => {
    //   block [0x826B1318..0x826B1384)
	// 826B1318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B131C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1324: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1328: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B132C: 38EB6D70  addi r7, r11, 0x6d70
	ctx.r[7].s64 = ctx.r[11].s64 + 28016;
	// 826B1330: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1334: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826B1338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B133C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1348: 386ACE94  addi r3, r10, -0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + -12652;
	// 826B134C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B135C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B136C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1370: 4BDB5AB1  bl 0x82466e20
	ctx.lr = 0x826B1374;
	sub_82466E20(ctx, base);
	// 826B1374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B137C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1388 size=112
    let mut pc: u32 = 0x826B1388;
    'dispatch: loop {
        match pc {
            0x826B1388 => {
    //   block [0x826B1388..0x826B13F8)
	// 826B1388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B138C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1394: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1398: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B139C: 392AF280  addi r9, r10, -0xd80
	ctx.r[9].s64 = ctx.r[10].s64 + -3456;
	// 826B13A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B13A4: 390B6DA0  addi r8, r11, 0x6da0
	ctx.r[8].s64 = ctx.r[11].s64 + 28064;
	// 826B13A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826B13AC: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826B13B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B13B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B13B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B13BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B13C0: 386ACEC4  addi r3, r10, -0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + -12604;
	// 826B13C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B13C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B13CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B13D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B13D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B13D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B13DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B13E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B13E4: 4BDB5A3D  bl 0x82466e20
	ctx.lr = 0x826B13E8;
	sub_82466E20(ctx, base);
	// 826B13E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B13EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B13F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B13F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B13F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B13F8 size=108
    let mut pc: u32 = 0x826B13F8;
    'dispatch: loop {
        match pc {
            0x826B13F8 => {
    //   block [0x826B13F8..0x826B1464)
	// 826B13F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B13FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1404: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1408: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B140C: 38EB6E00  addi r7, r11, 0x6e00
	ctx.r[7].s64 = ctx.r[11].s64 + 28160;
	// 826B1410: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826B1414: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826B1418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B141C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1428: 386ACEF4  addi r3, r10, -0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + -12556;
	// 826B142C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B143C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B144C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1450: 4BDB59D1  bl 0x82466e20
	ctx.lr = 0x826B1454;
	sub_82466E20(ctx, base);
	// 826B1454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B145C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1468 size=108
    let mut pc: u32 = 0x826B1468;
    'dispatch: loop {
        match pc {
            0x826B1468 => {
    //   block [0x826B1468..0x826B14D4)
	// 826B1468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B146C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1474: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1478: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826B147C: 38EB6EF0  addi r7, r11, 0x6ef0
	ctx.r[7].s64 = ctx.r[11].s64 + 28400;
	// 826B1480: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B1484: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826B1488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B148C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1498: 386ACF24  addi r3, r10, -0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12508;
	// 826B149C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B14A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B14A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B14A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B14AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B14B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B14B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B14B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B14BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B14C0: 4BDB5961  bl 0x82466e20
	ctx.lr = 0x826B14C4;
	sub_82466E20(ctx, base);
	// 826B14C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B14C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B14CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B14D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B14D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B14D8 size=108
    let mut pc: u32 = 0x826B14D8;
    'dispatch: loop {
        match pc {
            0x826B14D8 => {
    //   block [0x826B14D8..0x826B1544)
	// 826B14D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B14DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B14E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B14E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B14E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B14EC: 38EB6F08  addi r7, r11, 0x6f08
	ctx.r[7].s64 = ctx.r[11].s64 + 28424;
	// 826B14F0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B14F4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826B14F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B14FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1508: 386ACF54  addi r3, r10, -0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12460;
	// 826B150C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B151C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B152C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1530: 4BDB58F1  bl 0x82466e20
	ctx.lr = 0x826B1534;
	sub_82466E20(ctx, base);
	// 826B1534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B153C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B1548 size=24
    let mut pc: u32 = 0x826B1548;
    'dispatch: loop {
        match pc {
            0x826B1548 => {
    //   block [0x826B1548..0x826B1560)
	// 826B1548: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B154C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B1550: 394ABC40  addi r10, r10, -0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + -17344;
	// 826B1554: 816B6F98  lwz r11, 0x6f98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28568 as u32) ) } as u64;
	// 826B1558: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826B155C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1560 size=108
    let mut pc: u32 = 0x826B1560;
    'dispatch: loop {
        match pc {
            0x826B1560 => {
    //   block [0x826B1560..0x826B15CC)
	// 826B1560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B156C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B1570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1574: 38EBBC40  addi r7, r11, -0x43c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17344;
	// 826B1578: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B157C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826B1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1584: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1590: 386ACF84  addi r3, r10, -0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + -12412;
	// 826B1594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B159C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B15A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B15A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B15A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B15AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B15B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B15B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B15B8: 4BDB5869  bl 0x82466e20
	ctx.lr = 0x826B15BC;
	sub_82466E20(ctx, base);
	// 826B15BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B15C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B15C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B15C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B15D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B15D0 size=24
    let mut pc: u32 = 0x826B15D0;
    'dispatch: loop {
        match pc {
            0x826B15D0 => {
    //   block [0x826B15D0..0x826B15E8)
	// 826B15D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B15D4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B15D8: 394ABC70  addi r10, r10, -0x4390
	ctx.r[10].s64 = ctx.r[10].s64 + -17296;
	// 826B15DC: 816B6F98  lwz r11, 0x6f98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28568 as u32) ) } as u64;
	// 826B15E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826B15E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B15E8 size=108
    let mut pc: u32 = 0x826B15E8;
    'dispatch: loop {
        match pc {
            0x826B15E8 => {
    //   block [0x826B15E8..0x826B1654)
	// 826B15E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B15EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B15F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B15F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B15F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B15FC: 38EBBC70  addi r7, r11, -0x4390
	ctx.r[7].s64 = ctx.r[11].s64 + -17296;
	// 826B1600: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1604: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826B1608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B160C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1618: 386ACFB4  addi r3, r10, -0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + -12364;
	// 826B161C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B162C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B163C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1640: 4BDB57E1  bl 0x82466e20
	ctx.lr = 0x826B1644;
	sub_82466E20(ctx, base);
	// 826B1644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B164C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1658 size=108
    let mut pc: u32 = 0x826B1658;
    'dispatch: loop {
        match pc {
            0x826B1658 => {
    //   block [0x826B1658..0x826B16C4)
	// 826B1658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1664: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B166C: 38EB6F80  addi r7, r11, 0x6f80
	ctx.r[7].s64 = ctx.r[11].s64 + 28544;
	// 826B1670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B1674: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826B1678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B167C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1688: 386ACFE4  addi r3, r10, -0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + -12316;
	// 826B168C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B169C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B16A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B16A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B16A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B16AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B16B0: 4BDB5771  bl 0x82466e20
	ctx.lr = 0x826B16B4;
	sub_82466E20(ctx, base);
	// 826B16B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B16B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B16BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B16C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B16C8 size=24
    let mut pc: u32 = 0x826B16C8;
    'dispatch: loop {
        match pc {
            0x826B16C8 => {
    //   block [0x826B16C8..0x826B16E0)
	// 826B16C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B16CC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B16D0: 394ABCA0  addi r10, r10, -0x4360
	ctx.r[10].s64 = ctx.r[10].s64 + -17248;
	// 826B16D4: 816B6F98  lwz r11, 0x6f98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28568 as u32) ) } as u64;
	// 826B16D8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826B16DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B16E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B16E0 size=108
    let mut pc: u32 = 0x826B16E0;
    'dispatch: loop {
        match pc {
            0x826B16E0 => {
    //   block [0x826B16E0..0x826B174C)
	// 826B16E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B16E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B16E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B16EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B16F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B16F4: 38EBBCA0  addi r7, r11, -0x4360
	ctx.r[7].s64 = ctx.r[11].s64 + -17248;
	// 826B16F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B16FC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826B1700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B170C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1710: 386AD014  addi r3, r10, -0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + -12268;
	// 826B1714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B171C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B172C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1738: 4BDB56E9  bl 0x82466e20
	ctx.lr = 0x826B173C;
	sub_82466E20(ctx, base);
	// 826B173C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1750 size=112
    let mut pc: u32 = 0x826B1750;
    'dispatch: loop {
        match pc {
            0x826B1750 => {
    //   block [0x826B1750..0x826B17C0)
	// 826B1750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B175C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1760: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1764: 392AF2C4  addi r9, r10, -0xd3c
	ctx.r[9].s64 = ctx.r[10].s64 + -3388;
	// 826B1768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B176C: 390B6F9C  addi r8, r11, 0x6f9c
	ctx.r[8].s64 = ctx.r[11].s64 + 28572;
	// 826B1770: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B1774: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826B1778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B177C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1788: 386AD044  addi r3, r10, -0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -12220;
	// 826B178C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1790: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B1794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B179C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B17A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B17A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B17A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B17AC: 4BDB5675  bl 0x82466e20
	ctx.lr = 0x826B17B0;
	sub_82466E20(ctx, base);
	// 826B17B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B17B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B17B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B17BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B17C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B17C0 size=108
    let mut pc: u32 = 0x826B17C0;
    'dispatch: loop {
        match pc {
            0x826B17C0 => {
    //   block [0x826B17C0..0x826B182C)
	// 826B17C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B17C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B17C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B17CC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B17D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B17D4: 38EB6FCC  addi r7, r11, 0x6fcc
	ctx.r[7].s64 = ctx.r[11].s64 + 28620;
	// 826B17D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B17DC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826B17E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B17E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B17E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B17EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B17F0: 386AD074  addi r3, r10, -0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -12172;
	// 826B17F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B17F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B17FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B180C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1818: 4BDB5609  bl 0x82466e20
	ctx.lr = 0x826B181C;
	sub_82466E20(ctx, base);
	// 826B181C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1830 size=108
    let mut pc: u32 = 0x826B1830;
    'dispatch: loop {
        match pc {
            0x826B1830 => {
    //   block [0x826B1830..0x826B189C)
	// 826B1830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B183C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1844: 38EB6FFC  addi r7, r11, 0x6ffc
	ctx.r[7].s64 = ctx.r[11].s64 + 28668;
	// 826B1848: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B184C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826B1850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1858: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B185C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1860: 386AD0A4  addi r3, r10, -0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -12124;
	// 826B1864: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B186C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B187C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1888: 4BDB5599  bl 0x82466e20
	ctx.lr = 0x826B188C;
	sub_82466E20(ctx, base);
	// 826B188C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B18A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B18A0 size=112
    let mut pc: u32 = 0x826B18A0;
    'dispatch: loop {
        match pc {
            0x826B18A0 => {
    //   block [0x826B18A0..0x826B1910)
	// 826B18A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B18A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B18A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B18AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B18B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B18B4: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 826B18B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B18BC: 390B702C  addi r8, r11, 0x702c
	ctx.r[8].s64 = ctx.r[11].s64 + 28716;
	// 826B18C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B18C4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826B18C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B18CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B18D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B18D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B18D8: 386AD0D4  addi r3, r10, -0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -12076;
	// 826B18DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B18E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B18E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B18E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B18EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B18F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B18F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B18F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B18FC: 4BDB5525  bl 0x82466e20
	ctx.lr = 0x826B1900;
	sub_82466E20(ctx, base);
	// 826B1900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B190C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1910 size=108
    let mut pc: u32 = 0x826B1910;
    'dispatch: loop {
        match pc {
            0x826B1910 => {
    //   block [0x826B1910..0x826B197C)
	// 826B1910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B191C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1924: 38EB7044  addi r7, r11, 0x7044
	ctx.r[7].s64 = ctx.r[11].s64 + 28740;
	// 826B1928: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B192C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826B1930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B193C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1940: 386AD104  addi r3, r10, -0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + -12028;
	// 826B1944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B194C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B195C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1968: 4BDB54B9  bl 0x82466e20
	ctx.lr = 0x826B196C;
	sub_82466E20(ctx, base);
	// 826B196C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1980 size=108
    let mut pc: u32 = 0x826B1980;
    'dispatch: loop {
        match pc {
            0x826B1980 => {
    //   block [0x826B1980..0x826B19EC)
	// 826B1980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B198C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1994: 38EB7074  addi r7, r11, 0x7074
	ctx.r[7].s64 = ctx.r[11].s64 + 28788;
	// 826B1998: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B199C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826B19A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B19A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B19A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B19AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B19B0: 386AD134  addi r3, r10, -0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -11980;
	// 826B19B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B19B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B19BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B19C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B19C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B19C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B19CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B19D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B19D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B19D8: 4BDB5449  bl 0x82466e20
	ctx.lr = 0x826B19DC;
	sub_82466E20(ctx, base);
	// 826B19DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B19E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B19E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B19E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B19F0 size=108
    let mut pc: u32 = 0x826B19F0;
    'dispatch: loop {
        match pc {
            0x826B19F0 => {
    //   block [0x826B19F0..0x826B1A5C)
	// 826B19F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B19F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B19F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B19FC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1A04: 38EB708C  addi r7, r11, 0x708c
	ctx.r[7].s64 = ctx.r[11].s64 + 28812;
	// 826B1A08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1A0C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826B1A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1A14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1A18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1A20: 386AD164  addi r3, r10, -0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11932;
	// 826B1A24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1A48: 4BDB53D9  bl 0x82466e20
	ctx.lr = 0x826B1A4C;
	sub_82466E20(ctx, base);
	// 826B1A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1A60 size=108
    let mut pc: u32 = 0x826B1A60;
    'dispatch: loop {
        match pc {
            0x826B1A60 => {
    //   block [0x826B1A60..0x826B1ACC)
	// 826B1A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1A6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1A74: 38EB70C0  addi r7, r11, 0x70c0
	ctx.r[7].s64 = ctx.r[11].s64 + 28864;
	// 826B1A78: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826B1A7C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826B1A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1A88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1A90: 386AD194  addi r3, r10, -0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11884;
	// 826B1A94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1AB8: 4BDB5369  bl 0x82466e20
	ctx.lr = 0x826B1ABC;
	sub_82466E20(ctx, base);
	// 826B1ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1AD0 size=108
    let mut pc: u32 = 0x826B1AD0;
    'dispatch: loop {
        match pc {
            0x826B1AD0 => {
    //   block [0x826B1AD0..0x826B1B3C)
	// 826B1AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1ADC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1AE4: 38EB7168  addi r7, r11, 0x7168
	ctx.r[7].s64 = ctx.r[11].s64 + 29032;
	// 826B1AE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1AEC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826B1AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1AF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1AF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1B00: 386AD1C4  addi r3, r10, -0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11836;
	// 826B1B04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1B28: 4BDB52F9  bl 0x82466e20
	ctx.lr = 0x826B1B2C;
	sub_82466E20(ctx, base);
	// 826B1B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1B40 size=108
    let mut pc: u32 = 0x826B1B40;
    'dispatch: loop {
        match pc {
            0x826B1B40 => {
    //   block [0x826B1B40..0x826B1BAC)
	// 826B1B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1B4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1B54: 38EB7198  addi r7, r11, 0x7198
	ctx.r[7].s64 = ctx.r[11].s64 + 29080;
	// 826B1B58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B1B5C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826B1B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1B64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1B68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1B70: 386AD1F4  addi r3, r10, -0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11788;
	// 826B1B74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1B94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1B98: 4BDB5289  bl 0x82466e20
	ctx.lr = 0x826B1B9C;
	sub_82466E20(ctx, base);
	// 826B1B9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1BB0 size=108
    let mut pc: u32 = 0x826B1BB0;
    'dispatch: loop {
        match pc {
            0x826B1BB0 => {
    //   block [0x826B1BB0..0x826B1C1C)
	// 826B1BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1BBC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1BC4: 38EB71B0  addi r7, r11, 0x71b0
	ctx.r[7].s64 = ctx.r[11].s64 + 29104;
	// 826B1BC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1BCC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826B1BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1BD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1BD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1BE0: 386AD224  addi r3, r10, -0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -11740;
	// 826B1BE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1C04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1C08: 4BDB5219  bl 0x82466e20
	ctx.lr = 0x826B1C0C;
	sub_82466E20(ctx, base);
	// 826B1C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1C20 size=108
    let mut pc: u32 = 0x826B1C20;
    'dispatch: loop {
        match pc {
            0x826B1C20 => {
    //   block [0x826B1C20..0x826B1C8C)
	// 826B1C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1C2C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1C34: 38EB71E0  addi r7, r11, 0x71e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29152;
	// 826B1C38: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826B1C3C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826B1C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1C44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1C48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1C50: 386AD254  addi r3, r10, -0x2dac
	ctx.r[3].s64 = ctx.r[10].s64 + -11692;
	// 826B1C54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1C74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1C78: 4BDB51A9  bl 0x82466e20
	ctx.lr = 0x826B1C7C;
	sub_82466E20(ctx, base);
	// 826B1C7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B1C90 size=24
    let mut pc: u32 = 0x826B1C90;
    'dispatch: loop {
        match pc {
            0x826B1C90 => {
    //   block [0x826B1C90..0x826B1CA8)
	// 826B1C90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1C94: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B1C98: 394ABCD0  addi r10, r10, -0x4330
	ctx.r[10].s64 = ctx.r[10].s64 + -17200;
	// 826B1C9C: 816B70BC  lwz r11, 0x70bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28860 as u32) ) } as u64;
	// 826B1CA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B1CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1CA8 size=112
    let mut pc: u32 = 0x826B1CA8;
    'dispatch: loop {
        match pc {
            0x826B1CA8 => {
    //   block [0x826B1CA8..0x826B1D18)
	// 826B1CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1CB4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1CB8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B1CBC: 392AF2F0  addi r9, r10, -0xd10
	ctx.r[9].s64 = ctx.r[10].s64 + -3344;
	// 826B1CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1CC4: 390BBCD0  addi r8, r11, -0x4330
	ctx.r[8].s64 = ctx.r[11].s64 + -17200;
	// 826B1CC8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B1CCC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826B1CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1CE0: 386AD284  addi r3, r10, -0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11644;
	// 826B1CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1CE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B1CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1D04: 4BDB511D  bl 0x82466e20
	ctx.lr = 0x826B1D08;
	sub_82466E20(ctx, base);
	// 826B1D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1D18 size=108
    let mut pc: u32 = 0x826B1D18;
    'dispatch: loop {
        match pc {
            0x826B1D18 => {
    //   block [0x826B1D18..0x826B1D84)
	// 826B1D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1D24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1D2C: 38EB72A4  addi r7, r11, 0x72a4
	ctx.r[7].s64 = ctx.r[11].s64 + 29348;
	// 826B1D30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1D34: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826B1D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1D48: 386AD2B4  addi r3, r10, -0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11596;
	// 826B1D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1D70: 4BDB50B1  bl 0x82466e20
	ctx.lr = 0x826B1D74;
	sub_82466E20(ctx, base);
	// 826B1D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1D88 size=112
    let mut pc: u32 = 0x826B1D88;
    'dispatch: loop {
        match pc {
            0x826B1D88 => {
    //   block [0x826B1D88..0x826B1DF8)
	// 826B1D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1D94: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1D98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1D9C: 392AF334  addi r9, r10, -0xccc
	ctx.r[9].s64 = ctx.r[10].s64 + -3276;
	// 826B1DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1DA4: 390B72D8  addi r8, r11, 0x72d8
	ctx.r[8].s64 = ctx.r[11].s64 + 29400;
	// 826B1DA8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826B1DAC: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826B1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1DB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1DC0: 386AD2E4  addi r3, r10, -0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11548;
	// 826B1DC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1DC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B1DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1DE4: 4BDB503D  bl 0x82466e20
	ctx.lr = 0x826B1DE8;
	sub_82466E20(ctx, base);
	// 826B1DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B1DF8 size=24
    let mut pc: u32 = 0x826B1DF8;
    'dispatch: loop {
        match pc {
            0x826B1DF8 => {
    //   block [0x826B1DF8..0x826B1E10)
	// 826B1DF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1DFC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B1E00: 394ABD48  addi r10, r10, -0x42b8
	ctx.r[10].s64 = ctx.r[10].s64 + -17080;
	// 826B1E04: 816B72D4  lwz r11, 0x72d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29396 as u32) ) } as u64;
	// 826B1E08: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826B1E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1E10 size=112
    let mut pc: u32 = 0x826B1E10;
    'dispatch: loop {
        match pc {
            0x826B1E10 => {
    //   block [0x826B1E10..0x826B1E80)
	// 826B1E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1E1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1E20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B1E24: 392AF370  addi r9, r10, -0xc90
	ctx.r[9].s64 = ctx.r[10].s64 + -3216;
	// 826B1E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1E2C: 390BBD48  addi r8, r11, -0x42b8
	ctx.r[8].s64 = ctx.r[11].s64 + -17080;
	// 826B1E30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B1E34: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826B1E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1E3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1E48: 386AD314  addi r3, r10, -0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + -11500;
	// 826B1E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1E50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B1E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1E6C: 4BDB4FB5  bl 0x82466e20
	ctx.lr = 0x826B1E70;
	sub_82466E20(ctx, base);
	// 826B1E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1E80 size=108
    let mut pc: u32 = 0x826B1E80;
    'dispatch: loop {
        match pc {
            0x826B1E80 => {
    //   block [0x826B1E80..0x826B1EEC)
	// 826B1E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1E8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1E94: 38EB7398  addi r7, r11, 0x7398
	ctx.r[7].s64 = ctx.r[11].s64 + 29592;
	// 826B1E98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B1E9C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826B1EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1EA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1EB0: 386AD344  addi r3, r10, -0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11452;
	// 826B1EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1ED8: 4BDB4F49  bl 0x82466e20
	ctx.lr = 0x826B1EDC;
	sub_82466E20(ctx, base);
	// 826B1EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1EF0 size=108
    let mut pc: u32 = 0x826B1EF0;
    'dispatch: loop {
        match pc {
            0x826B1EF0 => {
    //   block [0x826B1EF0..0x826B1F5C)
	// 826B1EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1EFC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1F04: 38EB73B0  addi r7, r11, 0x73b0
	ctx.r[7].s64 = ctx.r[11].s64 + 29616;
	// 826B1F08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B1F0C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826B1F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1F14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B1F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B1F20: 386AD374  addi r3, r10, -0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11404;
	// 826B1F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B1F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B1F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1F48: 4BDB4ED9  bl 0x82466e20
	ctx.lr = 0x826B1F4C;
	sub_82466E20(ctx, base);
	// 826B1F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B1F60 size=24
    let mut pc: u32 = 0x826B1F60;
    'dispatch: loop {
        match pc {
            0x826B1F60 => {
    //   block [0x826B1F60..0x826B1F78)
	// 826B1F60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1F64: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B1F68: 394ABD90  addi r10, r10, -0x4270
	ctx.r[10].s64 = ctx.r[10].s64 + -17008;
	// 826B1F6C: 816B73E0  lwz r11, 0x73e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29664 as u32) ) } as u64;
	// 826B1F70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B1F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1F78 size=112
    let mut pc: u32 = 0x826B1F78;
    'dispatch: loop {
        match pc {
            0x826B1F78 => {
    //   block [0x826B1F78..0x826B1FE8)
	// 826B1F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1F84: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B1F88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B1F8C: 392AF3AC  addi r9, r10, -0xc54
	ctx.r[9].s64 = ctx.r[10].s64 + -3156;
	// 826B1F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1F94: 390BBD90  addi r8, r11, -0x4270
	ctx.r[8].s64 = ctx.r[11].s64 + -17008;
	// 826B1F98: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B1F9C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826B1FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B1FA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B1FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B1FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B1FB0: 386AD3A4  addi r3, r10, -0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11356;
	// 826B1FB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B1FB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B1FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B1FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B1FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B1FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B1FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B1FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B1FD4: 4BDB4E4D  bl 0x82466e20
	ctx.lr = 0x826B1FD8;
	sub_82466E20(ctx, base);
	// 826B1FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B1FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B1FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B1FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B1FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B1FE8 size=108
    let mut pc: u32 = 0x826B1FE8;
    'dispatch: loop {
        match pc {
            0x826B1FE8 => {
    //   block [0x826B1FE8..0x826B2054)
	// 826B1FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B1FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B1FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B1FF4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B1FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B1FFC: 38EB73E4  addi r7, r11, 0x73e4
	ctx.r[7].s64 = ctx.r[11].s64 + 29668;
	// 826B2000: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2004: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826B2008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B200C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2018: 386AD3D4  addi r3, r10, -0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	// 826B201C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B203C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2040: 4BDB4DE1  bl 0x82466e20
	ctx.lr = 0x826B2044;
	sub_82466E20(ctx, base);
	// 826B2044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B204C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2058 size=108
    let mut pc: u32 = 0x826B2058;
    'dispatch: loop {
        match pc {
            0x826B2058 => {
    //   block [0x826B2058..0x826B20C4)
	// 826B2058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2064: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B206C: 38EB7400  addi r7, r11, 0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + 29696;
	// 826B2070: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B2074: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826B2078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B207C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2088: 386AD404  addi r3, r10, -0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11260;
	// 826B208C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B209C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B20A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B20A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B20A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B20AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B20B0: 4BDB4D71  bl 0x82466e20
	ctx.lr = 0x826B20B4;
	sub_82466E20(ctx, base);
	// 826B20B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B20B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B20BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B20C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B20C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B20C8 size=108
    let mut pc: u32 = 0x826B20C8;
    'dispatch: loop {
        match pc {
            0x826B20C8 => {
    //   block [0x826B20C8..0x826B2134)
	// 826B20C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B20CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B20D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B20D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B20D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B20DC: 38EB7448  addi r7, r11, 0x7448
	ctx.r[7].s64 = ctx.r[11].s64 + 29768;
	// 826B20E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B20E4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826B20E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B20EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B20F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B20F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B20F8: 386AD434  addi r3, r10, -0x2bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -11212;
	// 826B20FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B210C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B211C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2120: 4BDB4D01  bl 0x82466e20
	ctx.lr = 0x826B2124;
	sub_82466E20(ctx, base);
	// 826B2124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B212C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2138 size=108
    let mut pc: u32 = 0x826B2138;
    'dispatch: loop {
        match pc {
            0x826B2138 => {
    //   block [0x826B2138..0x826B21A4)
	// 826B2138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2144: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B214C: 38EB7478  addi r7, r11, 0x7478
	ctx.r[7].s64 = ctx.r[11].s64 + 29816;
	// 826B2150: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826B2154: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826B2158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B215C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2168: 386AD464  addi r3, r10, -0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11164;
	// 826B216C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B217C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B218C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2190: 4BDB4C91  bl 0x82466e20
	ctx.lr = 0x826B2194;
	sub_82466E20(ctx, base);
	// 826B2194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B219C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B21A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B21A8 size=108
    let mut pc: u32 = 0x826B21A8;
    'dispatch: loop {
        match pc {
            0x826B21A8 => {
    //   block [0x826B21A8..0x826B2214)
	// 826B21A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B21AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B21B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B21B4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B21B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B21BC: 38EB7598  addi r7, r11, 0x7598
	ctx.r[7].s64 = ctx.r[11].s64 + 30104;
	// 826B21C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B21C4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826B21C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B21CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B21D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B21D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B21D8: 386AD494  addi r3, r10, -0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11116;
	// 826B21DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B21E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B21E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B21E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B21EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B21F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B21F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B21F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B21FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2200: 4BDB4C21  bl 0x82466e20
	ctx.lr = 0x826B2204;
	sub_82466E20(ctx, base);
	// 826B2204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2218 size=108
    let mut pc: u32 = 0x826B2218;
    'dispatch: loop {
        match pc {
            0x826B2218 => {
    //   block [0x826B2218..0x826B2284)
	// 826B2218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2224: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B222C: 38EB7628  addi r7, r11, 0x7628
	ctx.r[7].s64 = ctx.r[11].s64 + 30248;
	// 826B2230: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826B2234: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826B2238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B223C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2248: 386AD4C4  addi r3, r10, -0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11068;
	// 826B224C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B225C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B226C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2270: 4BDB4BB1  bl 0x82466e20
	ctx.lr = 0x826B2274;
	sub_82466E20(ctx, base);
	// 826B2274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B227C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2288 size=108
    let mut pc: u32 = 0x826B2288;
    'dispatch: loop {
        match pc {
            0x826B2288 => {
    //   block [0x826B2288..0x826B22F4)
	// 826B2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2294: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B229C: 38EB76E8  addi r7, r11, 0x76e8
	ctx.r[7].s64 = ctx.r[11].s64 + 30440;
	// 826B22A0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826B22A4: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826B22A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B22AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B22B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B22B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B22B8: 386AD4F4  addi r3, r10, -0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11020;
	// 826B22BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B22C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B22C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B22C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B22CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B22D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B22D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B22D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B22DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B22E0: 4BDB4B41  bl 0x82466e20
	ctx.lr = 0x826B22E4;
	sub_82466E20(ctx, base);
	// 826B22E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B22E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B22EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B22F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B22F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B22F8 size=108
    let mut pc: u32 = 0x826B22F8;
    'dispatch: loop {
        match pc {
            0x826B22F8 => {
    //   block [0x826B22F8..0x826B2364)
	// 826B22F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B22FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2304: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B230C: 38EB77C0  addi r7, r11, 0x77c0
	ctx.r[7].s64 = ctx.r[11].s64 + 30656;
	// 826B2310: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826B2314: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826B2318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B231C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2328: 386AD524  addi r3, r10, -0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + -10972;
	// 826B232C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B233C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B234C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2350: 4BDB4AD1  bl 0x82466e20
	ctx.lr = 0x826B2354;
	sub_82466E20(ctx, base);
	// 826B2354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B235C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2368 size=108
    let mut pc: u32 = 0x826B2368;
    'dispatch: loop {
        match pc {
            0x826B2368 => {
    //   block [0x826B2368..0x826B23D4)
	// 826B2368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2374: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B237C: 38EB7880  addi r7, r11, 0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + 30848;
	// 826B2380: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826B2384: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826B2388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B238C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2398: 386AD554  addi r3, r10, -0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + -10924;
	// 826B239C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B23A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B23A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B23A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B23AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B23B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B23B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B23B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B23BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B23C0: 4BDB4A61  bl 0x82466e20
	ctx.lr = 0x826B23C4;
	sub_82466E20(ctx, base);
	// 826B23C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B23C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B23CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B23D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B23D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B23D8 size=112
    let mut pc: u32 = 0x826B23D8;
    'dispatch: loop {
        match pc {
            0x826B23D8 => {
    //   block [0x826B23D8..0x826B2448)
	// 826B23D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B23DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B23E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B23E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B23E8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826B23EC: 38EA7928  addi r7, r10, 0x7928
	ctx.r[7].s64 = ctx.r[10].s64 + 31016;
	// 826B23F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B23F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B23F8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826B23FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2400: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2404: 396BF3C0  addi r11, r11, -0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + -3136;
	// 826B2408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B240C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2410: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2414: 386AD584  addi r3, r10, -0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -10876;
	// 826B2418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B241C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B2420: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2424: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B2428: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B242C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2430: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2434: 4BDB49ED  bl 0x82466e20
	ctx.lr = 0x826B2438;
	sub_82466E20(ctx, base);
	// 826B2438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2448 size=108
    let mut pc: u32 = 0x826B2448;
    'dispatch: loop {
        match pc {
            0x826B2448 => {
    //   block [0x826B2448..0x826B24B4)
	// 826B2448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2454: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B245C: 38EB7A48  addi r7, r11, 0x7a48
	ctx.r[7].s64 = ctx.r[11].s64 + 31304;
	// 826B2460: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B2464: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826B2468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B246C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2478: 386AD5B4  addi r3, r10, -0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -10828;
	// 826B247C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B248C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B249C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B24A0: 4BDB4981  bl 0x82466e20
	ctx.lr = 0x826B24A4;
	sub_82466E20(ctx, base);
	// 826B24A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B24A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B24AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B24B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B24B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B24B8 size=108
    let mut pc: u32 = 0x826B24B8;
    'dispatch: loop {
        match pc {
            0x826B24B8 => {
    //   block [0x826B24B8..0x826B2524)
	// 826B24B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B24BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B24C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B24C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B24C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B24CC: 38EB7AA8  addi r7, r11, 0x7aa8
	ctx.r[7].s64 = ctx.r[11].s64 + 31400;
	// 826B24D0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826B24D4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826B24D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B24DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B24E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B24E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B24E8: 386AD5E4  addi r3, r10, -0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -10780;
	// 826B24EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B24F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B24F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B24F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B24FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B250C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2510: 4BDB4911  bl 0x82466e20
	ctx.lr = 0x826B2514;
	sub_82466E20(ctx, base);
	// 826B2514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B251C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2528 size=108
    let mut pc: u32 = 0x826B2528;
    'dispatch: loop {
        match pc {
            0x826B2528 => {
    //   block [0x826B2528..0x826B2594)
	// 826B2528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2534: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B253C: 38EB7BB0  addi r7, r11, 0x7bb0
	ctx.r[7].s64 = ctx.r[11].s64 + 31664;
	// 826B2540: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826B2544: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826B2548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B254C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2558: 386AD614  addi r3, r10, -0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10732;
	// 826B255C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B256C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B257C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2580: 4BDB48A1  bl 0x82466e20
	ctx.lr = 0x826B2584;
	sub_82466E20(ctx, base);
	// 826B2584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B258C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2598 size=108
    let mut pc: u32 = 0x826B2598;
    'dispatch: loop {
        match pc {
            0x826B2598 => {
    //   block [0x826B2598..0x826B2604)
	// 826B2598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B25A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B25A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B25A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B25AC: 38EB7C88  addi r7, r11, 0x7c88
	ctx.r[7].s64 = ctx.r[11].s64 + 31880;
	// 826B25B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B25B4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826B25B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B25BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B25C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B25C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B25C8: 386AD644  addi r3, r10, -0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + -10684;
	// 826B25CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B25D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B25D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B25D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B25DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B25E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B25E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B25E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B25EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B25F0: 4BDB4831  bl 0x82466e20
	ctx.lr = 0x826B25F4;
	sub_82466E20(ctx, base);
	// 826B25F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B25F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B25FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2608 size=108
    let mut pc: u32 = 0x826B2608;
    'dispatch: loop {
        match pc {
            0x826B2608 => {
    //   block [0x826B2608..0x826B2674)
	// 826B2608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2614: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B261C: 38EB7CD0  addi r7, r11, 0x7cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 31952;
	// 826B2620: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2624: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826B2628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B262C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2638: 386AD674  addi r3, r10, -0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + -10636;
	// 826B263C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B265C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2660: 4BDB47C1  bl 0x82466e20
	ctx.lr = 0x826B2664;
	sub_82466E20(ctx, base);
	// 826B2664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B266C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2678 size=108
    let mut pc: u32 = 0x826B2678;
    'dispatch: loop {
        match pc {
            0x826B2678 => {
    //   block [0x826B2678..0x826B26E4)
	// 826B2678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B267C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2684: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B268C: 38EB7CE8  addi r7, r11, 0x7ce8
	ctx.r[7].s64 = ctx.r[11].s64 + 31976;
	// 826B2690: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B2694: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826B2698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B269C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B26A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B26A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B26A8: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 826B26AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B26B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B26B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B26B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B26BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B26C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B26C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B26C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B26CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B26D0: 4BDB4751  bl 0x82466e20
	ctx.lr = 0x826B26D4;
	sub_82466E20(ctx, base);
	// 826B26D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B26D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B26DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B26E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B26E8 size=108
    let mut pc: u32 = 0x826B26E8;
    'dispatch: loop {
        match pc {
            0x826B26E8 => {
    //   block [0x826B26E8..0x826B2754)
	// 826B26E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B26EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B26F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B26F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B26F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B26FC: 38EB7D30  addi r7, r11, 0x7d30
	ctx.r[7].s64 = ctx.r[11].s64 + 32048;
	// 826B2700: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2704: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826B2708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B270C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2718: 386AD6D4  addi r3, r10, -0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + -10540;
	// 826B271C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B272C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B273C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2740: 4BDB46E1  bl 0x82466e20
	ctx.lr = 0x826B2744;
	sub_82466E20(ctx, base);
	// 826B2744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B274C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2758 size=112
    let mut pc: u32 = 0x826B2758;
    'dispatch: loop {
        match pc {
            0x826B2758 => {
    //   block [0x826B2758..0x826B27C8)
	// 826B2758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B275C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2768: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B276C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B2770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2774: 390B7D48  addi r8, r11, 0x7d48
	ctx.r[8].s64 = ctx.r[11].s64 + 32072;
	// 826B2778: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B277C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826B2780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2790: 386AD704  addi r3, r10, -0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + -10492;
	// 826B2794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B279C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B27A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B27A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B27A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B27AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B27B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B27B4: 4BDB466D  bl 0x82466e20
	ctx.lr = 0x826B27B8;
	sub_82466E20(ctx, base);
	// 826B27B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B27BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B27C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B27C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B27C8 size=108
    let mut pc: u32 = 0x826B27C8;
    'dispatch: loop {
        match pc {
            0x826B27C8 => {
    //   block [0x826B27C8..0x826B2834)
	// 826B27C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B27CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B27D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B27D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B27D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B27DC: 38EB7D90  addi r7, r11, 0x7d90
	ctx.r[7].s64 = ctx.r[11].s64 + 32144;
	// 826B27E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B27E4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826B27E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B27EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B27F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B27F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B27F8: 386AD734  addi r3, r10, -0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10444;
	// 826B27FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B280C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B281C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2820: 4BDB4601  bl 0x82466e20
	ctx.lr = 0x826B2824;
	sub_82466E20(ctx, base);
	// 826B2824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B282C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2838 size=112
    let mut pc: u32 = 0x826B2838;
    'dispatch: loop {
        match pc {
            0x826B2838 => {
    //   block [0x826B2838..0x826B28A8)
	// 826B2838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B283C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2848: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B284C: 38AAD734  addi r5, r10, -0x28cc
	ctx.r[5].s64 = ctx.r[10].s64 + -10444;
	// 826B2850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2854: 390B7DF0  addi r8, r11, 0x7df0
	ctx.r[8].s64 = ctx.r[11].s64 + 32240;
	// 826B2858: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B285C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826B2860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B286C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2870: 386AD764  addi r3, r10, -0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + -10396;
	// 826B2874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B287C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B288C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2894: 4BDB458D  bl 0x82466e20
	ctx.lr = 0x826B2898;
	sub_82466E20(ctx, base);
	// 826B2898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B289C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B28A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B28A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B28A8 size=96
    let mut pc: u32 = 0x826B28A8;
    'dispatch: loop {
        match pc {
            0x826B28A8 => {
    //   block [0x826B28A8..0x826B2908)
	// 826B28A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B28AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B28B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B28B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B28B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B28BC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826B28C0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B28C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B28C8: 386AD794  addi r3, r10, -0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + -10348;
	// 826B28CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B28D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B28D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B28D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B28DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B28E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B28E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B28E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B28EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B28F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B28F4: 4BDB452D  bl 0x82466e20
	ctx.lr = 0x826B28F8;
	sub_82466E20(ctx, base);
	// 826B28F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B28FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2908 size=112
    let mut pc: u32 = 0x826B2908;
    'dispatch: loop {
        match pc {
            0x826B2908 => {
    //   block [0x826B2908..0x826B2978)
	// 826B2908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2918: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B291C: 38AAF474  addi r5, r10, -0xb8c
	ctx.r[5].s64 = ctx.r[10].s64 + -2956;
	// 826B2920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2924: 390B7E38  addi r8, r11, 0x7e38
	ctx.r[8].s64 = ctx.r[11].s64 + 32312;
	// 826B2928: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B292C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826B2930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B293C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2940: 386AD7C4  addi r3, r10, -0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + -10300;
	// 826B2944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B294C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B295C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2964: 4BDB44BD  bl 0x82466e20
	ctx.lr = 0x826B2968;
	sub_82466E20(ctx, base);
	// 826B2968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B296C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2978 size=96
    let mut pc: u32 = 0x826B2978;
    'dispatch: loop {
        match pc {
            0x826B2978 => {
    //   block [0x826B2978..0x826B29D8)
	// 826B2978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2984: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B298C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826B2990: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2998: 386AD7F4  addi r3, r10, -0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + -10252;
	// 826B299C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B29A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B29A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B29A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B29AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B29B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B29B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B29B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B29BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B29C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B29C4: 4BDB445D  bl 0x82466e20
	ctx.lr = 0x826B29C8;
	sub_82466E20(ctx, base);
	// 826B29C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B29CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B29D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B29D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B29D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B29D8 size=100
    let mut pc: u32 = 0x826B29D8;
    'dispatch: loop {
        match pc {
            0x826B29D8 => {
    //   block [0x826B29D8..0x826B2A3C)
	// 826B29D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B29DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B29E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B29E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B29E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B29EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B29F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B29F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B29F8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826B29FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2A04: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B2A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2A0C: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 826B2A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2A14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2A18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B2A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2A20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B2A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2A28: 4BDB43F9  bl 0x82466e20
	ctx.lr = 0x826B2A2C;
	sub_82466E20(ctx, base);
	// 826B2A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2A40 size=96
    let mut pc: u32 = 0x826B2A40;
    'dispatch: loop {
        match pc {
            0x826B2A40 => {
    //   block [0x826B2A40..0x826B2AA0)
	// 826B2A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2A4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2A54: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826B2A58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2A60: 386AD854  addi r3, r10, -0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10156;
	// 826B2A64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2A6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2A80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B2A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2A88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B2A8C: 4BDB4395  bl 0x82466e20
	ctx.lr = 0x826B2A90;
	sub_82466E20(ctx, base);
	// 826B2A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2AA0 size=112
    let mut pc: u32 = 0x826B2AA0;
    'dispatch: loop {
        match pc {
            0x826B2AA0 => {
    //   block [0x826B2AA0..0x826B2B10)
	// 826B2AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2AAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2AB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2AB4: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B2AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2ABC: 390B7E98  addi r8, r11, 0x7e98
	ctx.r[8].s64 = ctx.r[11].s64 + 32408;
	// 826B2AC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B2AC4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826B2AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2ACC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2AD8: 386AD884  addi r3, r10, -0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + -10108;
	// 826B2ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2AFC: 4BDB4325  bl 0x82466e20
	ctx.lr = 0x826B2B00;
	sub_82466E20(ctx, base);
	// 826B2B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2B10 size=112
    let mut pc: u32 = 0x826B2B10;
    'dispatch: loop {
        match pc {
            0x826B2B10 => {
    //   block [0x826B2B10..0x826B2B80)
	// 826B2B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2B1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2B20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2B24: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B2B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2B2C: 390B7EC8  addi r8, r11, 0x7ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 32456;
	// 826B2B30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2B34: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826B2B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2B3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2B40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2B48: 386AD8B4  addi r3, r10, -0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + -10060;
	// 826B2B4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2B6C: 4BDB42B5  bl 0x82466e20
	ctx.lr = 0x826B2B70;
	sub_82466E20(ctx, base);
	// 826B2B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2B80 size=100
    let mut pc: u32 = 0x826B2B80;
    'dispatch: loop {
        match pc {
            0x826B2B80 => {
    //   block [0x826B2B80..0x826B2BE4)
	// 826B2B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2B8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2B94: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B2B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2BA0: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826B2BA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2BB4: 386AD8E4  addi r3, r10, -0x271c
	ctx.r[3].s64 = ctx.r[10].s64 + -10012;
	// 826B2BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2BC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B2BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2BC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B2BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2BD0: 4BDB4251  bl 0x82466e20
	ctx.lr = 0x826B2BD4;
	sub_82466E20(ctx, base);
	// 826B2BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2BE8 size=96
    let mut pc: u32 = 0x826B2BE8;
    'dispatch: loop {
        match pc {
            0x826B2BE8 => {
    //   block [0x826B2BE8..0x826B2C48)
	// 826B2BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2BF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2BFC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826B2C00: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2C08: 386AD914  addi r3, r10, -0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9964;
	// 826B2C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2C14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2C28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B2C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B2C34: 4BDB41ED  bl 0x82466e20
	ctx.lr = 0x826B2C38;
	sub_82466E20(ctx, base);
	// 826B2C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2C48 size=112
    let mut pc: u32 = 0x826B2C48;
    'dispatch: loop {
        match pc {
            0x826B2C48 => {
    //   block [0x826B2C48..0x826B2CB8)
	// 826B2C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2C54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2C58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2C5C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B2C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2C64: 390B7EE0  addi r8, r11, 0x7ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 32480;
	// 826B2C68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2C6C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826B2C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2C74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2C80: 386AD944  addi r3, r10, -0x26bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9916;
	// 826B2C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2CA4: 4BDB417D  bl 0x82466e20
	ctx.lr = 0x826B2CA8;
	sub_82466E20(ctx, base);
	// 826B2CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2CB8 size=96
    let mut pc: u32 = 0x826B2CB8;
    'dispatch: loop {
        match pc {
            0x826B2CB8 => {
    //   block [0x826B2CB8..0x826B2D18)
	// 826B2CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2CC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2CCC: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826B2CD0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2CD8: 386AD974  addi r3, r10, -0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + -9868;
	// 826B2CDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2CE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B2CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2CF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B2CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2D00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B2D04: 4BDB411D  bl 0x82466e20
	ctx.lr = 0x826B2D08;
	sub_82466E20(ctx, base);
	// 826B2D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2D18 size=112
    let mut pc: u32 = 0x826B2D18;
    'dispatch: loop {
        match pc {
            0x826B2D18 => {
    //   block [0x826B2D18..0x826B2D88)
	// 826B2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2D24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2D28: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2D2C: 38AAD974  addi r5, r10, -0x268c
	ctx.r[5].s64 = ctx.r[10].s64 + -9868;
	// 826B2D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2D34: 390B7EF8  addi r8, r11, 0x7ef8
	ctx.r[8].s64 = ctx.r[11].s64 + 32504;
	// 826B2D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2D3C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826B2D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2D50: 386AD9A4  addi r3, r10, -0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + -9820;
	// 826B2D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2D74: 4BDB40AD  bl 0x82466e20
	ctx.lr = 0x826B2D78;
	sub_82466E20(ctx, base);
	// 826B2D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2D88 size=108
    let mut pc: u32 = 0x826B2D88;
    'dispatch: loop {
        match pc {
            0x826B2D88 => {
    //   block [0x826B2D88..0x826B2DF4)
	// 826B2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2D94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2D9C: 38EB7F10  addi r7, r11, 0x7f10
	ctx.r[7].s64 = ctx.r[11].s64 + 32528;
	// 826B2DA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B2DA4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826B2DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B2DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2DB8: 386AD9D4  addi r3, r10, -0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + -9772;
	// 826B2DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B2DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B2DE0: 4BDB4041  bl 0x82466e20
	ctx.lr = 0x826B2DE4;
	sub_82466E20(ctx, base);
	// 826B2DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2DF8 size=112
    let mut pc: u32 = 0x826B2DF8;
    'dispatch: loop {
        match pc {
            0x826B2DF8 => {
    //   block [0x826B2DF8..0x826B2E68)
	// 826B2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2E04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2E08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2E0C: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B2E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2E14: 390B7F70  addi r8, r11, 0x7f70
	ctx.r[8].s64 = ctx.r[11].s64 + 32624;
	// 826B2E18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2E1C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826B2E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2E24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2E28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2E30: 386ADA04  addi r3, r10, -0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9724;
	// 826B2E34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2E54: 4BDB3FCD  bl 0x82466e20
	ctx.lr = 0x826B2E58;
	sub_82466E20(ctx, base);
	// 826B2E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2E68 size=112
    let mut pc: u32 = 0x826B2E68;
    'dispatch: loop {
        match pc {
            0x826B2E68 => {
    //   block [0x826B2E68..0x826B2ED8)
	// 826B2E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2E74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2E78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2E7C: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B2E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2E84: 390B7F88  addi r8, r11, 0x7f88
	ctx.r[8].s64 = ctx.r[11].s64 + 32648;
	// 826B2E88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B2E8C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826B2E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2EA0: 386ADA34  addi r3, r10, -0x25cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9676;
	// 826B2EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2EC4: 4BDB3F5D  bl 0x82466e20
	ctx.lr = 0x826B2EC8;
	sub_82466E20(ctx, base);
	// 826B2EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2ED8 size=112
    let mut pc: u32 = 0x826B2ED8;
    'dispatch: loop {
        match pc {
            0x826B2ED8 => {
    //   block [0x826B2ED8..0x826B2F48)
	// 826B2ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2EE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2EEC: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B2EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2EF4: 390B7FB8  addi r8, r11, 0x7fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 32696;
	// 826B2EF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2EFC: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826B2F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2F10: 386ADA64  addi r3, r10, -0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + -9628;
	// 826B2F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B2F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B2F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B2F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2F34: 4BDB3EED  bl 0x82466e20
	ctx.lr = 0x826B2F38;
	sub_82466E20(ctx, base);
	// 826B2F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2F48 size=116
    let mut pc: u32 = 0x826B2F48;
    'dispatch: loop {
        match pc {
            0x826B2F48 => {
    //   block [0x826B2F48..0x826B2FBC)
	// 826B2F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2F54: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B2F58: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B2F5C: 390B7FD0  addi r8, r11, 0x7fd0
	ctx.r[8].s64 = ctx.r[11].s64 + 32720;
	// 826B2F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2F64: 392AF438  addi r9, r10, -0xbc8
	ctx.r[9].s64 = ctx.r[10].s64 + -3016;
	// 826B2F68: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2F6C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B2F70: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B2F74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B2F7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B2F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B2F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B2F8C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B2F90: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826B2F94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B2F98: 386BDA94  addi r3, r11, -0x256c
	ctx.r[3].s64 = ctx.r[11].s64 + -9580;
	// 826B2F9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B2FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B2FA8: 4BDB3E79  bl 0x82466e20
	ctx.lr = 0x826B2FAC;
	sub_82466E20(ctx, base);
	// 826B2FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B2FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B2FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B2FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B2FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B2FC0 size=112
    let mut pc: u32 = 0x826B2FC0;
    'dispatch: loop {
        match pc {
            0x826B2FC0 => {
    //   block [0x826B2FC0..0x826B3030)
	// 826B2FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B2FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B2FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B2FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2FD0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B2FD4: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B2FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B2FDC: 390B8000  addi r8, r11, -0x8000
	ctx.r[8].s64 = ctx.r[11].s64 + -32768;
	// 826B2FE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B2FE4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826B2FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B2FEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B2FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B2FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B2FF8: 386ADAC4  addi r3, r10, -0x253c
	ctx.r[3].s64 = ctx.r[10].s64 + -9532;
	// 826B2FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B300C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B3010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B301C: 4BDB3E05  bl 0x82466e20
	ctx.lr = 0x826B3020;
	sub_82466E20(ctx, base);
	// 826B3020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B302C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3030 size=112
    let mut pc: u32 = 0x826B3030;
    'dispatch: loop {
        match pc {
            0x826B3030 => {
    //   block [0x826B3030..0x826B30A0)
	// 826B3030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B303C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3040: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3044: 38AADFA4  addi r5, r10, -0x205c
	ctx.r[5].s64 = ctx.r[10].s64 + -8284;
	// 826B3048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B304C: 390B8018  addi r8, r11, -0x7fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -32744;
	// 826B3050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3054: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826B3058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B305C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3068: 386ADAF4  addi r3, r10, -0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	// 826B306C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B307C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B308C: 4BDB3D95  bl 0x82466e20
	ctx.lr = 0x826B3090;
	sub_82466E20(ctx, base);
	// 826B3090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B30A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B30A0 size=112
    let mut pc: u32 = 0x826B30A0;
    'dispatch: loop {
        match pc {
            0x826B30A0 => {
    //   block [0x826B30A0..0x826B3110)
	// 826B30A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B30A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B30A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B30AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B30B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B30B4: 38AADD04  addi r5, r10, -0x22fc
	ctx.r[5].s64 = ctx.r[10].s64 + -8956;
	// 826B30B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B30BC: 390B8030  addi r8, r11, -0x7fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -32720;
	// 826B30C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B30C4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826B30C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B30CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B30D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B30D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B30D8: 386ADB24  addi r3, r10, -0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9436;
	// 826B30DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B30E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B30E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B30E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B30EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B30F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B30F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B30F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B30FC: 4BDB3D25  bl 0x82466e20
	ctx.lr = 0x826B3100;
	sub_82466E20(ctx, base);
	// 826B3100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3110 size=112
    let mut pc: u32 = 0x826B3110;
    'dispatch: loop {
        match pc {
            0x826B3110 => {
    //   block [0x826B3110..0x826B3180)
	// 826B3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B311C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3120: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3124: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B312C: 390B8048  addi r8, r11, -0x7fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -32696;
	// 826B3130: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B3134: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826B3138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B313C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3148: 386ADB54  addi r3, r10, -0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9388;
	// 826B314C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B315C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B316C: 4BDB3CB5  bl 0x82466e20
	ctx.lr = 0x826B3170;
	sub_82466E20(ctx, base);
	// 826B3170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3180 size=112
    let mut pc: u32 = 0x826B3180;
    'dispatch: loop {
        match pc {
            0x826B3180 => {
    //   block [0x826B3180..0x826B31F0)
	// 826B3180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B318C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3190: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3194: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B319C: 390B8090  addi r8, r11, -0x7f70
	ctx.r[8].s64 = ctx.r[11].s64 + -32624;
	// 826B31A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B31A4: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826B31A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B31AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B31B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B31B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B31B8: 386ADB84  addi r3, r10, -0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + -9340;
	// 826B31BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B31C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B31C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B31C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B31CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B31D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B31D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B31D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B31DC: 4BDB3C45  bl 0x82466e20
	ctx.lr = 0x826B31E0;
	sub_82466E20(ctx, base);
	// 826B31E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B31E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B31E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B31EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B31F0 size=112
    let mut pc: u32 = 0x826B31F0;
    'dispatch: loop {
        match pc {
            0x826B31F0 => {
    //   block [0x826B31F0..0x826B3260)
	// 826B31F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B31F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B31F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B31FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3204: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B320C: 390B80C0  addi r8, r11, -0x7f40
	ctx.r[8].s64 = ctx.r[11].s64 + -32576;
	// 826B3210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3214: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826B3218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B321C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3228: 386ADBB4  addi r3, r10, -0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + -9292;
	// 826B322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B324C: 4BDB3BD5  bl 0x82466e20
	ctx.lr = 0x826B3250;
	sub_82466E20(ctx, base);
	// 826B3250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3260 size=108
    let mut pc: u32 = 0x826B3260;
    'dispatch: loop {
        match pc {
            0x826B3260 => {
    //   block [0x826B3260..0x826B32CC)
	// 826B3260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B326C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3274: 38EB80F0  addi r7, r11, -0x7f10
	ctx.r[7].s64 = ctx.r[11].s64 + -32528;
	// 826B3278: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B327C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826B3280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B328C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3290: 386ADBE4  addi r3, r10, -0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + -9244;
	// 826B3294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B329C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B32A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B32A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B32A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B32AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B32B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B32B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B32B8: 4BDB3B69  bl 0x82466e20
	ctx.lr = 0x826B32BC;
	sub_82466E20(ctx, base);
	// 826B32BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B32C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B32C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B32C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B32D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B32D0 size=112
    let mut pc: u32 = 0x826B32D0;
    'dispatch: loop {
        match pc {
            0x826B32D0 => {
    //   block [0x826B32D0..0x826B3340)
	// 826B32D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B32D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B32D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B32DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B32E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B32E4: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B32E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B32EC: 390B8138  addi r8, r11, -0x7ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -32456;
	// 826B32F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B32F4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826B32F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B32FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3308: 386ADC14  addi r3, r10, -0x23ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9196;
	// 826B330C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B331C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B332C: 4BDB3AF5  bl 0x82466e20
	ctx.lr = 0x826B3330;
	sub_82466E20(ctx, base);
	// 826B3330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


