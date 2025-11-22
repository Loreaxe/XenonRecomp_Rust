pub fn sub_83237B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237B60 size=120
    let mut pc: u32 = 0x83237B60;
    'dispatch: loop {
        match pc {
            0x83237B60 => {
    //   block [0x83237B60..0x83237BD8)
	// 83237B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237B68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83237B6C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237B70: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237B74: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237B78: 38CA9F28  addi r6, r10, -0x60d8
	ctx.r[6].s64 = ctx.r[10].s64 + -24792;
	// 83237B7C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83237B80: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83237B84: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237B8C: 38A97C04  addi r5, r9, 0x7c04
	ctx.r[5].s64 = ctx.r[9].s64 + 31748;
	// 83237B90: 38889FAC  addi r4, r8, -0x6054
	ctx.r[4].s64 = ctx.r[8].s64 + -24660;
	// 83237B94: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237B98: 38678C64  addi r3, r7, -0x739c
	ctx.r[3].s64 = ctx.r[7].s64 + -29596;
	// 83237B9C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237BA0: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83237BA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237BB0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237BBC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83237BC0: 4BC70609  bl 0x82ea81c8
	ctx.lr = 0x83237BC4;
	sub_82EA81C8(ctx, base);
	// 83237BC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237BD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237BD8 size=100
    let mut pc: u32 = 0x83237BD8;
    'dispatch: loop {
        match pc {
            0x83237BD8 => {
    //   block [0x83237BD8..0x83237C3C)
	// 83237BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237BE4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83237BE8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237BEC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237BF4: 38AA9174  addi r5, r10, -0x6e8c
	ctx.r[5].s64 = ctx.r[10].s64 + -28300;
	// 83237BF8: 3889A098  addi r4, r9, -0x5f68
	ctx.r[4].s64 = ctx.r[9].s64 + -24424;
	// 83237BFC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237C00: 38688C94  addi r3, r8, -0x736c
	ctx.r[3].s64 = ctx.r[8].s64 + -29548;
	// 83237C04: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237C0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237C10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237C14: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83237C18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237C1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83237C20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237C24: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83237C28: 4BC705A1  bl 0x82ea81c8
	ctx.lr = 0x83237C2C;
	sub_82EA81C8(ctx, base);
	// 83237C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237C40 size=100
    let mut pc: u32 = 0x83237C40;
    'dispatch: loop {
        match pc {
            0x83237C40 => {
    //   block [0x83237C40..0x83237CA4)
	// 83237C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237C4C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83237C50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237C54: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237C5C: 38AA8D54  addi r5, r10, -0x72ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29356;
	// 83237C60: 3889A0AC  addi r4, r9, -0x5f54
	ctx.r[4].s64 = ctx.r[9].s64 + -24404;
	// 83237C64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237C68: 38688CC4  addi r3, r8, -0x733c
	ctx.r[3].s64 = ctx.r[8].s64 + -29500;
	// 83237C6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237C74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237C78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237C7C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83237C80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237C84: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83237C88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237C8C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83237C90: 4BC70539  bl 0x82ea81c8
	ctx.lr = 0x83237C94;
	sub_82EA81C8(ctx, base);
	// 83237C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237CA8 size=100
    let mut pc: u32 = 0x83237CA8;
    'dispatch: loop {
        match pc {
            0x83237CA8 => {
    //   block [0x83237CA8..0x83237D0C)
	// 83237CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237CB4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83237CB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237CBC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237CC4: 38AA9174  addi r5, r10, -0x6e8c
	ctx.r[5].s64 = ctx.r[10].s64 + -28300;
	// 83237CC8: 3889A130  addi r4, r9, -0x5ed0
	ctx.r[4].s64 = ctx.r[9].s64 + -24272;
	// 83237CCC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237CD0: 38688CF4  addi r3, r8, -0x730c
	ctx.r[3].s64 = ctx.r[8].s64 + -29452;
	// 83237CD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237CDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237CE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237CE4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83237CE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237CEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83237CF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237CF4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83237CF8: 4BC704D1  bl 0x82ea81c8
	ctx.lr = 0x83237CFC;
	sub_82EA81C8(ctx, base);
	// 83237CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237D10 size=120
    let mut pc: u32 = 0x83237D10;
    'dispatch: loop {
        match pc {
            0x83237D10 => {
    //   block [0x83237D10..0x83237D88)
	// 83237D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83237D1C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237D20: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237D24: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237D28: 38CAA168  addi r6, r10, -0x5e98
	ctx.r[6].s64 = ctx.r[10].s64 + -24216;
	// 83237D2C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83237D30: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83237D34: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237D3C: 38A97C04  addi r5, r9, 0x7c04
	ctx.r[5].s64 = ctx.r[9].s64 + 31748;
	// 83237D40: 3888A1AC  addi r4, r8, -0x5e54
	ctx.r[4].s64 = ctx.r[8].s64 + -24148;
	// 83237D44: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237D48: 38678D24  addi r3, r7, -0x72dc
	ctx.r[3].s64 = ctx.r[7].s64 + -29404;
	// 83237D4C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237D50: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83237D54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237D60: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237D68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237D6C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 83237D70: 4BC70459  bl 0x82ea81c8
	ctx.lr = 0x83237D74;
	sub_82EA81C8(ctx, base);
	// 83237D74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237D88 size=100
    let mut pc: u32 = 0x83237D88;
    'dispatch: loop {
        match pc {
            0x83237D88 => {
    //   block [0x83237D88..0x83237DEC)
	// 83237D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237D94: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83237D98: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237D9C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237DA4: 38AA7BD4  addi r5, r10, 0x7bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 31700;
	// 83237DA8: 3889A1CC  addi r4, r9, -0x5e34
	ctx.r[4].s64 = ctx.r[9].s64 + -24116;
	// 83237DAC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237DB0: 38688D54  addi r3, r8, -0x72ac
	ctx.r[3].s64 = ctx.r[8].s64 + -29356;
	// 83237DB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237DBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237DC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237DC4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83237DC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237DCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83237DD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237DD4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83237DD8: 4BC703F1  bl 0x82ea81c8
	ctx.lr = 0x83237DDC;
	sub_82EA81C8(ctx, base);
	// 83237DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83237DF0 size=24
    let mut pc: u32 = 0x83237DF0;
    'dispatch: loop {
        match pc {
            0x83237DF0 => {
    //   block [0x83237DF0..0x83237E08)
	// 83237DF0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83237DF4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83237DF8: 392AE820  addi r9, r10, -0x17e0
	ctx.r[9].s64 = ctx.r[10].s64 + -6112;
	// 83237DFC: 816BE80C  lwz r11, -0x17f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6132 as u32) ) } as u64;
	// 83237E00: 91690038  stw r11, 0x38(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83237E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237E08 size=120
    let mut pc: u32 = 0x83237E08;
    'dispatch: loop {
        match pc {
            0x83237E08 => {
    //   block [0x83237E08..0x83237E80)
	// 83237E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237E14: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83237E18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83237E1C: 38AAE820  addi r5, r10, -0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6112;
	// 83237E20: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 83237E24: 90810074  stw r4, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[4].u32 ) };
	// 83237E28: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83237E2C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83237E30: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83237E34: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83237E38: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83237E3C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237E44: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83237E48: 3887A300  addi r4, r7, -0x5d00
	ctx.r[4].s64 = ctx.r[7].s64 + -23808;
	// 83237E4C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237E50: 38668D84  addi r3, r6, -0x727c
	ctx.r[3].s64 = ctx.r[6].s64 + -29308;
	// 83237E54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237E58: 3929A294  addi r9, r9, -0x5d6c
	ctx.r[9].s64 = ctx.r[9].s64 + -23916;
	// 83237E5C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83237E60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237E68: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 83237E6C: 4BC7035D  bl 0x82ea81c8
	ctx.lr = 0x83237E70;
	sub_82EA81C8(ctx, base);
	// 83237E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237E80 size=96
    let mut pc: u32 = 0x83237E80;
    'dispatch: loop {
        match pc {
            0x83237E80 => {
    //   block [0x83237E80..0x83237EE0)
	// 83237E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237E88: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237E8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83237E90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83237E94: 4BCA142D  bl 0x82ed92c0
	ctx.lr = 0x83237E98;
	sub_82ED92C0(ctx, base);
	// 83237E98: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83237E9C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83237EA0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 83237EA4: 396BA368  addi r11, r11, -0x5c98
	ctx.r[11].s64 = ctx.r[11].s64 + -23704;
	// 83237EA8: 38C98DB4  addi r6, r9, -0x724c
	ctx.r[6].s64 = ctx.r[9].s64 + -29260;
	// 83237EAC: 3CE082ED  lis r7, -0x7d13
	ctx.r[7].s64 = -2098397184;
	// 83237EB0: 91698DB4  stw r11, -0x724c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29260 as u32), ctx.r[11].u32 ) };
	// 83237EB4: 3D0082ED  lis r8, -0x7d13
	ctx.r[8].s64 = -2098397184;
	// 83237EB8: 392A3F10  addi r9, r10, 0x3f10
	ctx.r[9].s64 = ctx.r[10].s64 + 16144;
	// 83237EBC: 39477DB8  addi r10, r7, 0x7db8
	ctx.r[10].s64 = ctx.r[7].s64 + 32184;
	// 83237EC0: 39687DA0  addi r11, r8, 0x7da0
	ctx.r[11].s64 = ctx.r[8].s64 + 32160;
	// 83237EC4: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83237EC8: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83237ECC: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83237ED0: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 83237ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237EE0 size=100
    let mut pc: u32 = 0x83237EE0;
    'dispatch: loop {
        match pc {
            0x83237EE0 => {
    //   block [0x83237EE0..0x83237F44)
	// 83237EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237EEC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83237EF0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83237EF4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237EFC: 38AA8AE4  addi r5, r10, -0x751c
	ctx.r[5].s64 = ctx.r[10].s64 + -29980;
	// 83237F00: 3889A368  addi r4, r9, -0x5c98
	ctx.r[4].s64 = ctx.r[9].s64 + -23704;
	// 83237F04: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237F08: 38688DC4  addi r3, r8, -0x723c
	ctx.r[3].s64 = ctx.r[8].s64 + -29244;
	// 83237F0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237F14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237F18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237F1C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83237F20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237F24: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83237F28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237F2C: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 83237F30: 4BC70299  bl 0x82ea81c8
	ctx.lr = 0x83237F34;
	sub_82EA81C8(ctx, base);
	// 83237F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83237F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237F48 size=120
    let mut pc: u32 = 0x83237F48;
    'dispatch: loop {
        match pc {
            0x83237F48 => {
    //   block [0x83237F48..0x83237FC0)
	// 83237F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237F50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83237F54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237F58: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83237F5C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83237F60: 38CAA3B8  addi r6, r10, -0x5c48
	ctx.r[6].s64 = ctx.r[10].s64 + -23624;
	// 83237F64: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83237F68: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83237F6C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83237F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83237F74: 38A97C04  addi r5, r9, 0x7c04
	ctx.r[5].s64 = ctx.r[9].s64 + 31748;
	// 83237F78: 3888A42C  addi r4, r8, -0x5bd4
	ctx.r[4].s64 = ctx.r[8].s64 + -23508;
	// 83237F7C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83237F80: 38678DF4  addi r3, r7, -0x720c
	ctx.r[3].s64 = ctx.r[7].s64 + -29196;
	// 83237F84: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83237F88: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83237F8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83237F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83237F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83237F98: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83237F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83237FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83237FA4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 83237FA8: 4BC70221  bl 0x82ea81c8
	ctx.lr = 0x83237FAC;
	sub_82EA81C8(ctx, base);
	// 83237FAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83237FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83237FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83237FB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83237FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83237FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83237FC0 size=112
    let mut pc: u32 = 0x83237FC0;
    'dispatch: loop {
        match pc {
            0x83237FC0 => {
    //   block [0x83237FC0..0x83238030)
	// 83237FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83237FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83237FC8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83237FCC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83237FD0: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83237FD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83237FD8: 392BA55C  addi r9, r11, -0x5aa4
	ctx.r[9].s64 = ctx.r[11].s64 + -23204;
	// 83237FDC: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 83237FE0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 83237FE4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 83237FE8: 4BCB53F1  bl 0x82eed3d8
	ctx.lr = 0x83237FEC;
	sub_82EED3D8(ctx, base);
	// 83237FEC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83237FF0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83237FF4: 38E88E24  addi r7, r8, -0x71dc
	ctx.r[7].s64 = ctx.r[8].s64 + -29148;
	// 83237FF8: 396BA58C  addi r11, r11, -0x5a74
	ctx.r[11].s64 = ctx.r[11].s64 + -23156;
	// 83237FFC: 3D4082ED  lis r10, -0x7d13
	ctx.r[10].s64 = -2098397184;
	// 83238000: 91688E24  stw r11, -0x71dc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-29148 as u32), ctx.r[11].u32 ) };
	// 83238004: 3D2082ED  lis r9, -0x7d13
	ctx.r[9].s64 = -2098397184;
	// 83238008: 394A7E90  addi r10, r10, 0x7e90
	ctx.r[10].s64 = ctx.r[10].s64 + 32400;
	// 8323800C: 39297E78  addi r9, r9, 0x7e78
	ctx.r[9].s64 = ctx.r[9].s64 + 32376;
	// 83238010: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83238014: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83238018: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323801C: 9167000C  stw r11, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83238020: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 83238024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83238030 size=24
    let mut pc: u32 = 0x83238030;
    'dispatch: loop {
        match pc {
            0x83238030 => {
    //   block [0x83238030..0x83238048)
	// 83238030: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83238034: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238038: 392AE958  addi r9, r10, -0x16a8
	ctx.r[9].s64 = ctx.r[10].s64 + -5800;
	// 8323803C: 816BE950  lwz r11, -0x16b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5808 as u32) ) } as u64;
	// 83238040: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83238044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238048 size=116
    let mut pc: u32 = 0x83238048;
    'dispatch: loop {
        match pc {
            0x83238048 => {
    //   block [0x83238048..0x832380BC)
	// 83238048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323804C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238054: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238058: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323805C: 38AAE958  addi r5, r10, -0x16a8
	ctx.r[5].s64 = ctx.r[10].s64 + -5800;
	// 83238060: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83238064: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83238068: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323806C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83238070: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238074: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238078: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323807C: 38A87C94  addi r5, r8, 0x7c94
	ctx.r[5].s64 = ctx.r[8].s64 + 31892;
	// 83238080: 3887A58C  addi r4, r7, -0x5a74
	ctx.r[4].s64 = ctx.r[7].s64 + -23156;
	// 83238084: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238088: 38668E34  addi r3, r6, -0x71cc
	ctx.r[3].s64 = ctx.r[6].s64 + -29132;
	// 8323808C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238090: 3929A544  addi r9, r9, -0x5abc
	ctx.r[9].s64 = ctx.r[9].s64 + -23228;
	// 83238094: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323809C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832380A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832380A4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 832380A8: 4BC70121  bl 0x82ea81c8
	ctx.lr = 0x832380AC;
	sub_82EA81C8(ctx, base);
	// 832380AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832380B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832380B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832380B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832380C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832380C0 size=120
    let mut pc: u32 = 0x832380C0;
    'dispatch: loop {
        match pc {
            0x832380C0 => {
    //   block [0x832380C0..0x83238138)
	// 832380C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832380C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832380C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832380CC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832380D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832380D4: 38AAA670  addi r5, r10, -0x5990
	ctx.r[5].s64 = ctx.r[10].s64 + -22928;
	// 832380D8: 3889A5C8  addi r4, r9, -0x5a38
	ctx.r[4].s64 = ctx.r[9].s64 + -23096;
	// 832380DC: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 832380E0: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 832380E4: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 832380E8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 832380EC: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 832380F0: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 832380F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832380F8: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 832380FC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83238100: 3887A6A8  addi r4, r7, -0x5958
	ctx.r[4].s64 = ctx.r[7].s64 + -22872;
	// 83238104: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238108: 38668E64  addi r3, r6, -0x719c
	ctx.r[3].s64 = ctx.r[6].s64 + -29084;
	// 8323810C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238118: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323811C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238120: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 83238124: 4BC700A5  bl 0x82ea81c8
	ctx.lr = 0x83238128;
	sub_82EA81C8(ctx, base);
	// 83238128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323812C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238138 size=100
    let mut pc: u32 = 0x83238138;
    'dispatch: loop {
        match pc {
            0x83238138 => {
    //   block [0x83238138..0x8323819C)
	// 83238138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323813C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238144: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83238148: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323814C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238154: 38AA7BD4  addi r5, r10, 0x7bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 31700;
	// 83238158: 3889A6BC  addi r4, r9, -0x5944
	ctx.r[4].s64 = ctx.r[9].s64 + -22852;
	// 8323815C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238160: 38688E94  addi r3, r8, -0x716c
	ctx.r[3].s64 = ctx.r[8].s64 + -29036;
	// 83238164: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323816C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238170: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238174: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323817C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238180: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238184: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83238188: 4BC70041  bl 0x82ea81c8
	ctx.lr = 0x8323818C;
	sub_82EA81C8(ctx, base);
	// 8323818C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832381A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832381A0 size=100
    let mut pc: u32 = 0x832381A0;
    'dispatch: loop {
        match pc {
            0x832381A0 => {
    //   block [0x832381A0..0x83238204)
	// 832381A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832381A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832381A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832381AC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 832381B0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832381B4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832381B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832381BC: 38AA8E94  addi r5, r10, -0x716c
	ctx.r[5].s64 = ctx.r[10].s64 + -29036;
	// 832381C0: 3889A6D4  addi r4, r9, -0x592c
	ctx.r[4].s64 = ctx.r[9].s64 + -22828;
	// 832381C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832381C8: 38688EC4  addi r3, r8, -0x713c
	ctx.r[3].s64 = ctx.r[8].s64 + -28988;
	// 832381CC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832381D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832381D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832381D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832381DC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832381E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832381E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832381E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832381EC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 832381F0: 4BC6FFD9  bl 0x82ea81c8
	ctx.lr = 0x832381F4;
	sub_82EA81C8(ctx, base);
	// 832381F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832381F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832381FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238208 size=92
    let mut pc: u32 = 0x83238208;
    'dispatch: loop {
        match pc {
            0x83238208 => {
    //   block [0x83238208..0x83238264)
	// 83238208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323820C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238214: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83238218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323821C: 4BCA575D  bl 0x82edd978
	ctx.lr = 0x83238220;
	sub_82EDD978(ctx, base);
	// 83238220: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238224: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83238228: 39098EF4  addi r8, r9, -0x710c
	ctx.r[8].s64 = ctx.r[9].s64 + -28940;
	// 8323822C: 396BA740  addi r11, r11, -0x58c0
	ctx.r[11].s64 = ctx.r[11].s64 + -22720;
	// 83238230: 3D4082EE  lis r10, -0x7d12
	ctx.r[10].s64 = -2098331648;
	// 83238234: 91698EF4  stw r11, -0x710c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-28940 as u32), ctx.r[11].u32 ) };
	// 83238238: 3D2082EE  lis r9, -0x7d12
	ctx.r[9].s64 = -2098331648;
	// 8323823C: 394A80C0  addi r10, r10, -0x7f40
	ctx.r[10].s64 = ctx.r[10].s64 + -32576;
	// 83238240: 392980A8  addi r9, r9, -0x7f58
	ctx.r[9].s64 = ctx.r[9].s64 + -32600;
	// 83238244: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83238248: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323824C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83238250: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83238254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323825C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238268 size=120
    let mut pc: u32 = 0x83238268;
    'dispatch: loop {
        match pc {
            0x83238268 => {
    //   block [0x83238268..0x832382E0)
	// 83238268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323826C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238274: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238278: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323827C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83238280: 38CAA6F8  addi r6, r10, -0x5908
	ctx.r[6].s64 = ctx.r[10].s64 + -22792;
	// 83238284: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238288: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323828C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238294: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 83238298: 3888A740  addi r4, r8, -0x58c0
	ctx.r[4].s64 = ctx.r[8].s64 + -22720;
	// 8323829C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832382A0: 38678F04  addi r3, r7, -0x70fc
	ctx.r[3].s64 = ctx.r[7].s64 + -28924;
	// 832382A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832382A8: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 832382AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832382B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832382B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832382B8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832382BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832382C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832382C4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832382C8: 4BC6FF01  bl 0x82ea81c8
	ctx.lr = 0x832382CC;
	sub_82EA81C8(ctx, base);
	// 832382CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832382D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832382D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832382D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832382DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832382E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832382E0 size=120
    let mut pc: u32 = 0x832382E0;
    'dispatch: loop {
        match pc {
            0x832382E0 => {
    //   block [0x832382E0..0x83238358)
	// 832382E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832382E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832382E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832382EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832382F0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832382F4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832382F8: 38CAA7A8  addi r6, r10, -0x5858
	ctx.r[6].s64 = ctx.r[10].s64 + -22616;
	// 832382FC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238300: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238304: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323830C: 38A97BA4  addi r5, r9, 0x7ba4
	ctx.r[5].s64 = ctx.r[9].s64 + 31652;
	// 83238310: 3888A854  addi r4, r8, -0x57ac
	ctx.r[4].s64 = ctx.r[8].s64 + -22444;
	// 83238314: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238318: 38678F34  addi r3, r7, -0x70cc
	ctx.r[3].s64 = ctx.r[7].s64 + -28876;
	// 8323831C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238320: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 83238324: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323832C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238330: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238338: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323833C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83238340: 4BC6FE89  bl 0x82ea81c8
	ctx.lr = 0x83238344;
	sub_82EA81C8(ctx, base);
	// 83238344: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323834C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238358 size=92
    let mut pc: u32 = 0x83238358;
    'dispatch: loop {
        match pc {
            0x83238358 => {
    //   block [0x83238358..0x832383B4)
	// 83238358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323835C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238360: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83238368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323836C: 4BCBCBA5  bl 0x82ef4f10
	ctx.lr = 0x83238370;
	sub_82EF4F10(ctx, base);
	// 83238370: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238374: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83238378: 39098F64  addi r8, r9, -0x709c
	ctx.r[8].s64 = ctx.r[9].s64 + -28828;
	// 8323837C: 396BA8A8  addi r11, r11, -0x5758
	ctx.r[11].s64 = ctx.r[11].s64 + -22360;
	// 83238380: 3D4082EE  lis r10, -0x7d12
	ctx.r[10].s64 = -2098331648;
	// 83238384: 91698F64  stw r11, -0x709c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-28828 as u32), ctx.r[11].u32 ) };
	// 83238388: 3D2082EE  lis r9, -0x7d12
	ctx.r[9].s64 = -2098331648;
	// 8323838C: 394A8260  addi r10, r10, -0x7da0
	ctx.r[10].s64 = ctx.r[10].s64 + -32160;
	// 83238390: 39298248  addi r9, r9, -0x7db8
	ctx.r[9].s64 = ctx.r[9].s64 + -32184;
	// 83238394: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83238398: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323839C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832383A0: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832383A4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832383A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832383AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832383B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832383B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832383B8 size=120
    let mut pc: u32 = 0x832383B8;
    'dispatch: loop {
        match pc {
            0x832383B8 => {
    //   block [0x832383B8..0x83238430)
	// 832383B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832383BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832383C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832383C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832383C8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832383CC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832383D0: 38CAA878  addi r6, r10, -0x5788
	ctx.r[6].s64 = ctx.r[10].s64 + -22408;
	// 832383D4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832383D8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832383DC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832383E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832383E4: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 832383E8: 3888A8A8  addi r4, r8, -0x5758
	ctx.r[4].s64 = ctx.r[8].s64 + -22360;
	// 832383EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832383F0: 38678F74  addi r3, r7, -0x708c
	ctx.r[3].s64 = ctx.r[7].s64 + -28812;
	// 832383F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832383F8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832383FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238408: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323840C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238410: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238414: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 83238418: 4BC6FDB1  bl 0x82ea81c8
	ctx.lr = 0x8323841C;
	sub_82EA81C8(ctx, base);
	// 8323841C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238430 size=100
    let mut pc: u32 = 0x83238430;
    'dispatch: loop {
        match pc {
            0x83238430 => {
    //   block [0x83238430..0x83238494)
	// 83238430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323843C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83238440: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238444: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323844C: 38AA8E94  addi r5, r10, -0x716c
	ctx.r[5].s64 = ctx.r[10].s64 + -29036;
	// 83238450: 3889A93C  addi r4, r9, -0x56c4
	ctx.r[4].s64 = ctx.r[9].s64 + -22212;
	// 83238454: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238458: 38688FA4  addi r3, r8, -0x705c
	ctx.r[3].s64 = ctx.r[8].s64 + -28764;
	// 8323845C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238464: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238468: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323846C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238474: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323847C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83238480: 4BC6FD49  bl 0x82ea81c8
	ctx.lr = 0x83238484;
	sub_82EA81C8(ctx, base);
	// 83238484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323848C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238498 size=100
    let mut pc: u32 = 0x83238498;
    'dispatch: loop {
        match pc {
            0x83238498 => {
    //   block [0x83238498..0x832384FC)
	// 83238498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323849C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832384A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832384A4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832384A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832384AC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832384B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832384B4: 38AA7B44  addi r5, r10, 0x7b44
	ctx.r[5].s64 = ctx.r[10].s64 + 31556;
	// 832384B8: 3889A96C  addi r4, r9, -0x5694
	ctx.r[4].s64 = ctx.r[9].s64 + -22164;
	// 832384BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832384C0: 38688FD4  addi r3, r8, -0x702c
	ctx.r[3].s64 = ctx.r[8].s64 + -28716;
	// 832384C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832384C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832384CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832384D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832384D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832384D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832384DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832384E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832384E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832384E8: 4BC6FCE1  bl 0x82ea81c8
	ctx.lr = 0x832384EC;
	sub_82EA81C8(ctx, base);
	// 832384EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832384F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832384F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832384F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83238500 size=24
    let mut pc: u32 = 0x83238500;
    'dispatch: loop {
        match pc {
            0x83238500 => {
    //   block [0x83238500..0x83238518)
	// 83238500: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83238504: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238508: 392AEB38  addi r9, r10, -0x14c8
	ctx.r[9].s64 = ctx.r[10].s64 + -5320;
	// 8323850C: 816BEB18  lwz r11, -0x14e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5352 as u32) ) } as u64;
	// 83238510: 91690068  stw r11, 0x68(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83238514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238518 size=116
    let mut pc: u32 = 0x83238518;
    'dispatch: loop {
        match pc {
            0x83238518 => {
    //   block [0x83238518..0x8323858C)
	// 83238518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323851C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238524: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238528: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323852C: 38AAEB38  addi r5, r10, -0x14c8
	ctx.r[5].s64 = ctx.r[10].s64 + -5320;
	// 83238530: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83238534: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83238538: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323853C: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 83238540: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238544: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238548: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323854C: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83238550: 3887AB70  addi r4, r7, -0x5490
	ctx.r[4].s64 = ctx.r[7].s64 + -21648;
	// 83238554: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238558: 38669004  addi r3, r6, -0x6ffc
	ctx.r[3].s64 = ctx.r[6].s64 + -28668;
	// 8323855C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238560: 3929AB00  addi r9, r9, -0x5500
	ctx.r[9].s64 = ctx.r[9].s64 + -21760;
	// 83238564: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8323856C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238574: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 83238578: 4BC6FC51  bl 0x82ea81c8
	ctx.lr = 0x8323857C;
	sub_82EA81C8(ctx, base);
	// 8323857C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238590 size=120
    let mut pc: u32 = 0x83238590;
    'dispatch: loop {
        match pc {
            0x83238590 => {
    //   block [0x83238590..0x83238608)
	// 83238590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323859C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832385A0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832385A4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832385A8: 38CAABB4  addi r6, r10, -0x544c
	ctx.r[6].s64 = ctx.r[10].s64 + -21580;
	// 832385AC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832385B0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832385B4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832385B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832385BC: 38A97D54  addi r5, r9, 0x7d54
	ctx.r[5].s64 = ctx.r[9].s64 + 32084;
	// 832385C0: 3888ABF0  addi r4, r8, -0x5410
	ctx.r[4].s64 = ctx.r[8].s64 + -21520;
	// 832385C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832385C8: 38679034  addi r3, r7, -0x6fcc
	ctx.r[3].s64 = ctx.r[7].s64 + -28620;
	// 832385CC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832385D0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 832385D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832385D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832385DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832385E0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832385E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832385E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832385EC: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 832385F0: 4BC6FBD9  bl 0x82ea81c8
	ctx.lr = 0x832385F4;
	sub_82EA81C8(ctx, base);
	// 832385F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832385F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832385FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238608 size=120
    let mut pc: u32 = 0x83238608;
    'dispatch: loop {
        match pc {
            0x83238608 => {
    //   block [0x83238608..0x83238680)
	// 83238608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238614: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238618: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323861C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238620: 38CAAC2C  addi r6, r10, -0x53d4
	ctx.r[6].s64 = ctx.r[10].s64 + -21460;
	// 83238624: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238628: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323862C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238634: 38A99004  addi r5, r9, -0x6ffc
	ctx.r[5].s64 = ctx.r[9].s64 + -28668;
	// 83238638: 3888AC7C  addi r4, r8, -0x5384
	ctx.r[4].s64 = ctx.r[8].s64 + -21380;
	// 8323863C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238640: 38679064  addi r3, r7, -0x6f9c
	ctx.r[3].s64 = ctx.r[7].s64 + -28572;
	// 83238644: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238648: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323864C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238658: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238664: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 83238668: 4BC6FB61  bl 0x82ea81c8
	ctx.lr = 0x8323866C;
	sub_82EA81C8(ctx, base);
	// 8323866C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238680 size=100
    let mut pc: u32 = 0x83238680;
    'dispatch: loop {
        match pc {
            0x83238680 => {
    //   block [0x83238680..0x832386E4)
	// 83238680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323868C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83238690: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238694: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323869C: 38AA7BD4  addi r5, r10, 0x7bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 31700;
	// 832386A0: 3889AC98  addi r4, r9, -0x5368
	ctx.r[4].s64 = ctx.r[9].s64 + -21352;
	// 832386A4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832386A8: 38689094  addi r3, r8, -0x6f6c
	ctx.r[3].s64 = ctx.r[8].s64 + -28524;
	// 832386AC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832386B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832386B4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832386B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832386BC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832386C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832386C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832386C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832386CC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 832386D0: 4BC6FAF9  bl 0x82ea81c8
	ctx.lr = 0x832386D4;
	sub_82EA81C8(ctx, base);
	// 832386D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832386D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832386DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832386E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832386E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832386E8 size=24
    let mut pc: u32 = 0x832386E8;
    'dispatch: loop {
        match pc {
            0x832386E8 => {
    //   block [0x832386E8..0x83238700)
	// 832386E8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832386EC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832386F0: 392AECF0  addi r9, r10, -0x1310
	ctx.r[9].s64 = ctx.r[10].s64 + -4880;
	// 832386F4: 816BECD8  lwz r11, -0x1328(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4904 as u32) ) } as u64;
	// 832386F8: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 832386FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238700 size=116
    let mut pc: u32 = 0x83238700;
    'dispatch: loop {
        match pc {
            0x83238700 => {
    //   block [0x83238700..0x83238774)
	// 83238700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323870C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238710: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83238714: 38AAECF0  addi r5, r10, -0x1310
	ctx.r[5].s64 = ctx.r[10].s64 + -4880;
	// 83238718: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323871C: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83238720: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83238724: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83238728: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323872C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238730: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83238734: 38A87C04  addi r5, r8, 0x7c04
	ctx.r[5].s64 = ctx.r[8].s64 + 31748;
	// 83238738: 3887ADCC  addi r4, r7, -0x5234
	ctx.r[4].s64 = ctx.r[7].s64 + -21044;
	// 8323873C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238740: 386690C4  addi r3, r6, -0x6f3c
	ctx.r[3].s64 = ctx.r[6].s64 + -28476;
	// 83238744: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238748: 3929ADA4  addi r9, r9, -0x525c
	ctx.r[9].s64 = ctx.r[9].s64 + -21084;
	// 8323874C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83238754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238758: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323875C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 83238760: 4BC6FA69  bl 0x82ea81c8
	ctx.lr = 0x83238764;
	sub_82EA81C8(ctx, base);
	// 83238764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323876C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238778 size=92
    let mut pc: u32 = 0x83238778;
    'dispatch: loop {
        match pc {
            0x83238778 => {
    //   block [0x83238778..0x832387D4)
	// 83238778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238780: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238784: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83238788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323878C: 4BC9FF6D  bl 0x82ed86f8
	ctx.lr = 0x83238790;
	sub_82ED86F8(ctx, base);
	// 83238790: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238794: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83238798: 390990F4  addi r8, r9, -0x6f0c
	ctx.r[8].s64 = ctx.r[9].s64 + -28428;
	// 8323879C: 396BAE24  addi r11, r11, -0x51dc
	ctx.r[11].s64 = ctx.r[11].s64 + -20956;
	// 832387A0: 3D4082EE  lis r10, -0x7d12
	ctx.r[10].s64 = -2098331648;
	// 832387A4: 916990F4  stw r11, -0x6f0c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-28428 as u32), ctx.r[11].u32 ) };
	// 832387A8: 3D2082EE  lis r9, -0x7d12
	ctx.r[9].s64 = -2098331648;
	// 832387AC: 394A86B0  addi r10, r10, -0x7950
	ctx.r[10].s64 = ctx.r[10].s64 + -31056;
	// 832387B0: 39298698  addi r9, r9, -0x7968
	ctx.r[9].s64 = ctx.r[9].s64 + -31080;
	// 832387B4: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832387B8: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832387BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832387C0: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832387C4: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 832387C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832387CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832387D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832387D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832387D8 size=120
    let mut pc: u32 = 0x832387D8;
    'dispatch: loop {
        match pc {
            0x832387D8 => {
    //   block [0x832387D8..0x83238850)
	// 832387D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832387DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832387E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832387E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832387E8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832387EC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832387F0: 38CAAE0C  addi r6, r10, -0x51f4
	ctx.r[6].s64 = ctx.r[10].s64 + -20980;
	// 832387F4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832387F8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832387FC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238804: 38A97B14  addi r5, r9, 0x7b14
	ctx.r[5].s64 = ctx.r[9].s64 + 31508;
	// 83238808: 3888AE24  addi r4, r8, -0x51dc
	ctx.r[4].s64 = ctx.r[8].s64 + -20956;
	// 8323880C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238810: 38679104  addi r3, r7, -0x6efc
	ctx.r[3].s64 = ctx.r[7].s64 + -28412;
	// 83238814: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238818: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323881C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238828: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323882C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238834: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 83238838: 4BC6F991  bl 0x82ea81c8
	ctx.lr = 0x8323883C;
	sub_82EA81C8(ctx, base);
	// 8323883C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323884C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238850 size=92
    let mut pc: u32 = 0x83238850;
    'dispatch: loop {
        match pc {
            0x83238850 => {
    //   block [0x83238850..0x832388AC)
	// 83238850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238858: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323885C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83238860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83238864: 4BCBDF45  bl 0x82ef67a8
	ctx.lr = 0x83238868;
	sub_82EF67A8(ctx, base);
	// 83238868: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323886C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83238870: 39099134  addi r8, r9, -0x6ecc
	ctx.r[8].s64 = ctx.r[9].s64 + -28364;
	// 83238874: 396BAF78  addi r11, r11, -0x5088
	ctx.r[11].s64 = ctx.r[11].s64 + -20616;
	// 83238878: 3D4082EE  lis r10, -0x7d12
	ctx.r[10].s64 = -2098331648;
	// 8323887C: 91699134  stw r11, -0x6ecc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-28364 as u32), ctx.r[11].u32 ) };
	// 83238880: 3D2082EE  lis r9, -0x7d12
	ctx.r[9].s64 = -2098331648;
	// 83238884: 394A8778  addi r10, r10, -0x7888
	ctx.r[10].s64 = ctx.r[10].s64 + -30856;
	// 83238888: 39298760  addi r9, r9, -0x78a0
	ctx.r[9].s64 = ctx.r[9].s64 + -30880;
	// 8323888C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83238890: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83238894: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83238898: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323889C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832388A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832388A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832388A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832388B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832388B0 size=120
    let mut pc: u32 = 0x832388B0;
    'dispatch: loop {
        match pc {
            0x832388B0 => {
    //   block [0x832388B0..0x83238928)
	// 832388B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832388B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832388B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832388BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832388C0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832388C4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 832388C8: 38CAAEB8  addi r6, r10, -0x5148
	ctx.r[6].s64 = ctx.r[10].s64 + -20808;
	// 832388CC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832388D0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832388D4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832388D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832388DC: 38A97C94  addi r5, r9, 0x7c94
	ctx.r[5].s64 = ctx.r[9].s64 + 31892;
	// 832388E0: 3888AF78  addi r4, r8, -0x5088
	ctx.r[4].s64 = ctx.r[8].s64 + -20616;
	// 832388E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832388E8: 38679144  addi r3, r7, -0x6ebc
	ctx.r[3].s64 = ctx.r[7].s64 + -28348;
	// 832388EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832388F0: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 832388F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832388F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832388FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238900: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238908: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323890C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 83238910: 4BC6F8B9  bl 0x82ea81c8
	ctx.lr = 0x83238914;
	sub_82EA81C8(ctx, base);
	// 83238914: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323891C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238928 size=100
    let mut pc: u32 = 0x83238928;
    'dispatch: loop {
        match pc {
            0x83238928 => {
    //   block [0x83238928..0x8323898C)
	// 83238928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238934: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83238938: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323893C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238944: 38AA7BD4  addi r5, r10, 0x7bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 31700;
	// 83238948: 3889AF94  addi r4, r9, -0x506c
	ctx.r[4].s64 = ctx.r[9].s64 + -20588;
	// 8323894C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238950: 38689174  addi r3, r8, -0x6e8c
	ctx.r[3].s64 = ctx.r[8].s64 + -28300;
	// 83238954: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323895C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238960: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238964: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238968: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323896C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238970: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238974: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 83238978: 4BC6F851  bl 0x82ea81c8
	ctx.lr = 0x8323897C;
	sub_82EA81C8(ctx, base);
	// 8323897C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238990 size=92
    let mut pc: u32 = 0x83238990;
    'dispatch: loop {
        match pc {
            0x83238990 => {
    //   block [0x83238990..0x832389EC)
	// 83238990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238998: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323899C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832389A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832389A4: 4BC9FEC5  bl 0x82ed8868
	ctx.lr = 0x832389A8;
	sub_82ED8868(ctx, base);
	// 832389A8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832389AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 832389B0: 390991A4  addi r8, r9, -0x6e5c
	ctx.r[8].s64 = ctx.r[9].s64 + -28252;
	// 832389B4: 396BB02C  addi r11, r11, -0x4fd4
	ctx.r[11].s64 = ctx.r[11].s64 + -20436;
	// 832389B8: 3D4082EE  lis r10, -0x7d12
	ctx.r[10].s64 = -2098331648;
	// 832389BC: 916991A4  stw r11, -0x6e5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-28252 as u32), ctx.r[11].u32 ) };
	// 832389C0: 3D2082EE  lis r9, -0x7d12
	ctx.r[9].s64 = -2098331648;
	// 832389C4: 394A8820  addi r10, r10, -0x77e0
	ctx.r[10].s64 = ctx.r[10].s64 + -30688;
	// 832389C8: 39298808  addi r9, r9, -0x77f8
	ctx.r[9].s64 = ctx.r[9].s64 + -30712;
	// 832389CC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832389D0: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832389D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832389D8: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832389DC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 832389E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832389E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832389E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832389F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832389F0 size=120
    let mut pc: u32 = 0x832389F0;
    'dispatch: loop {
        match pc {
            0x832389F0 => {
    //   block [0x832389F0..0x83238A68)
	// 832389F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832389F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832389F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832389FC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238A00: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238A04: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83238A08: 38CAAFC4  addi r6, r10, -0x503c
	ctx.r[6].s64 = ctx.r[10].s64 + -20540;
	// 83238A0C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238A10: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238A14: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238A1C: 38A97B74  addi r5, r9, 0x7b74
	ctx.r[5].s64 = ctx.r[9].s64 + 31604;
	// 83238A20: 3888B02C  addi r4, r8, -0x4fd4
	ctx.r[4].s64 = ctx.r[8].s64 + -20436;
	// 83238A24: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238A28: 386791B4  addi r3, r7, -0x6e4c
	ctx.r[3].s64 = ctx.r[7].s64 + -28236;
	// 83238A2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238A30: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83238A34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238A40: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238A48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238A4C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 83238A50: 4BC6F779  bl 0x82ea81c8
	ctx.lr = 0x83238A54;
	sub_82EA81C8(ctx, base);
	// 83238A54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83238A68 size=12
    let mut pc: u32 = 0x83238A68;
    'dispatch: loop {
        match pc {
            0x83238A68 => {
    //   block [0x83238A68..0x83238A74)
	// 83238A68: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 83238A6C: 386B1ED8  addi r3, r11, 0x1ed8
	ctx.r[3].s64 = ctx.r[11].s64 + 7896;
	// 83238A70: 4BF6FA68  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238A78 size=100
    let mut pc: u32 = 0x83238A78;
    'dispatch: loop {
        match pc {
            0x83238A78 => {
    //   block [0x83238A78..0x83238ADC)
	// 83238A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238A84: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83238A88: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238A8C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238A94: 38AA9344  addi r5, r10, -0x6cbc
	ctx.r[5].s64 = ctx.r[10].s64 + -27836;
	// 83238A98: 3889BFA8  addi r4, r9, -0x4058
	ctx.r[4].s64 = ctx.r[9].s64 + -16472;
	// 83238A9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238AA0: 38689254  addi r3, r8, -0x6dac
	ctx.r[3].s64 = ctx.r[8].s64 + -28076;
	// 83238AA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238AAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238AB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238AB4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238AB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238ABC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238AC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238AC4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83238AC8: 4BC6F701  bl 0x82ea81c8
	ctx.lr = 0x83238ACC;
	sub_82EA81C8(ctx, base);
	// 83238ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238AE0 size=96
    let mut pc: u32 = 0x83238AE0;
    'dispatch: loop {
        match pc {
            0x83238AE0 => {
    //   block [0x83238AE0..0x83238B40)
	// 83238AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238AEC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238AF0: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238AF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238AF8: 388ABFE0  addi r4, r10, -0x4020
	ctx.r[4].s64 = ctx.r[10].s64 + -16416;
	// 83238AFC: 38699284  addi r3, r9, -0x6d7c
	ctx.r[3].s64 = ctx.r[9].s64 + -28028;
	// 83238B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238B14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83238B18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238B20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238B24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83238B28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83238B2C: 4BC6F69D  bl 0x82ea81c8
	ctx.lr = 0x83238B30;
	sub_82EA81C8(ctx, base);
	// 83238B30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238B40 size=120
    let mut pc: u32 = 0x83238B40;
    'dispatch: loop {
        match pc {
            0x83238B40 => {
    //   block [0x83238B40..0x83238BB8)
	// 83238B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238B4C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238B50: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238B54: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238B58: 38CABFC8  addi r6, r10, -0x4038
	ctx.r[6].s64 = ctx.r[10].s64 + -16440;
	// 83238B5C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238B60: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238B64: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238B6C: 38A99284  addi r5, r9, -0x6d7c
	ctx.r[5].s64 = ctx.r[9].s64 + -28028;
	// 83238B70: 3888BFF4  addi r4, r8, -0x400c
	ctx.r[4].s64 = ctx.r[8].s64 + -16396;
	// 83238B74: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238B78: 386792B4  addi r3, r7, -0x6d4c
	ctx.r[3].s64 = ctx.r[7].s64 + -27980;
	// 83238B7C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238B80: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83238B84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238B90: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238B98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238B9C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83238BA0: 4BC6F629  bl 0x82ea81c8
	ctx.lr = 0x83238BA4;
	sub_82EA81C8(ctx, base);
	// 83238BA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238BB8 size=96
    let mut pc: u32 = 0x83238BB8;
    'dispatch: loop {
        match pc {
            0x83238BB8 => {
    //   block [0x83238BB8..0x83238C18)
	// 83238BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238BC4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238BC8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238BCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238BD0: 388AC00C  addi r4, r10, -0x3ff4
	ctx.r[4].s64 = ctx.r[10].s64 + -16372;
	// 83238BD4: 386992E4  addi r3, r9, -0x6d1c
	ctx.r[3].s64 = ctx.r[9].s64 + -27932;
	// 83238BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238BDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238BEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83238BF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238BF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238BFC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83238C00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83238C04: 4BC6F5C5  bl 0x82ea81c8
	ctx.lr = 0x83238C08;
	sub_82EA81C8(ctx, base);
	// 83238C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238C18 size=120
    let mut pc: u32 = 0x83238C18;
    'dispatch: loop {
        match pc {
            0x83238C18 => {
    //   block [0x83238C18..0x83238C90)
	// 83238C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238C24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238C28: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238C2C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238C30: 38CAC038  addi r6, r10, -0x3fc8
	ctx.r[6].s64 = ctx.r[10].s64 + -16328;
	// 83238C34: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238C38: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238C3C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238C44: 38A99344  addi r5, r9, -0x6cbc
	ctx.r[5].s64 = ctx.r[9].s64 + -27836;
	// 83238C48: 3888C050  addi r4, r8, -0x3fb0
	ctx.r[4].s64 = ctx.r[8].s64 + -16304;
	// 83238C4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238C50: 38679314  addi r3, r7, -0x6cec
	ctx.r[3].s64 = ctx.r[7].s64 + -27884;
	// 83238C54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238C58: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83238C5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238C68: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238C6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83238C70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238C74: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83238C78: 4BC6F551  bl 0x82ea81c8
	ctx.lr = 0x83238C7C;
	sub_82EA81C8(ctx, base);
	// 83238C7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238C90 size=120
    let mut pc: u32 = 0x83238C90;
    'dispatch: loop {
        match pc {
            0x83238C90 => {
    //   block [0x83238C90..0x83238D08)
	// 83238C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238C9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238CA0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238CA4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83238CA8: 38CAC064  addi r6, r10, -0x3f9c
	ctx.r[6].s64 = ctx.r[10].s64 + -16284;
	// 83238CAC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238CB0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238CB4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238CBC: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 83238CC0: 3888C094  addi r4, r8, -0x3f6c
	ctx.r[4].s64 = ctx.r[8].s64 + -16236;
	// 83238CC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238CC8: 38679344  addi r3, r7, -0x6cbc
	ctx.r[3].s64 = ctx.r[7].s64 + -27836;
	// 83238CCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238CD0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83238CD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238CE0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238CE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238CEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83238CF0: 4BC6F4D9  bl 0x82ea81c8
	ctx.lr = 0x83238CF4;
	sub_82EA81C8(ctx, base);
	// 83238CF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238D08 size=120
    let mut pc: u32 = 0x83238D08;
    'dispatch: loop {
        match pc {
            0x83238D08 => {
    //   block [0x83238D08..0x83238D80)
	// 83238D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83238D14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238D18: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238D1C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238D20: 38CAC128  addi r6, r10, -0x3ed8
	ctx.r[6].s64 = ctx.r[10].s64 + -16088;
	// 83238D24: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83238D28: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83238D2C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83238D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238D34: 38A99434  addi r5, r9, -0x6bcc
	ctx.r[5].s64 = ctx.r[9].s64 + -27596;
	// 83238D38: 3888C1E8  addi r4, r8, -0x3e18
	ctx.r[4].s64 = ctx.r[8].s64 + -15896;
	// 83238D3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238D40: 38679374  addi r3, r7, -0x6c8c
	ctx.r[3].s64 = ctx.r[7].s64 + -27788;
	// 83238D44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238D48: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 83238D4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238D58: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83238D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238D60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238D64: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 83238D68: 4BC6F461  bl 0x82ea81c8
	ctx.lr = 0x83238D6C;
	sub_82EA81C8(ctx, base);
	// 83238D6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83238D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83238D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238D80 size=96
    let mut pc: u32 = 0x83238D80;
    'dispatch: loop {
        match pc {
            0x83238D80 => {
    //   block [0x83238D80..0x83238DE0)
	// 83238D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238D8C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238D90: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238D94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238D98: 388AC204  addi r4, r10, -0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -15868;
	// 83238D9C: 386993A4  addi r3, r9, -0x6c5c
	ctx.r[3].s64 = ctx.r[9].s64 + -27740;
	// 83238DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238DA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238DB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83238DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238DC4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83238DC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83238DCC: 4BC6F3FD  bl 0x82ea81c8
	ctx.lr = 0x83238DD0;
	sub_82EA81C8(ctx, base);
	// 83238DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238DE0 size=96
    let mut pc: u32 = 0x83238DE0;
    'dispatch: loop {
        match pc {
            0x83238DE0 => {
    //   block [0x83238DE0..0x83238E40)
	// 83238DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238DEC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238DF0: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83238DF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238DF8: 388AC220  addi r4, r10, -0x3de0
	ctx.r[4].s64 = ctx.r[10].s64 + -15840;
	// 83238DFC: 386993D4  addi r3, r9, -0x6c2c
	ctx.r[3].s64 = ctx.r[9].s64 + -27692;
	// 83238E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238E04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238E14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83238E18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238E20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238E24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83238E28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83238E2C: 4BC6F39D  bl 0x82ea81c8
	ctx.lr = 0x83238E30;
	sub_82EA81C8(ctx, base);
	// 83238E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238E40 size=100
    let mut pc: u32 = 0x83238E40;
    'dispatch: loop {
        match pc {
            0x83238E40 => {
    //   block [0x83238E40..0x83238EA4)
	// 83238E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238E4C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83238E50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238E54: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238E5C: 38AA9344  addi r5, r10, -0x6cbc
	ctx.r[5].s64 = ctx.r[10].s64 + -27836;
	// 83238E60: 3889C238  addi r4, r9, -0x3dc8
	ctx.r[4].s64 = ctx.r[9].s64 + -15816;
	// 83238E64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238E68: 38689404  addi r3, r8, -0x6bfc
	ctx.r[3].s64 = ctx.r[8].s64 + -27644;
	// 83238E6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238E74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238E78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238E7C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238E84: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238E8C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83238E90: 4BC6F339  bl 0x82ea81c8
	ctx.lr = 0x83238E94;
	sub_82EA81C8(ctx, base);
	// 83238E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238EA8 size=100
    let mut pc: u32 = 0x83238EA8;
    'dispatch: loop {
        match pc {
            0x83238EA8 => {
    //   block [0x83238EA8..0x83238F0C)
	// 83238EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238EB4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83238EB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238EBC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238EC4: 38AA9344  addi r5, r10, -0x6cbc
	ctx.r[5].s64 = ctx.r[10].s64 + -27836;
	// 83238EC8: 3889C250  addi r4, r9, -0x3db0
	ctx.r[4].s64 = ctx.r[9].s64 + -15792;
	// 83238ECC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238ED0: 38689434  addi r3, r8, -0x6bcc
	ctx.r[3].s64 = ctx.r[8].s64 + -27596;
	// 83238ED4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83238EDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238EE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83238EE4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238EE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238EEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238EF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238EF4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83238EF8: 4BC6F2D1  bl 0x82ea81c8
	ctx.lr = 0x83238EFC;
	sub_82EA81C8(ctx, base);
	// 83238EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238F10 size=112
    let mut pc: u32 = 0x83238F10;
    'dispatch: loop {
        match pc {
            0x83238F10 => {
    //   block [0x83238F10..0x83238F80)
	// 83238F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238F1C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238F20: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83238F24: 38AAC320  addi r5, r10, -0x3ce0
	ctx.r[5].s64 = ctx.r[10].s64 + -15584;
	// 83238F28: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83238F2C: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83238F30: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83238F34: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83238F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238F3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83238F40: 38A89254  addi r5, r8, -0x6dac
	ctx.r[5].s64 = ctx.r[8].s64 + -28076;
	// 83238F44: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238F48: 3887C338  addi r4, r7, -0x3cc8
	ctx.r[4].s64 = ctx.r[7].s64 + -15560;
	// 83238F4C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238F50: 38669464  addi r3, r6, -0x6b9c
	ctx.r[3].s64 = ctx.r[6].s64 + -27548;
	// 83238F54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238F58: 3929C30C  addi r9, r9, -0x3cf4
	ctx.r[9].s64 = ctx.r[9].s64 + -15604;
	// 83238F5C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83238F60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238F68: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83238F6C: 4BC6F25D  bl 0x82ea81c8
	ctx.lr = 0x83238F70;
	sub_82EA81C8(ctx, base);
	// 83238F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83238F80 size=104
    let mut pc: u32 = 0x83238F80;
    'dispatch: loop {
        match pc {
            0x83238F80 => {
    //   block [0x83238F80..0x83238FE8)
	// 83238F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83238F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83238F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83238F8C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83238F90: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83238F94: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83238F98: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83238F9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83238FA0: 392AC3CC  addi r9, r10, -0x3c34
	ctx.r[9].s64 = ctx.r[10].s64 + -15412;
	// 83238FA4: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83238FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83238FAC: 3887C3E0  addi r4, r7, -0x3c20
	ctx.r[4].s64 = ctx.r[7].s64 + -15392;
	// 83238FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83238FB4: 38669494  addi r3, r6, -0x6b6c
	ctx.r[3].s64 = ctx.r[6].s64 + -27500;
	// 83238FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83238FBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83238FC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83238FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83238FC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83238FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83238FD0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83238FD4: 4BC6F1F5  bl 0x82ea81c8
	ctx.lr = 0x83238FD8;
	sub_82EA81C8(ctx, base);
	// 83238FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83238FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83238FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83238FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83238FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83238FE8 size=24
    let mut pc: u32 = 0x83238FE8;
    'dispatch: loop {
        match pc {
            0x83238FE8 => {
    //   block [0x83238FE8..0x83239000)
	// 83238FE8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83238FEC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83238FF0: 392AF690  addi r9, r10, -0x970
	ctx.r[9].s64 = ctx.r[10].s64 + -2416;
	// 83238FF4: 816BF688  lwz r11, -0x978(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2424 as u32) ) } as u64;
	// 83238FF8: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83238FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239000 size=116
    let mut pc: u32 = 0x83239000;
    'dispatch: loop {
        match pc {
            0x83239000 => {
    //   block [0x83239000..0x83239074)
	// 83239000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323900C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239010: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 83239014: 38AAF690  addi r5, r10, -0x970
	ctx.r[5].s64 = ctx.r[10].s64 + -2416;
	// 83239018: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323901C: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83239020: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83239024: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83239028: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323902C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239030: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83239034: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 83239038: 3887C4E0  addi r4, r7, -0x3b20
	ctx.r[4].s64 = ctx.r[7].s64 + -15136;
	// 8323903C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239040: 386694C4  addi r3, r6, -0x6b3c
	ctx.r[3].s64 = ctx.r[6].s64 + -27452;
	// 83239044: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239048: 3929C4C4  addi r9, r9, -0x3b3c
	ctx.r[9].s64 = ctx.r[9].s64 + -15164;
	// 8323904C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239050: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83239054: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 83239058: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323905C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83239060: 4BC6F169  bl 0x82ea81c8
	ctx.lr = 0x83239064;
	sub_82EA81C8(ctx, base);
	// 83239064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323906C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239078 size=96
    let mut pc: u32 = 0x83239078;
    'dispatch: loop {
        match pc {
            0x83239078 => {
    //   block [0x83239078..0x832390D8)
	// 83239078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239084: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239088: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323908C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239090: 388AC4F4  addi r4, r10, -0x3b0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15116;
	// 83239094: 386994F4  addi r3, r9, -0x6b0c
	ctx.r[3].s64 = ctx.r[9].s64 + -27404;
	// 83239098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323909C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832390A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832390A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832390A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832390AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832390B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832390B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832390B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832390BC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832390C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832390C4: 4BC6F105  bl 0x82ea81c8
	ctx.lr = 0x832390C8;
	sub_82EA81C8(ctx, base);
	// 832390C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832390CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832390D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832390D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832390D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832390D8 size=100
    let mut pc: u32 = 0x832390D8;
    'dispatch: loop {
        match pc {
            0x832390D8 => {
    //   block [0x832390D8..0x8323913C)
	// 832390D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832390DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832390E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832390E4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 832390E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832390EC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832390F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832390F4: 38AA9344  addi r5, r10, -0x6cbc
	ctx.r[5].s64 = ctx.r[10].s64 + -27836;
	// 832390F8: 3889C514  addi r4, r9, -0x3aec
	ctx.r[4].s64 = ctx.r[9].s64 + -15084;
	// 832390FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239100: 38689524  addi r3, r8, -0x6adc
	ctx.r[3].s64 = ctx.r[8].s64 + -27356;
	// 83239104: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323910C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239110: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239114: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83239118: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323911C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83239120: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239124: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83239128: 4BC6F0A1  bl 0x82ea81c8
	ctx.lr = 0x8323912C;
	sub_82EA81C8(ctx, base);
	// 8323912C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239140 size=120
    let mut pc: u32 = 0x83239140;
    'dispatch: loop {
        match pc {
            0x83239140 => {
    //   block [0x83239140..0x832391B8)
	// 83239140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323914C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239150: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239154: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239158: 38CAC558  addi r6, r10, -0x3aa8
	ctx.r[6].s64 = ctx.r[10].s64 + -15016;
	// 8323915C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239160: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239164: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323916C: 38A99524  addi r5, r9, -0x6adc
	ctx.r[5].s64 = ctx.r[9].s64 + -27356;
	// 83239170: 3888C5E8  addi r4, r8, -0x3a18
	ctx.r[4].s64 = ctx.r[8].s64 + -14872;
	// 83239174: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239178: 38679554  addi r3, r7, -0x6aac
	ctx.r[3].s64 = ctx.r[7].s64 + -27308;
	// 8323917C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239180: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83239184: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323918C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239190: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323919C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832391A0: 4BC6F029  bl 0x82ea81c8
	ctx.lr = 0x832391A4;
	sub_82EA81C8(ctx, base);
	// 832391A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832391A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832391AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832391B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832391B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832391B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832391B8 size=120
    let mut pc: u32 = 0x832391B8;
    'dispatch: loop {
        match pc {
            0x832391B8 => {
    //   block [0x832391B8..0x83239230)
	// 832391B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832391BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832391C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832391C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832391C8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832391CC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832391D0: 38CAC5B8  addi r6, r10, -0x3a48
	ctx.r[6].s64 = ctx.r[10].s64 + -14920;
	// 832391D4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832391D8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832391DC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832391E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832391E4: 38A99554  addi r5, r9, -0x6aac
	ctx.r[5].s64 = ctx.r[9].s64 + -27308;
	// 832391E8: 3888C600  addi r4, r8, -0x3a00
	ctx.r[4].s64 = ctx.r[8].s64 + -14848;
	// 832391EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832391F0: 38679584  addi r3, r7, -0x6a7c
	ctx.r[3].s64 = ctx.r[7].s64 + -27260;
	// 832391F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832391F8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 832391FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239208: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323920C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239210: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239214: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83239218: 4BC6EFB1  bl 0x82ea81c8
	ctx.lr = 0x8323921C;
	sub_82EA81C8(ctx, base);
	// 8323921C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323922C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239230 size=100
    let mut pc: u32 = 0x83239230;
    'dispatch: loop {
        match pc {
            0x83239230 => {
    //   block [0x83239230..0x83239294)
	// 83239230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323923C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239240: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239244: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323924C: 392AC724  addi r9, r10, -0x38dc
	ctx.r[9].s64 = ctx.r[10].s64 + -14556;
	// 83239250: 3888C760  addi r4, r8, -0x38a0
	ctx.r[4].s64 = ctx.r[8].s64 + -14496;
	// 83239254: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239258: 386795B4  addi r3, r7, -0x6a4c
	ctx.r[3].s64 = ctx.r[7].s64 + -27212;
	// 8323925C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239260: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83239264: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239268: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323926C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83239270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239274: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83239278: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8323927C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239280: 4BC6EF49  bl 0x82ea81c8
	ctx.lr = 0x83239284;
	sub_82EA81C8(ctx, base);
	// 83239284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239298 size=120
    let mut pc: u32 = 0x83239298;
    'dispatch: loop {
        match pc {
            0x83239298 => {
    //   block [0x83239298..0x83239310)
	// 83239298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323929C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832392A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832392A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832392A8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832392AC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832392B0: 38CAC7A8  addi r6, r10, -0x3858
	ctx.r[6].s64 = ctx.r[10].s64 + -14424;
	// 832392B4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832392B8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832392BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832392C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832392C4: 38A9A8F8  addi r5, r9, -0x5708
	ctx.r[5].s64 = ctx.r[9].s64 + -22280;
	// 832392C8: 3888C808  addi r4, r8, -0x37f8
	ctx.r[4].s64 = ctx.r[8].s64 + -14328;
	// 832392CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832392D0: 386795E4  addi r3, r7, -0x6a1c
	ctx.r[3].s64 = ctx.r[7].s64 + -27164;
	// 832392D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832392D8: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 832392DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832392E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832392E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832392E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 832392EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832392F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832392F4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832392F8: 4BC6EED1  bl 0x82ea81c8
	ctx.lr = 0x832392FC;
	sub_82EA81C8(ctx, base);
	// 832392FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323930C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239310 size=108
    let mut pc: u32 = 0x83239310;
    'dispatch: loop {
        match pc {
            0x83239310 => {
    //   block [0x83239310..0x8323937C)
	// 83239310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323931C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239320: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239324: 38EAC868  addi r7, r10, -0x3798
	ctx.r[7].s64 = ctx.r[10].s64 + -14232;
	// 83239328: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323932C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83239330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239338: 3889C92C  addi r4, r9, -0x36d4
	ctx.r[4].s64 = ctx.r[9].s64 + -14036;
	// 8323933C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239340: 38689614  addi r3, r8, -0x69ec
	ctx.r[3].s64 = ctx.r[8].s64 + -27116;
	// 83239344: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323934C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239350: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239354: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239358: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323935C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239360: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83239364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239368: 4BC6EE61  bl 0x82ea81c8
	ctx.lr = 0x8323936C;
	sub_82EA81C8(ctx, base);
	// 8323936C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239380 size=24
    let mut pc: u32 = 0x83239380;
    'dispatch: loop {
        match pc {
            0x83239380 => {
    //   block [0x83239380..0x83239398)
	// 83239380: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83239384: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239388: 392AF758  addi r9, r10, -0x8a8
	ctx.r[9].s64 = ctx.r[10].s64 + -2216;
	// 8323938C: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 83239390: 91690068  stw r11, 0x68(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83239394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239398 size=120
    let mut pc: u32 = 0x83239398;
    'dispatch: loop {
        match pc {
            0x83239398 => {
    //   block [0x83239398..0x83239410)
	// 83239398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832393A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832393A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832393A8: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 832393AC: 38AAC8C8  addi r5, r10, -0x3738
	ctx.r[5].s64 = ctx.r[10].s64 + -14136;
	// 832393B0: 3889F758  addi r4, r9, -0x8a8
	ctx.r[4].s64 = ctx.r[9].s64 + -2216;
	// 832393B4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832393B8: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 832393BC: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 832393C0: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 832393C4: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 832393C8: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 832393CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832393D0: 38A89314  addi r5, r8, -0x6cec
	ctx.r[5].s64 = ctx.r[8].s64 + -27884;
	// 832393D4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832393D8: 3887C948  addi r4, r7, -0x36b8
	ctx.r[4].s64 = ctx.r[7].s64 + -14008;
	// 832393DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832393E0: 38669644  addi r3, r6, -0x69bc
	ctx.r[3].s64 = ctx.r[6].s64 + -27068;
	// 832393E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832393E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832393EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832393F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832393F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832393F8: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 832393FC: 4BC6EDCD  bl 0x82ea81c8
	ctx.lr = 0x83239400;
	sub_82EA81C8(ctx, base);
	// 83239400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239410 size=108
    let mut pc: u32 = 0x83239410;
    'dispatch: loop {
        match pc {
            0x83239410 => {
    //   block [0x83239410..0x8323947C)
	// 83239410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323941C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239420: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239424: 38EAC988  addi r7, r10, -0x3678
	ctx.r[7].s64 = ctx.r[10].s64 + -13944;
	// 83239428: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323942C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83239430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239438: 3889C9E8  addi r4, r9, -0x3618
	ctx.r[4].s64 = ctx.r[9].s64 + -13848;
	// 8323943C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239440: 38689674  addi r3, r8, -0x698c
	ctx.r[3].s64 = ctx.r[8].s64 + -27020;
	// 83239444: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323944C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239450: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239454: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239458: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323945C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239460: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83239464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239468: 4BC6ED61  bl 0x82ea81c8
	ctx.lr = 0x8323946C;
	sub_82EA81C8(ctx, base);
	// 8323946C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239480 size=108
    let mut pc: u32 = 0x83239480;
    'dispatch: loop {
        match pc {
            0x83239480 => {
    //   block [0x83239480..0x832394EC)
	// 83239480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323948C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239490: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239494: 38EACA20  addi r7, r10, -0x35e0
	ctx.r[7].s64 = ctx.r[10].s64 + -13792;
	// 83239498: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323949C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 832394A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832394A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832394A8: 3889CAA8  addi r4, r9, -0x3558
	ctx.r[4].s64 = ctx.r[9].s64 + -13656;
	// 832394AC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832394B0: 386896A4  addi r3, r8, -0x695c
	ctx.r[3].s64 = ctx.r[8].s64 + -26972;
	// 832394B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832394B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832394BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832394C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832394C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832394C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832394CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832394D0: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832394D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832394D8: 4BC6ECF1  bl 0x82ea81c8
	ctx.lr = 0x832394DC;
	sub_82EA81C8(ctx, base);
	// 832394DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832394E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832394E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832394E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832394F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832394F0 size=120
    let mut pc: u32 = 0x832394F0;
    'dispatch: loop {
        match pc {
            0x832394F0 => {
    //   block [0x832394F0..0x83239568)
	// 832394F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832394F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832394F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832394FC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239500: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239504: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239508: 38CACA50  addi r6, r10, -0x35b0
	ctx.r[6].s64 = ctx.r[10].s64 + -13744;
	// 8323950C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239510: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239514: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323951C: 38A99344  addi r5, r9, -0x6cbc
	ctx.r[5].s64 = ctx.r[9].s64 + -27836;
	// 83239520: 3888CABC  addi r4, r8, -0x3544
	ctx.r[4].s64 = ctx.r[8].s64 + -13636;
	// 83239524: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239528: 386796D4  addi r3, r7, -0x692c
	ctx.r[3].s64 = ctx.r[7].s64 + -26924;
	// 8323952C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239530: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83239534: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323953C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239540: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239548: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323954C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83239550: 4BC6EC79  bl 0x82ea81c8
	ctx.lr = 0x83239554;
	sub_82EA81C8(ctx, base);
	// 83239554: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323955C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239568 size=28
    let mut pc: u32 = 0x83239568;
    'dispatch: loop {
        match pc {
            0x83239568 => {
    //   block [0x83239568..0x83239584)
	// 83239568: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323956C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239570: 392AF838  addi r9, r10, -0x7c8
	ctx.r[9].s64 = ctx.r[10].s64 + -1992;
	// 83239574: 816BF820  lwz r11, -0x7e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2016 as u32) ) } as u64;
	// 83239578: 91690068  stw r11, 0x68(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8323957C: 91690080  stw r11, 0x80(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83239580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239588 size=116
    let mut pc: u32 = 0x83239588;
    'dispatch: loop {
        match pc {
            0x83239588 => {
    //   block [0x83239588..0x832395FC)
	// 83239588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323958C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239594: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 83239598: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323959C: 38A9F838  addi r5, r9, -0x7c8
	ctx.r[5].s64 = ctx.r[9].s64 + -1992;
	// 832395A0: 38CACC80  addi r6, r10, -0x3380
	ctx.r[6].s64 = ctx.r[10].s64 + -13184;
	// 832395A4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832395A8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 832395AC: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832395B0: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 832395B4: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 832395B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832395BC: 3888CCE0  addi r4, r8, -0x3320
	ctx.r[4].s64 = ctx.r[8].s64 + -13088;
	// 832395C0: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832395C4: 38679704  addi r3, r7, -0x68fc
	ctx.r[3].s64 = ctx.r[7].s64 + -26876;
	// 832395C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832395CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832395D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832395D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832395D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832395DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832395E0: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 832395E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832395E8: 4BC6EBE1  bl 0x82ea81c8
	ctx.lr = 0x832395EC;
	sub_82EA81C8(ctx, base);
	// 832395EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832395F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832395F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832395F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239600 size=92
    let mut pc: u32 = 0x83239600;
    'dispatch: loop {
        match pc {
            0x83239600 => {
    //   block [0x83239600..0x8323965C)
	// 83239600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239608: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323960C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83239610: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83239614: 4BCF0BD5  bl 0x82f2a1e8
	ctx.lr = 0x83239618;
	sub_82F2A1E8(ctx, base);
	// 83239618: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323961C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83239620: 39099734  addi r8, r9, -0x68cc
	ctx.r[8].s64 = ctx.r[9].s64 + -26828;
	// 83239624: 396BCDB8  addi r11, r11, -0x3248
	ctx.r[11].s64 = ctx.r[11].s64 + -12872;
	// 83239628: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323962C: 91699734  stw r11, -0x68cc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26828 as u32), ctx.r[11].u32 ) };
	// 83239630: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 83239634: 394ABD98  addi r10, r10, -0x4268
	ctx.r[10].s64 = ctx.r[10].s64 + -17000;
	// 83239638: 3929BDB0  addi r9, r9, -0x4250
	ctx.r[9].s64 = ctx.r[9].s64 + -16976;
	// 8323963C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83239640: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83239644: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83239648: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323964C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83239650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239660 size=24
    let mut pc: u32 = 0x83239660;
    'dispatch: loop {
        match pc {
            0x83239660 => {
    //   block [0x83239660..0x83239678)
	// 83239660: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83239664: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239668: 392AF9A0  addi r9, r10, -0x660
	ctx.r[9].s64 = ctx.r[10].s64 + -1632;
	// 8323966C: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 83239670: 91690068  stw r11, 0x68(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83239674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239678 size=120
    let mut pc: u32 = 0x83239678;
    'dispatch: loop {
        match pc {
            0x83239678 => {
    //   block [0x83239678..0x832396F0)
	// 83239678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239684: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83239688: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323968C: 392BCC58  addi r9, r11, -0x33a8
	ctx.r[9].s64 = ctx.r[11].s64 + -13224;
	// 83239690: 388AF9A0  addi r4, r10, -0x660
	ctx.r[4].s64 = ctx.r[10].s64 + -1632;
	// 83239694: 38A90068  addi r5, r9, 0x68
	ctx.r[5].s64 = ctx.r[9].s64 + 104;
	// 83239698: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 8323969C: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 832396A0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832396A4: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 832396A8: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 832396AC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 832396B0: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 832396B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832396B8: 38A89314  addi r5, r8, -0x6cec
	ctx.r[5].s64 = ctx.r[8].s64 + -27884;
	// 832396BC: 3887CDB8  addi r4, r7, -0x3248
	ctx.r[4].s64 = ctx.r[7].s64 + -12872;
	// 832396C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832396C4: 38669744  addi r3, r6, -0x68bc
	ctx.r[3].s64 = ctx.r[6].s64 + -26812;
	// 832396C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832396CC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832396D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832396D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832396D8: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832396DC: 4BC6EAED  bl 0x82ea81c8
	ctx.lr = 0x832396E0;
	sub_82EA81C8(ctx, base);
	// 832396E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832396E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832396E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832396EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832396F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832396F0 size=108
    let mut pc: u32 = 0x832396F0;
    'dispatch: loop {
        match pc {
            0x832396F0 => {
    //   block [0x832396F0..0x8323975C)
	// 832396F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832396F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832396F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832396FC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239700: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239704: 38EACDC8  addi r7, r10, -0x3238
	ctx.r[7].s64 = ctx.r[10].s64 + -12856;
	// 83239708: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323970C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83239710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239718: 3889CDE0  addi r4, r9, -0x3220
	ctx.r[4].s64 = ctx.r[9].s64 + -12832;
	// 8323971C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239720: 38689774  addi r3, r8, -0x688c
	ctx.r[3].s64 = ctx.r[8].s64 + -26764;
	// 83239724: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323972C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239730: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239734: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323973C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239740: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83239744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239748: 4BC6EA81  bl 0x82ea81c8
	ctx.lr = 0x8323974C;
	sub_82EA81C8(ctx, base);
	// 8323974C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239760 size=108
    let mut pc: u32 = 0x83239760;
    'dispatch: loop {
        match pc {
            0x83239760 => {
    //   block [0x83239760..0x832397CC)
	// 83239760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323976C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239770: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239774: 38EACE50  addi r7, r10, -0x31b0
	ctx.r[7].s64 = ctx.r[10].s64 + -12720;
	// 83239778: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323977C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83239780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239788: 3889CF10  addi r4, r9, -0x30f0
	ctx.r[4].s64 = ctx.r[9].s64 + -12528;
	// 8323978C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239790: 386897A4  addi r3, r8, -0x685c
	ctx.r[3].s64 = ctx.r[8].s64 + -26716;
	// 83239794: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323979C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832397A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832397A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832397A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832397AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832397B0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832397B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832397B8: 4BC6EA11  bl 0x82ea81c8
	ctx.lr = 0x832397BC;
	sub_82EA81C8(ctx, base);
	// 832397BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832397C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832397C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832397C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832397D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832397D0 size=92
    let mut pc: u32 = 0x832397D0;
    'dispatch: loop {
        match pc {
            0x832397D0 => {
    //   block [0x832397D0..0x8323982C)
	// 832397D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832397D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832397D8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832397DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832397E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832397E4: 4BCF11F5  bl 0x82f2a9d8
	ctx.lr = 0x832397E8;
	sub_82F2A9D8(ctx, base);
	// 832397E8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832397EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 832397F0: 390997D4  addi r8, r9, -0x682c
	ctx.r[8].s64 = ctx.r[9].s64 + -26668;
	// 832397F4: 396BCF28  addi r11, r11, -0x30d8
	ctx.r[11].s64 = ctx.r[11].s64 + -12504;
	// 832397F8: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 832397FC: 916997D4  stw r11, -0x682c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26668 as u32), ctx.r[11].u32 ) };
	// 83239800: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 83239804: 394ABE20  addi r10, r10, -0x41e0
	ctx.r[10].s64 = ctx.r[10].s64 + -16864;
	// 83239808: 3929BE08  addi r9, r9, -0x41f8
	ctx.r[9].s64 = ctx.r[9].s64 + -16888;
	// 8323980C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83239810: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83239814: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83239818: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323981C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83239820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239830 size=120
    let mut pc: u32 = 0x83239830;
    'dispatch: loop {
        match pc {
            0x83239830 => {
    //   block [0x83239830..0x832398A8)
	// 83239830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323983C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239840: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239844: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239848: 38CACEB0  addi r6, r10, -0x3150
	ctx.r[6].s64 = ctx.r[10].s64 + -12624;
	// 8323984C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239850: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239854: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323985C: 38A99314  addi r5, r9, -0x6cec
	ctx.r[5].s64 = ctx.r[9].s64 + -27884;
	// 83239860: 3888CF28  addi r4, r8, -0x30d8
	ctx.r[4].s64 = ctx.r[8].s64 + -12504;
	// 83239864: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239868: 386797E4  addi r3, r7, -0x681c
	ctx.r[3].s64 = ctx.r[7].s64 + -26652;
	// 8323986C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239870: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 83239874: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323987C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239880: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239888: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323988C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83239890: 4BC6E939  bl 0x82ea81c8
	ctx.lr = 0x83239894;
	sub_82EA81C8(ctx, base);
	// 83239894: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323989C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832398A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832398A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832398A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832398A8 size=36
    let mut pc: u32 = 0x832398A8;
    'dispatch: loop {
        match pc {
            0x832398A8 => {
    //   block [0x832398A8..0x832398CC)
	// 832398A8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832398AC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832398B0: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 832398B4: 3909FAA8  addi r8, r9, -0x558
	ctx.r[8].s64 = ctx.r[9].s64 + -1368;
	// 832398B8: 816BFA70  lwz r11, -0x590(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1424 as u32) ) } as u64;
	// 832398BC: 814AFA68  lwz r10, -0x598(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-1432 as u32) ) } as u64;
	// 832398C0: 91680008  stw r11, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832398C4: 91480020  stw r10, 0x20(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 832398C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832398D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832398D0 size=108
    let mut pc: u32 = 0x832398D0;
    'dispatch: loop {
        match pc {
            0x832398D0 => {
    //   block [0x832398D0..0x8323993C)
	// 832398D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832398D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832398D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832398DC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832398E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 832398E4: 38EAFAA8  addi r7, r10, -0x558
	ctx.r[7].s64 = ctx.r[10].s64 + -1368;
	// 832398E8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 832398EC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 832398F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832398F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832398F8: 3889D140  addi r4, r9, -0x2ec0
	ctx.r[4].s64 = ctx.r[9].s64 + -11968;
	// 832398FC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239900: 38689814  addi r3, r8, -0x67ec
	ctx.r[3].s64 = ctx.r[8].s64 + -26604;
	// 83239904: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323990C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239910: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239914: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239918: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323991C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239920: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83239924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239928: 4BC6E8A1  bl 0x82ea81c8
	ctx.lr = 0x8323992C;
	sub_82EA81C8(ctx, base);
	// 8323992C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239940 size=24
    let mut pc: u32 = 0x83239940;
    'dispatch: loop {
        match pc {
            0x83239940 => {
    //   block [0x83239940..0x83239958)
	// 83239940: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83239944: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239948: 392AFB50  addi r9, r10, -0x4b0
	ctx.r[9].s64 = ctx.r[10].s64 + -1200;
	// 8323994C: 816BFA68  lwz r11, -0x598(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1432 as u32) ) } as u64;
	// 83239950: 916900B0  stw r11, 0xb0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83239954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239958 size=120
    let mut pc: u32 = 0x83239958;
    'dispatch: loop {
        match pc {
            0x83239958 => {
    //   block [0x83239958..0x832399D0)
	// 83239958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323995C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239964: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239968: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 8323996C: 38AAD06C  addi r5, r10, -0x2f94
	ctx.r[5].s64 = ctx.r[10].s64 + -12180;
	// 83239970: 3889FB50  addi r4, r9, -0x4b0
	ctx.r[4].s64 = ctx.r[9].s64 + -1200;
	// 83239974: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239978: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 8323997C: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83239980: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83239984: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83239988: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8323998C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239990: 38A89814  addi r5, r8, -0x67ec
	ctx.r[5].s64 = ctx.r[8].s64 + -26604;
	// 83239994: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83239998: 3887D15C  addi r4, r7, -0x2ea4
	ctx.r[4].s64 = ctx.r[7].s64 + -11940;
	// 8323999C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832399A0: 38669844  addi r3, r6, -0x67bc
	ctx.r[3].s64 = ctx.r[6].s64 + -26556;
	// 832399A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832399A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832399AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832399B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832399B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832399B8: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832399BC: 4BC6E80D  bl 0x82ea81c8
	ctx.lr = 0x832399C0;
	sub_82EA81C8(ctx, base);
	// 832399C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832399C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832399C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832399CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832399D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832399D0 size=120
    let mut pc: u32 = 0x832399D0;
    'dispatch: loop {
        match pc {
            0x832399D0 => {
    //   block [0x832399D0..0x83239A48)
	// 832399D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832399D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832399D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832399DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832399E0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832399E4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 832399E8: 38CAD098  addi r6, r10, -0x2f68
	ctx.r[6].s64 = ctx.r[10].s64 + -12136;
	// 832399EC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 832399F0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 832399F4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832399F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832399FC: 38A99814  addi r5, r9, -0x67ec
	ctx.r[5].s64 = ctx.r[9].s64 + -26604;
	// 83239A00: 3888D184  addi r4, r8, -0x2e7c
	ctx.r[4].s64 = ctx.r[8].s64 + -11900;
	// 83239A04: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239A08: 38679874  addi r3, r7, -0x678c
	ctx.r[3].s64 = ctx.r[7].s64 + -26508;
	// 83239A0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239A10: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 83239A14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239A20: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239A28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239A2C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83239A30: 4BC6E799  bl 0x82ea81c8
	ctx.lr = 0x83239A34;
	sub_82EA81C8(ctx, base);
	// 83239A34: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239A48 size=92
    let mut pc: u32 = 0x83239A48;
    'dispatch: loop {
        match pc {
            0x83239A48 => {
    //   block [0x83239A48..0x83239AA4)
	// 83239A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239A50: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239A54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83239A58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83239A5C: 4BCDF53D  bl 0x82f18f98
	ctx.lr = 0x83239A60;
	sub_82F18F98(ctx, base);
	// 83239A60: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239A64: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83239A68: 390998A4  addi r8, r9, -0x675c
	ctx.r[8].s64 = ctx.r[9].s64 + -26460;
	// 83239A6C: 396BD1A8  addi r11, r11, -0x2e58
	ctx.r[11].s64 = ctx.r[11].s64 + -11864;
	// 83239A70: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 83239A74: 916998A4  stw r11, -0x675c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26460 as u32), ctx.r[11].u32 ) };
	// 83239A78: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 83239A7C: 394ABE78  addi r10, r10, -0x4188
	ctx.r[10].s64 = ctx.r[10].s64 + -16776;
	// 83239A80: 3929BE90  addi r9, r9, -0x4170
	ctx.r[9].s64 = ctx.r[9].s64 + -16752;
	// 83239A84: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83239A88: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83239A8C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83239A90: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83239A94: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 83239A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239AA8 size=24
    let mut pc: u32 = 0x83239AA8;
    'dispatch: loop {
        match pc {
            0x83239AA8 => {
    //   block [0x83239AA8..0x83239AC0)
	// 83239AA8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83239AAC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239AB0: 392AFC40  addi r9, r10, -0x3c0
	ctx.r[9].s64 = ctx.r[10].s64 + -960;
	// 83239AB4: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 83239AB8: 916900B0  stw r11, 0xb0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83239ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239AC0 size=120
    let mut pc: u32 = 0x83239AC0;
    'dispatch: loop {
        match pc {
            0x83239AC0 => {
    //   block [0x83239AC0..0x83239B38)
	// 83239AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239ACC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83239AD0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239AD4: 392BD030  addi r9, r11, -0x2fd0
	ctx.r[9].s64 = ctx.r[11].s64 + -12240;
	// 83239AD8: 388AFC40  addi r4, r10, -0x3c0
	ctx.r[4].s64 = ctx.r[10].s64 + -960;
	// 83239ADC: 38A900E0  addi r5, r9, 0xe0
	ctx.r[5].s64 = ctx.r[9].s64 + 224;
	// 83239AE0: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 83239AE4: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83239AE8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239AEC: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83239AF0: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83239AF4: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83239AF8: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83239AFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239B00: 38A89314  addi r5, r8, -0x6cec
	ctx.r[5].s64 = ctx.r[8].s64 + -27884;
	// 83239B04: 3887D1A8  addi r4, r7, -0x2e58
	ctx.r[4].s64 = ctx.r[7].s64 + -11864;
	// 83239B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239B0C: 386698B4  addi r3, r6, -0x674c
	ctx.r[3].s64 = ctx.r[6].s64 + -26444;
	// 83239B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239B14: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83239B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239B20: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 83239B24: 4BC6E6A5  bl 0x82ea81c8
	ctx.lr = 0x83239B28;
	sub_82EA81C8(ctx, base);
	// 83239B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239B38 size=108
    let mut pc: u32 = 0x83239B38;
    'dispatch: loop {
        match pc {
            0x83239B38 => {
    //   block [0x83239B38..0x83239BA4)
	// 83239B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239B44: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239B48: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239B4C: 38EAD220  addi r7, r10, -0x2de0
	ctx.r[7].s64 = ctx.r[10].s64 + -11744;
	// 83239B50: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239B54: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83239B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239B5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239B60: 3889D2F8  addi r4, r9, -0x2d08
	ctx.r[4].s64 = ctx.r[9].s64 + -11528;
	// 83239B64: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239B68: 386898E4  addi r3, r8, -0x671c
	ctx.r[3].s64 = ctx.r[8].s64 + -26396;
	// 83239B6C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239B74: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239B78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239B7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239B80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239B88: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83239B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239B90: 4BC6E639  bl 0x82ea81c8
	ctx.lr = 0x83239B94;
	sub_82EA81C8(ctx, base);
	// 83239B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239BA8 size=120
    let mut pc: u32 = 0x83239BA8;
    'dispatch: loop {
        match pc {
            0x83239BA8 => {
    //   block [0x83239BA8..0x83239C20)
	// 83239BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83239BB4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239BB8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239BBC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239BC0: 38CAD268  addi r6, r10, -0x2d98
	ctx.r[6].s64 = ctx.r[10].s64 + -11672;
	// 83239BC4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239BC8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239BCC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239BD4: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 83239BD8: 3888D31C  addi r4, r8, -0x2ce4
	ctx.r[4].s64 = ctx.r[8].s64 + -11492;
	// 83239BDC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239BE0: 38679914  addi r3, r7, -0x66ec
	ctx.r[3].s64 = ctx.r[7].s64 + -26348;
	// 83239BE4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239BE8: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 83239BEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239BF8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239C04: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 83239C08: 4BC6E5C1  bl 0x82ea81c8
	ctx.lr = 0x83239C0C;
	sub_82EA81C8(ctx, base);
	// 83239C0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239C18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239C20 size=108
    let mut pc: u32 = 0x83239C20;
    'dispatch: loop {
        match pc {
            0x83239C20 => {
    //   block [0x83239C20..0x83239C8C)
	// 83239C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239C2C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239C30: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239C34: 38EAD458  addi r7, r10, -0x2ba8
	ctx.r[7].s64 = ctx.r[10].s64 + -11176;
	// 83239C38: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239C3C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83239C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239C44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239C48: 3889D5A8  addi r4, r9, -0x2a58
	ctx.r[4].s64 = ctx.r[9].s64 + -10840;
	// 83239C4C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239C50: 38689944  addi r3, r8, -0x66bc
	ctx.r[3].s64 = ctx.r[8].s64 + -26300;
	// 83239C54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239C5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239C60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239C64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239C68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239C70: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 83239C74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239C78: 4BC6E551  bl 0x82ea81c8
	ctx.lr = 0x83239C7C;
	sub_82EA81C8(ctx, base);
	// 83239C7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239C90 size=116
    let mut pc: u32 = 0x83239C90;
    'dispatch: loop {
        match pc {
            0x83239C90 => {
    //   block [0x83239C90..0x83239D04)
	// 83239C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239C9C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239CA0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239CA4: 38AAD518  addi r5, r10, -0x2ae8
	ctx.r[5].s64 = ctx.r[10].s64 + -10984;
	// 83239CA8: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83239CAC: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83239CB0: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83239CB4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 83239CB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239CBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239CC0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83239CC4: 38A899A4  addi r5, r8, -0x665c
	ctx.r[5].s64 = ctx.r[8].s64 + -26204;
	// 83239CC8: 3887D5C8  addi r4, r7, -0x2a38
	ctx.r[4].s64 = ctx.r[7].s64 + -10808;
	// 83239CCC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239CD0: 38669974  addi r3, r6, -0x668c
	ctx.r[3].s64 = ctx.r[6].s64 + -26252;
	// 83239CD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239CD8: 3929D440  addi r9, r9, -0x2bc0
	ctx.r[9].s64 = ctx.r[9].s64 + -11200;
	// 83239CDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239CE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83239CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239CE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239CEC: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 83239CF0: 4BC6E4D9  bl 0x82ea81c8
	ctx.lr = 0x83239CF4;
	sub_82EA81C8(ctx, base);
	// 83239CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239D08 size=108
    let mut pc: u32 = 0x83239D08;
    'dispatch: loop {
        match pc {
            0x83239D08 => {
    //   block [0x83239D08..0x83239D74)
	// 83239D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239D14: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239D18: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 83239D1C: 38EAD5E8  addi r7, r10, -0x2a18
	ctx.r[7].s64 = ctx.r[10].s64 + -10776;
	// 83239D20: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239D24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83239D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83239D2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239D30: 3889D648  addi r4, r9, -0x29b8
	ctx.r[4].s64 = ctx.r[9].s64 + -10680;
	// 83239D34: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83239D38: 386899A4  addi r3, r8, -0x665c
	ctx.r[3].s64 = ctx.r[8].s64 + -26204;
	// 83239D3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239D44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239D48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239D4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239D50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239D58: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83239D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83239D60: 4BC6E469  bl 0x82ea81c8
	ctx.lr = 0x83239D64;
	sub_82EA81C8(ctx, base);
	// 83239D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83239D78 size=24
    let mut pc: u32 = 0x83239D78;
    'dispatch: loop {
        match pc {
            0x83239D78 => {
    //   block [0x83239D78..0x83239D90)
	// 83239D78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83239D7C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83239D80: 392AFDB0  addi r9, r10, -0x250
	ctx.r[9].s64 = ctx.r[10].s64 + -592;
	// 83239D84: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 83239D88: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83239D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239D90 size=120
    let mut pc: u32 = 0x83239D90;
    'dispatch: loop {
        match pc {
            0x83239D90 => {
    //   block [0x83239D90..0x83239E08)
	// 83239D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239D9C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239DA0: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 83239DA4: 38AAD654  addi r5, r10, -0x29ac
	ctx.r[5].s64 = ctx.r[10].s64 + -10668;
	// 83239DA8: 3889FDB0  addi r4, r9, -0x250
	ctx.r[4].s64 = ctx.r[9].s64 + -592;
	// 83239DAC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 83239DB0: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83239DB4: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83239DB8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83239DBC: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 83239DC0: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 83239DC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239DC8: 38A89464  addi r5, r8, -0x6b9c
	ctx.r[5].s64 = ctx.r[8].s64 + -27548;
	// 83239DCC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83239DD0: 3887D674  addi r4, r7, -0x298c
	ctx.r[4].s64 = ctx.r[7].s64 + -10636;
	// 83239DD4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239DD8: 386699D4  addi r3, r6, -0x662c
	ctx.r[3].s64 = ctx.r[6].s64 + -26156;
	// 83239DDC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239DF0: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 83239DF4: 4BC6E3D5  bl 0x82ea81c8
	ctx.lr = 0x83239DF8;
	sub_82EA81C8(ctx, base);
	// 83239DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83239DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239E08 size=120
    let mut pc: u32 = 0x83239E08;
    'dispatch: loop {
        match pc {
            0x83239E08 => {
    //   block [0x83239E08..0x83239E80)
	// 83239E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239E10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83239E14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239E18: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239E1C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239E20: 38CAD6B8  addi r6, r10, -0x2948
	ctx.r[6].s64 = ctx.r[10].s64 + -10568;
	// 83239E24: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239E28: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239E2C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239E34: 38A99314  addi r5, r9, -0x6cec
	ctx.r[5].s64 = ctx.r[9].s64 + -27884;
	// 83239E38: 3888D74C  addi r4, r8, -0x28b4
	ctx.r[4].s64 = ctx.r[8].s64 + -10420;
	// 83239E3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239E40: 38679A04  addi r3, r7, -0x65fc
	ctx.r[3].s64 = ctx.r[7].s64 + -26108;
	// 83239E44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239E48: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83239E4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239E58: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239E60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239E64: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 83239E68: 4BC6E361  bl 0x82ea81c8
	ctx.lr = 0x83239E6C;
	sub_82EA81C8(ctx, base);
	// 83239E6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239E80 size=120
    let mut pc: u32 = 0x83239E80;
    'dispatch: loop {
        match pc {
            0x83239E80 => {
    //   block [0x83239E80..0x83239EF8)
	// 83239E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83239E8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239E90: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239E94: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239E98: 38CAD798  addi r6, r10, -0x2868
	ctx.r[6].s64 = ctx.r[10].s64 + -10344;
	// 83239E9C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239EA0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239EA4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239EAC: 38A99524  addi r5, r9, -0x6adc
	ctx.r[5].s64 = ctx.r[9].s64 + -27356;
	// 83239EB0: 3888D814  addi r4, r8, -0x27ec
	ctx.r[4].s64 = ctx.r[8].s64 + -10220;
	// 83239EB4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239EB8: 38679A34  addi r3, r7, -0x65cc
	ctx.r[3].s64 = ctx.r[7].s64 + -26060;
	// 83239EBC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239EC0: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83239EC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239ED0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239ED8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239EDC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83239EE0: 4BC6E2E9  bl 0x82ea81c8
	ctx.lr = 0x83239EE4;
	sub_82EA81C8(ctx, base);
	// 83239EE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239EF8 size=120
    let mut pc: u32 = 0x83239EF8;
    'dispatch: loop {
        match pc {
            0x83239EF8 => {
    //   block [0x83239EF8..0x83239F70)
	// 83239EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83239F04: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239F08: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239F0C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239F10: 38CAD838  addi r6, r10, -0x27c8
	ctx.r[6].s64 = ctx.r[10].s64 + -10184;
	// 83239F14: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239F18: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239F1C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239F24: 38A99344  addi r5, r9, -0x6cbc
	ctx.r[5].s64 = ctx.r[9].s64 + -27836;
	// 83239F28: 3888D8A8  addi r4, r8, -0x2758
	ctx.r[4].s64 = ctx.r[8].s64 + -10072;
	// 83239F2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239F30: 38679A64  addi r3, r7, -0x659c
	ctx.r[3].s64 = ctx.r[7].s64 + -26012;
	// 83239F34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239F38: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83239F3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239F48: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239F50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239F54: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83239F58: 4BC6E271  bl 0x82ea81c8
	ctx.lr = 0x83239F5C;
	sub_82EA81C8(ctx, base);
	// 83239F5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239F70 size=120
    let mut pc: u32 = 0x83239F70;
    'dispatch: loop {
        match pc {
            0x83239F70 => {
    //   block [0x83239F70..0x83239FE8)
	// 83239F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83239F7C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239F80: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83239F84: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 83239F88: 38CAD8D4  addi r6, r10, -0x272c
	ctx.r[6].s64 = ctx.r[10].s64 + -10028;
	// 83239F8C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 83239F90: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83239F94: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83239F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83239F9C: 38A99374  addi r5, r9, -0x6c8c
	ctx.r[5].s64 = ctx.r[9].s64 + -27788;
	// 83239FA0: 3888D990  addi r4, r8, -0x2670
	ctx.r[4].s64 = ctx.r[8].s64 + -9840;
	// 83239FA4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83239FA8: 38679A94  addi r3, r7, -0x656c
	ctx.r[3].s64 = ctx.r[7].s64 + -25964;
	// 83239FAC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83239FB0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83239FB4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83239FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83239FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83239FC0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83239FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83239FC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83239FCC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83239FD0: 4BC6E1F9  bl 0x82ea81c8
	ctx.lr = 0x83239FD4;
	sub_82EA81C8(ctx, base);
	// 83239FD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83239FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83239FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83239FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83239FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83239FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83239FE8 size=92
    let mut pc: u32 = 0x83239FE8;
    'dispatch: loop {
        match pc {
            0x83239FE8 => {
    //   block [0x83239FE8..0x8323A044)
	// 83239FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83239FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83239FF0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83239FF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83239FF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83239FFC: 4BCF4C7D  bl 0x82f2ec78
	ctx.lr = 0x8323A000;
	sub_82F2EC78(ctx, base);
	// 8323A000: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A004: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A008: 39099AC4  addi r8, r9, -0x653c
	ctx.r[8].s64 = ctx.r[9].s64 + -25916;
	// 8323A00C: 396BDA70  addi r11, r11, -0x2590
	ctx.r[11].s64 = ctx.r[11].s64 + -9616;
	// 8323A010: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A014: 91699AC4  stw r11, -0x653c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25916 as u32), ctx.r[11].u32 ) };
	// 8323A018: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A01C: 394AC6F8  addi r10, r10, -0x3908
	ctx.r[10].s64 = ctx.r[10].s64 + -14600;
	// 8323A020: 3929C710  addi r9, r9, -0x38f0
	ctx.r[9].s64 = ctx.r[9].s64 + -14576;
	// 8323A024: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A028: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A02C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A030: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A034: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8323A038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A048 size=120
    let mut pc: u32 = 0x8323A048;
    'dispatch: loop {
        match pc {
            0x8323A048 => {
    //   block [0x8323A048..0x8323A0C0)
	// 8323A048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A054: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A058: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A05C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323A060: 38CAD9C8  addi r6, r10, -0x2638
	ctx.r[6].s64 = ctx.r[10].s64 + -9784;
	// 8323A064: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A068: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A06C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A074: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323A078: 3888DA70  addi r4, r8, -0x2590
	ctx.r[4].s64 = ctx.r[8].s64 + -9616;
	// 8323A07C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A080: 38679AD4  addi r3, r7, -0x652c
	ctx.r[3].s64 = ctx.r[7].s64 + -25900;
	// 8323A084: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A088: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 8323A08C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A098: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A0A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A0A4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323A0A8: 4BC6E121  bl 0x82ea81c8
	ctx.lr = 0x8323A0AC;
	sub_82EA81C8(ctx, base);
	// 8323A0AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A0B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A0B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A0B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A0C0 size=92
    let mut pc: u32 = 0x8323A0C0;
    'dispatch: loop {
        match pc {
            0x8323A0C0 => {
    //   block [0x8323A0C0..0x8323A11C)
	// 8323A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A0C8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A0CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A0D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A0D4: 4BCF4BD5  bl 0x82f2eca8
	ctx.lr = 0x8323A0D8;
	sub_82F2ECA8(ctx, base);
	// 8323A0D8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A0DC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A0E0: 39099B04  addi r8, r9, -0x64fc
	ctx.r[8].s64 = ctx.r[9].s64 + -25852;
	// 8323A0E4: 396BDA94  addi r11, r11, -0x256c
	ctx.r[11].s64 = ctx.r[11].s64 + -9580;
	// 8323A0E8: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A0EC: 91699B04  stw r11, -0x64fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25852 as u32), ctx.r[11].u32 ) };
	// 8323A0F0: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A0F4: 394AC758  addi r10, r10, -0x38a8
	ctx.r[10].s64 = ctx.r[10].s64 + -14504;
	// 8323A0F8: 3929C770  addi r9, r9, -0x3890
	ctx.r[9].s64 = ctx.r[9].s64 + -14480;
	// 8323A0FC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A100: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A104: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A108: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A10C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8323A110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A120 size=120
    let mut pc: u32 = 0x8323A120;
    'dispatch: loop {
        match pc {
            0x8323A120 => {
    //   block [0x8323A120..0x8323A198)
	// 8323A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A12C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A130: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A134: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A138: 38CADA58  addi r6, r10, -0x25a8
	ctx.r[6].s64 = ctx.r[10].s64 + -9640;
	// 8323A13C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A140: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A144: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A14C: 38A99744  addi r5, r9, -0x68bc
	ctx.r[5].s64 = ctx.r[9].s64 + -26812;
	// 8323A150: 3888DA94  addi r4, r8, -0x256c
	ctx.r[4].s64 = ctx.r[8].s64 + -9580;
	// 8323A154: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A158: 38679B14  addi r3, r7, -0x64ec
	ctx.r[3].s64 = ctx.r[7].s64 + -25836;
	// 8323A15C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A160: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323A164: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A170: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A178: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A17C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8323A180: 4BC6E049  bl 0x82ea81c8
	ctx.lr = 0x8323A184;
	sub_82EA81C8(ctx, base);
	// 8323A184: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A198 size=92
    let mut pc: u32 = 0x8323A198;
    'dispatch: loop {
        match pc {
            0x8323A198 => {
    //   block [0x8323A198..0x8323A1F4)
	// 8323A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A1A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A1A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A1A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A1AC: 4BCF5975  bl 0x82f2fb20
	ctx.lr = 0x8323A1B0;
	sub_82F2FB20(ctx, base);
	// 8323A1B0: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A1B4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A1B8: 39099B44  addi r8, r9, -0x64bc
	ctx.r[8].s64 = ctx.r[9].s64 + -25788;
	// 8323A1BC: 396BDBF0  addi r11, r11, -0x2410
	ctx.r[11].s64 = ctx.r[11].s64 + -9232;
	// 8323A1C0: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A1C4: 91699B44  stw r11, -0x64bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25788 as u32), ctx.r[11].u32 ) };
	// 8323A1C8: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A1CC: 394AC7B8  addi r10, r10, -0x3848
	ctx.r[10].s64 = ctx.r[10].s64 + -14408;
	// 8323A1D0: 3929C7D0  addi r9, r9, -0x3830
	ctx.r[9].s64 = ctx.r[9].s64 + -14384;
	// 8323A1D4: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A1D8: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A1DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A1E0: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A1E4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8323A1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A1F8 size=120
    let mut pc: u32 = 0x8323A1F8;
    'dispatch: loop {
        match pc {
            0x8323A1F8 => {
    //   block [0x8323A1F8..0x8323A270)
	// 8323A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A204: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A208: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A20C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323A210: 38CADAD0  addi r6, r10, -0x2530
	ctx.r[6].s64 = ctx.r[10].s64 + -9520;
	// 8323A214: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A218: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A21C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A224: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323A228: 3888DBF0  addi r4, r8, -0x2410
	ctx.r[4].s64 = ctx.r[8].s64 + -9232;
	// 8323A22C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A230: 38679B54  addi r3, r7, -0x64ac
	ctx.r[3].s64 = ctx.r[7].s64 + -25772;
	// 8323A234: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A238: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 8323A23C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A248: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A250: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A254: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323A258: 4BC6DF71  bl 0x82ea81c8
	ctx.lr = 0x8323A25C;
	sub_82EA81C8(ctx, base);
	// 8323A25C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A270 size=92
    let mut pc: u32 = 0x8323A270;
    'dispatch: loop {
        match pc {
            0x8323A270 => {
    //   block [0x8323A270..0x8323A2CC)
	// 8323A270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A278: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A27C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A284: 4BCF561D  bl 0x82f2f8a0
	ctx.lr = 0x8323A288;
	sub_82F2F8A0(ctx, base);
	// 8323A288: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A28C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A290: 39099B84  addi r8, r9, -0x647c
	ctx.r[8].s64 = ctx.r[9].s64 + -25724;
	// 8323A294: 396BDC20  addi r11, r11, -0x23e0
	ctx.r[11].s64 = ctx.r[11].s64 + -9184;
	// 8323A298: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A29C: 91699B84  stw r11, -0x647c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25724 as u32), ctx.r[11].u32 ) };
	// 8323A2A0: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A2A4: 394AC818  addi r10, r10, -0x37e8
	ctx.r[10].s64 = ctx.r[10].s64 + -14312;
	// 8323A2A8: 3929C830  addi r9, r9, -0x37d0
	ctx.r[9].s64 = ctx.r[9].s64 + -14288;
	// 8323A2AC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A2B0: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A2B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A2B8: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A2BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8323A2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A2D0 size=120
    let mut pc: u32 = 0x8323A2D0;
    'dispatch: loop {
        match pc {
            0x8323A2D0 => {
    //   block [0x8323A2D0..0x8323A348)
	// 8323A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A2DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A2E0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A2E4: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323A2E8: 38CADB60  addi r6, r10, -0x24a0
	ctx.r[6].s64 = ctx.r[10].s64 + -9376;
	// 8323A2EC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A2F0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A2F4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A2FC: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323A300: 3888DC20  addi r4, r8, -0x23e0
	ctx.r[4].s64 = ctx.r[8].s64 + -9184;
	// 8323A304: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A308: 38679B94  addi r3, r7, -0x646c
	ctx.r[3].s64 = ctx.r[7].s64 + -25708;
	// 8323A30C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A310: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323A314: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A320: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A328: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A32C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 8323A330: 4BC6DE99  bl 0x82ea81c8
	ctx.lr = 0x8323A334;
	sub_82EA81C8(ctx, base);
	// 8323A334: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A348 size=92
    let mut pc: u32 = 0x8323A348;
    'dispatch: loop {
        match pc {
            0x8323A348 => {
    //   block [0x8323A348..0x8323A3A4)
	// 8323A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A350: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A35C: 4BCF57DD  bl 0x82f2fb38
	ctx.lr = 0x8323A360;
	sub_82F2FB38(ctx, base);
	// 8323A360: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A364: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A368: 39099BC4  addi r8, r9, -0x643c
	ctx.r[8].s64 = ctx.r[9].s64 + -25660;
	// 8323A36C: 396BDC50  addi r11, r11, -0x23b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9136;
	// 8323A370: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A374: 91699BC4  stw r11, -0x643c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25660 as u32), ctx.r[11].u32 ) };
	// 8323A378: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A37C: 394AC878  addi r10, r10, -0x3788
	ctx.r[10].s64 = ctx.r[10].s64 + -14216;
	// 8323A380: 3929C890  addi r9, r9, -0x3770
	ctx.r[9].s64 = ctx.r[9].s64 + -14192;
	// 8323A384: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A388: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A38C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A390: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A394: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 8323A398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A3A8 size=120
    let mut pc: u32 = 0x8323A3A8;
    'dispatch: loop {
        match pc {
            0x8323A3A8 => {
    //   block [0x8323A3A8..0x8323A420)
	// 8323A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A3B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A3B8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A3BC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A3C0: 38CADBC0  addi r6, r10, -0x2440
	ctx.r[6].s64 = ctx.r[10].s64 + -9280;
	// 8323A3C4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A3C8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A3CC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A3D4: 38A998B4  addi r5, r9, -0x674c
	ctx.r[5].s64 = ctx.r[9].s64 + -26444;
	// 8323A3D8: 3888DC50  addi r4, r8, -0x23b0
	ctx.r[4].s64 = ctx.r[8].s64 + -9136;
	// 8323A3DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A3E0: 38679BD4  addi r3, r7, -0x642c
	ctx.r[3].s64 = ctx.r[7].s64 + -25644;
	// 8323A3E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A3E8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323A3EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A3F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A400: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A404: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 8323A408: 4BC6DDC1  bl 0x82ea81c8
	ctx.lr = 0x8323A40C;
	sub_82EA81C8(ctx, base);
	// 8323A40C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A420 size=120
    let mut pc: u32 = 0x8323A420;
    'dispatch: loop {
        match pc {
            0x8323A420 => {
    //   block [0x8323A420..0x8323A498)
	// 8323A420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A42C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A430: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A434: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A438: 38CADC6C  addi r6, r10, -0x2394
	ctx.r[6].s64 = ctx.r[10].s64 + -9108;
	// 8323A43C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A440: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A444: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A44C: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 8323A450: 3888DC84  addi r4, r8, -0x237c
	ctx.r[4].s64 = ctx.r[8].s64 + -9084;
	// 8323A454: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A458: 38679C04  addi r3, r7, -0x63fc
	ctx.r[3].s64 = ctx.r[7].s64 + -25596;
	// 8323A45C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A460: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323A464: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A470: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A47C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323A480: 4BC6DD49  bl 0x82ea81c8
	ctx.lr = 0x8323A484;
	sub_82EA81C8(ctx, base);
	// 8323A484: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A498 size=120
    let mut pc: u32 = 0x8323A498;
    'dispatch: loop {
        match pc {
            0x8323A498 => {
    //   block [0x8323A498..0x8323A510)
	// 8323A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A4A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A4A8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A4AC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323A4B0: 38CADCB4  addi r6, r10, -0x234c
	ctx.r[6].s64 = ctx.r[10].s64 + -9036;
	// 8323A4B4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A4B8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A4BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A4C4: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323A4C8: 3888DD14  addi r4, r8, -0x22ec
	ctx.r[4].s64 = ctx.r[8].s64 + -8940;
	// 8323A4CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A4D0: 38679C34  addi r3, r7, -0x63cc
	ctx.r[3].s64 = ctx.r[7].s64 + -25548;
	// 8323A4D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A4D8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323A4DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A4E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A4EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323A4F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A4F4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323A4F8: 4BC6DCD1  bl 0x82ea81c8
	ctx.lr = 0x8323A4FC;
	sub_82EA81C8(ctx, base);
	// 8323A4FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A510 size=120
    let mut pc: u32 = 0x8323A510;
    'dispatch: loop {
        match pc {
            0x8323A510 => {
    //   block [0x8323A510..0x8323A588)
	// 8323A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A51C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A520: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A524: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A528: 38CADD40  addi r6, r10, -0x22c0
	ctx.r[6].s64 = ctx.r[10].s64 + -8896;
	// 8323A52C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A530: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A534: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A53C: 38A99434  addi r5, r9, -0x6bcc
	ctx.r[5].s64 = ctx.r[9].s64 + -27596;
	// 8323A540: 3888DDB8  addi r4, r8, -0x2248
	ctx.r[4].s64 = ctx.r[8].s64 + -8776;
	// 8323A544: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A548: 38679C64  addi r3, r7, -0x639c
	ctx.r[3].s64 = ctx.r[7].s64 + -25500;
	// 8323A54C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A550: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323A554: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A560: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A568: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A56C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323A570: 4BC6DC59  bl 0x82ea81c8
	ctx.lr = 0x8323A574;
	sub_82EA81C8(ctx, base);
	// 8323A574: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A588 size=92
    let mut pc: u32 = 0x8323A588;
    'dispatch: loop {
        match pc {
            0x8323A588 => {
    //   block [0x8323A588..0x8323A5E4)
	// 8323A588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A590: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A598: 4BCE3429  bl 0x82f1d9c0
	ctx.lr = 0x8323A59C;
	sub_82F1D9C0(ctx, base);
	// 8323A59C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A5A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A5A4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 8323A5A8: 396BDDC8  addi r11, r11, -0x2238
	ctx.r[11].s64 = ctx.r[11].s64 + -8760;
	// 8323A5AC: 38C99C94  addi r6, r9, -0x636c
	ctx.r[6].s64 = ctx.r[9].s64 + -25452;
	// 8323A5B0: 3CE082F2  lis r7, -0x7d0e
	ctx.r[7].s64 = -2098069504;
	// 8323A5B4: 91699C94  stw r11, -0x636c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25452 as u32), ctx.r[11].u32 ) };
	// 8323A5B8: 3D0082F2  lis r8, -0x7d0e
	ctx.r[8].s64 = -2098069504;
	// 8323A5BC: 392A4294  addi r9, r10, 0x4294
	ctx.r[9].s64 = ctx.r[10].s64 + 17044;
	// 8323A5C0: 3947CAA8  addi r10, r7, -0x3558
	ctx.r[10].s64 = ctx.r[7].s64 + -13656;
	// 8323A5C4: 3968CA90  addi r11, r8, -0x3570
	ctx.r[11].s64 = ctx.r[8].s64 + -13680;
	// 8323A5C8: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323A5CC: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A5D0: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323A5D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A5E8 size=100
    let mut pc: u32 = 0x8323A5E8;
    'dispatch: loop {
        match pc {
            0x8323A5E8 => {
    //   block [0x8323A5E8..0x8323A64C)
	// 8323A5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A5F4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8323A5F8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323A5FC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323A600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A604: 38AA94C4  addi r5, r10, -0x6b3c
	ctx.r[5].s64 = ctx.r[10].s64 + -27452;
	// 8323A608: 3889DDC8  addi r4, r9, -0x2238
	ctx.r[4].s64 = ctx.r[9].s64 + -8760;
	// 8323A60C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A610: 38689CA4  addi r3, r8, -0x635c
	ctx.r[3].s64 = ctx.r[8].s64 + -25436;
	// 8323A614: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A61C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A620: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A624: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323A628: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A62C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323A630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A634: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323A638: 4BC6DB91  bl 0x82ea81c8
	ctx.lr = 0x8323A63C;
	sub_82EA81C8(ctx, base);
	// 8323A63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323A640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A650 size=120
    let mut pc: u32 = 0x8323A650;
    'dispatch: loop {
        match pc {
            0x8323A650 => {
    //   block [0x8323A650..0x8323A6C8)
	// 8323A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A65C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A660: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A664: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A668: 38CADDF4  addi r6, r10, -0x220c
	ctx.r[6].s64 = ctx.r[10].s64 + -8716;
	// 8323A66C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A670: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A674: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A67C: 38A99254  addi r5, r9, -0x6dac
	ctx.r[5].s64 = ctx.r[9].s64 + -28076;
	// 8323A680: 3888DE54  addi r4, r8, -0x21ac
	ctx.r[4].s64 = ctx.r[8].s64 + -8620;
	// 8323A684: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A688: 38679CD4  addi r3, r7, -0x632c
	ctx.r[3].s64 = ctx.r[7].s64 + -25388;
	// 8323A68C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A690: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323A694: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A69C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A6A0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A6A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A6AC: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8323A6B0: 4BC6DB19  bl 0x82ea81c8
	ctx.lr = 0x8323A6B4;
	sub_82EA81C8(ctx, base);
	// 8323A6B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A6C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A6C8 size=92
    let mut pc: u32 = 0x8323A6C8;
    'dispatch: loop {
        match pc {
            0x8323A6C8 => {
    //   block [0x8323A6C8..0x8323A724)
	// 8323A6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A6D0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A6D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A6D8: 4BCE32E9  bl 0x82f1d9c0
	ctx.lr = 0x8323A6DC;
	sub_82F1D9C0(ctx, base);
	// 8323A6DC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A6E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A6E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A6E8: 396BDF34  addi r11, r11, -0x20cc
	ctx.r[11].s64 = ctx.r[11].s64 + -8396;
	// 8323A6EC: 38C99D04  addi r6, r9, -0x62fc
	ctx.r[6].s64 = ctx.r[9].s64 + -25340;
	// 8323A6F0: 3CE082F2  lis r7, -0x7d0e
	ctx.r[7].s64 = -2098069504;
	// 8323A6F4: 91699D04  stw r11, -0x62fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25340 as u32), ctx.r[11].u32 ) };
	// 8323A6F8: 3D0082F2  lis r8, -0x7d0e
	ctx.r[8].s64 = -2098069504;
	// 8323A6FC: 392ADF24  addi r9, r10, -0x20dc
	ctx.r[9].s64 = ctx.r[10].s64 + -8412;
	// 8323A700: 3947CBD0  addi r10, r7, -0x3430
	ctx.r[10].s64 = ctx.r[7].s64 + -13360;
	// 8323A704: 3968CBB8  addi r11, r8, -0x3448
	ctx.r[11].s64 = ctx.r[8].s64 + -13384;
	// 8323A708: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323A70C: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A710: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323A714: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8323A718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A728 size=120
    let mut pc: u32 = 0x8323A728;
    'dispatch: loop {
        match pc {
            0x8323A728 => {
    //   block [0x8323A728..0x8323A7A0)
	// 8323A728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A734: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A738: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A73C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A740: 38CADEA0  addi r6, r10, -0x2160
	ctx.r[6].s64 = ctx.r[10].s64 + -8544;
	// 8323A744: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A748: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A74C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A754: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323A758: 3888DF34  addi r4, r8, -0x20cc
	ctx.r[4].s64 = ctx.r[8].s64 + -8396;
	// 8323A75C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A760: 38679D14  addi r3, r7, -0x62ec
	ctx.r[3].s64 = ctx.r[7].s64 + -25324;
	// 8323A764: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A768: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323A76C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A778: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A780: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A784: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 8323A788: 4BC6DA41  bl 0x82ea81c8
	ctx.lr = 0x8323A78C;
	sub_82EA81C8(ctx, base);
	// 8323A78C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323A790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A798: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323A79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A7A0 size=96
    let mut pc: u32 = 0x8323A7A0;
    'dispatch: loop {
        match pc {
            0x8323A7A0 => {
    //   block [0x8323A7A0..0x8323A800)
	// 8323A7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A7A8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A7AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A7B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A7B4: 4BCEFA35  bl 0x82f2a1e8
	ctx.lr = 0x8323A7B8;
	sub_82F2A1E8(ctx, base);
	// 8323A7B8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A7BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A7C0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A7C4: 396BDF94  addi r11, r11, -0x206c
	ctx.r[11].s64 = ctx.r[11].s64 + -8300;
	// 8323A7C8: 38C99D44  addi r6, r9, -0x62bc
	ctx.r[6].s64 = ctx.r[9].s64 + -25276;
	// 8323A7CC: 3CE082F2  lis r7, -0x7d0e
	ctx.r[7].s64 = -2098069504;
	// 8323A7D0: 91699D44  stw r11, -0x62bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25276 as u32), ctx.r[11].u32 ) };
	// 8323A7D4: 3D0082F2  lis r8, -0x7d0e
	ctx.r[8].s64 = -2098069504;
	// 8323A7D8: 392ADF6C  addi r9, r10, -0x2094
	ctx.r[9].s64 = ctx.r[10].s64 + -8340;
	// 8323A7DC: 3947CD30  addi r10, r7, -0x32d0
	ctx.r[10].s64 = ctx.r[7].s64 + -13008;
	// 8323A7E0: 3968CD18  addi r11, r8, -0x32e8
	ctx.r[11].s64 = ctx.r[8].s64 + -13032;
	// 8323A7E4: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323A7E8: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A7EC: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323A7F0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8323A7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A800 size=100
    let mut pc: u32 = 0x8323A800;
    'dispatch: loop {
        match pc {
            0x8323A800 => {
    //   block [0x8323A800..0x8323A864)
	// 8323A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A80C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8323A810: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323A814: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323A818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A81C: 38AA9744  addi r5, r10, -0x68bc
	ctx.r[5].s64 = ctx.r[10].s64 + -26812;
	// 8323A820: 3889DF94  addi r4, r9, -0x206c
	ctx.r[4].s64 = ctx.r[9].s64 + -8300;
	// 8323A824: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A828: 38689D54  addi r3, r8, -0x62ac
	ctx.r[3].s64 = ctx.r[8].s64 + -25260;
	// 8323A82C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A834: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A838: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A83C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323A840: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A844: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323A848: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A84C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323A850: 4BC6D979  bl 0x82ea81c8
	ctx.lr = 0x8323A854;
	sub_82EA81C8(ctx, base);
	// 8323A854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323A858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A868 size=100
    let mut pc: u32 = 0x8323A868;
    'dispatch: loop {
        match pc {
            0x8323A868 => {
    //   block [0x8323A868..0x8323A8CC)
	// 8323A868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A874: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8323A878: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323A87C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323A880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A884: 38AA9494  addi r5, r10, -0x6b6c
	ctx.r[5].s64 = ctx.r[10].s64 + -27500;
	// 8323A888: 3889DFA8  addi r4, r9, -0x2058
	ctx.r[4].s64 = ctx.r[9].s64 + -8280;
	// 8323A88C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A890: 38689D84  addi r3, r8, -0x627c
	ctx.r[3].s64 = ctx.r[8].s64 + -25212;
	// 8323A894: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A89C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A8A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A8A4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323A8A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A8AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323A8B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A8B4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323A8B8: 4BC6D911  bl 0x82ea81c8
	ctx.lr = 0x8323A8BC;
	sub_82EA81C8(ctx, base);
	// 8323A8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323A8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A8D0 size=92
    let mut pc: u32 = 0x8323A8D0;
    'dispatch: loop {
        match pc {
            0x8323A8D0 => {
    //   block [0x8323A8D0..0x8323A92C)
	// 8323A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A8D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A8DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323A8E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323A8E4: 4BCE443D  bl 0x82f1ed20
	ctx.lr = 0x8323A8E8;
	sub_82F1ED20(ctx, base);
	// 8323A8E8: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323A8EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A8F0: 39099DB4  addi r8, r9, -0x624c
	ctx.r[8].s64 = ctx.r[9].s64 + -25164;
	// 8323A8F4: 396BE1D0  addi r11, r11, -0x1e30
	ctx.r[11].s64 = ctx.r[11].s64 + -7728;
	// 8323A8F8: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 8323A8FC: 91699DB4  stw r11, -0x624c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25164 as u32), ctx.r[11].u32 ) };
	// 8323A900: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 8323A904: 394ACF38  addi r10, r10, -0x30c8
	ctx.r[10].s64 = ctx.r[10].s64 + -12488;
	// 8323A908: 3929CF20  addi r9, r9, -0x30e0
	ctx.r[9].s64 = ctx.r[9].s64 + -12512;
	// 8323A90C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323A910: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323A914: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323A918: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323A91C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8323A920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A930 size=120
    let mut pc: u32 = 0x8323A930;
    'dispatch: loop {
        match pc {
            0x8323A930 => {
    //   block [0x8323A930..0x8323A9A8)
	// 8323A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A93C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323A940: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A944: 392BE110  addi r9, r11, -0x1ef0
	ctx.r[9].s64 = ctx.r[11].s64 + -7920;
	// 8323A948: 388AE140  addi r4, r10, -0x1ec0
	ctx.r[4].s64 = ctx.r[10].s64 + -7872;
	// 8323A94C: 38A90014  addi r5, r9, 0x14
	ctx.r[5].s64 = ctx.r[9].s64 + 20;
	// 8323A950: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8323A954: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8323A958: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323A95C: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 8323A960: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323A964: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8323A968: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 8323A96C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A970: 38A89464  addi r5, r8, -0x6b9c
	ctx.r[5].s64 = ctx.r[8].s64 + -27548;
	// 8323A974: 3887E1D0  addi r4, r7, -0x1e30
	ctx.r[4].s64 = ctx.r[7].s64 + -7728;
	// 8323A978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A97C: 38669DC4  addi r3, r6, -0x623c
	ctx.r[3].s64 = ctx.r[6].s64 + -25148;
	// 8323A980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323A988: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323A98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323A990: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323A994: 4BC6D835  bl 0x82ea81c8
	ctx.lr = 0x8323A998;
	sub_82EA81C8(ctx, base);
	// 8323A998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323A99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323A9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323A9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323A9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323A9A8 size=120
    let mut pc: u32 = 0x8323A9A8;
    'dispatch: loop {
        match pc {
            0x8323A9A8 => {
    //   block [0x8323A9A8..0x8323AA20)
	// 8323A9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323A9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323A9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323A9B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323A9B8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323A9BC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323A9C0: 38CAE208  addi r6, r10, -0x1df8
	ctx.r[6].s64 = ctx.r[10].s64 + -7672;
	// 8323A9C4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323A9C8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323A9CC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323A9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323A9D4: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323A9D8: 3888E24C  addi r4, r8, -0x1db4
	ctx.r[4].s64 = ctx.r[8].s64 + -7604;
	// 8323A9DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323A9E0: 38679DF4  addi r3, r7, -0x620c
	ctx.r[3].s64 = ctx.r[7].s64 + -25100;
	// 8323A9E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323A9E8: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323A9EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323A9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323A9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323A9F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323A9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AA00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AA04: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323AA08: 4BC6D7C1  bl 0x82ea81c8
	ctx.lr = 0x8323AA0C;
	sub_82EA81C8(ctx, base);
	// 8323AA0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AA18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AA20 size=120
    let mut pc: u32 = 0x8323AA20;
    'dispatch: loop {
        match pc {
            0x8323AA20 => {
    //   block [0x8323AA20..0x8323AA98)
	// 8323AA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AA2C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AA30: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AA34: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AA38: 38CAE280  addi r6, r10, -0x1d80
	ctx.r[6].s64 = ctx.r[10].s64 + -7552;
	// 8323AA3C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AA40: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AA44: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AA4C: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 8323AA50: 3888E30C  addi r4, r8, -0x1cf4
	ctx.r[4].s64 = ctx.r[8].s64 + -7412;
	// 8323AA54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AA58: 38679E24  addi r3, r7, -0x61dc
	ctx.r[3].s64 = ctx.r[7].s64 + -25052;
	// 8323AA5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AA60: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323AA64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AA70: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323AA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AA78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AA7C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323AA80: 4BC6D749  bl 0x82ea81c8
	ctx.lr = 0x8323AA84;
	sub_82EA81C8(ctx, base);
	// 8323AA84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AA98 size=120
    let mut pc: u32 = 0x8323AA98;
    'dispatch: loop {
        match pc {
            0x8323AA98 => {
    //   block [0x8323AA98..0x8323AB10)
	// 8323AA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AAA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AAA4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AAA8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AAAC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AAB0: 38CAE328  addi r6, r10, -0x1cd8
	ctx.r[6].s64 = ctx.r[10].s64 + -7384;
	// 8323AAB4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AAB8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AABC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AAC4: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 8323AAC8: 3888E3B4  addi r4, r8, -0x1c4c
	ctx.r[4].s64 = ctx.r[8].s64 + -7244;
	// 8323AACC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AAD0: 38679E54  addi r3, r7, -0x61ac
	ctx.r[3].s64 = ctx.r[7].s64 + -25004;
	// 8323AAD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AAD8: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323AADC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AAE8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323AAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AAF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AAF4: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323AAF8: 4BC6D6D1  bl 0x82ea81c8
	ctx.lr = 0x8323AAFC;
	sub_82EA81C8(ctx, base);
	// 8323AAFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AB08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AB10 size=120
    let mut pc: u32 = 0x8323AB10;
    'dispatch: loop {
        match pc {
            0x8323AB10 => {
    //   block [0x8323AB10..0x8323AB88)
	// 8323AB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AB18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AB1C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AB20: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AB24: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AB28: 38CAE3F0  addi r6, r10, -0x1c10
	ctx.r[6].s64 = ctx.r[10].s64 + -7184;
	// 8323AB2C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AB30: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AB34: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AB3C: 38A99314  addi r5, r9, -0x6cec
	ctx.r[5].s64 = ctx.r[9].s64 + -27884;
	// 8323AB40: 3888E484  addi r4, r8, -0x1b7c
	ctx.r[4].s64 = ctx.r[8].s64 + -7036;
	// 8323AB44: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AB48: 38679E84  addi r3, r7, -0x617c
	ctx.r[3].s64 = ctx.r[7].s64 + -24956;
	// 8323AB4C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AB50: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323AB54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AB60: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AB68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AB6C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323AB70: 4BC6D659  bl 0x82ea81c8
	ctx.lr = 0x8323AB74;
	sub_82EA81C8(ctx, base);
	// 8323AB74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AB80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AB88 size=120
    let mut pc: u32 = 0x8323AB88;
    'dispatch: loop {
        match pc {
            0x8323AB88 => {
    //   block [0x8323AB88..0x8323AC00)
	// 8323AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AB90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AB94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AB98: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AB9C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323ABA0: 38CAE4E0  addi r6, r10, -0x1b20
	ctx.r[6].s64 = ctx.r[10].s64 + -6944;
	// 8323ABA4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323ABA8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323ABAC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323ABB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323ABB4: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 8323ABB8: 3888E5C4  addi r4, r8, -0x1a3c
	ctx.r[4].s64 = ctx.r[8].s64 + -6716;
	// 8323ABBC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323ABC0: 38679EB4  addi r3, r7, -0x614c
	ctx.r[3].s64 = ctx.r[7].s64 + -24908;
	// 8323ABC4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323ABC8: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 8323ABCC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323ABD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323ABD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323ABD8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323ABDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323ABE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323ABE4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323ABE8: 4BC6D5E1  bl 0x82ea81c8
	ctx.lr = 0x8323ABEC;
	sub_82EA81C8(ctx, base);
	// 8323ABEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323ABF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323ABF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323ABF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323ABFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AC00 size=92
    let mut pc: u32 = 0x8323AC00;
    'dispatch: loop {
        match pc {
            0x8323AC00 => {
    //   block [0x8323AC00..0x8323AC5C)
	// 8323AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AC08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AC0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323AC10: 4BCE2DB1  bl 0x82f1d9c0
	ctx.lr = 0x8323AC14;
	sub_82F1D9C0(ctx, base);
	// 8323AC14: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AC18: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323AC1C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AC20: 396BE64C  addi r11, r11, -0x19b4
	ctx.r[11].s64 = ctx.r[11].s64 + -6580;
	// 8323AC24: 38C99EE4  addi r6, r9, -0x611c
	ctx.r[6].s64 = ctx.r[9].s64 + -24860;
	// 8323AC28: 3CE082F2  lis r7, -0x7d0e
	ctx.r[7].s64 = -2098069504;
	// 8323AC2C: 91699EE4  stw r11, -0x611c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-24860 as u32), ctx.r[11].u32 ) };
	// 8323AC30: 3D0082F2  lis r8, -0x7d0e
	ctx.r[8].s64 = -2098069504;
	// 8323AC34: 392AE640  addi r9, r10, -0x19c0
	ctx.r[9].s64 = ctx.r[10].s64 + -6592;
	// 8323AC38: 3947D450  addi r10, r7, -0x2bb0
	ctx.r[10].s64 = ctx.r[7].s64 + -11184;
	// 8323AC3C: 3968D438  addi r11, r8, -0x2bc8
	ctx.r[11].s64 = ctx.r[8].s64 + -11208;
	// 8323AC40: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323AC44: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323AC48: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323AC4C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8323AC50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AC54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AC60 size=120
    let mut pc: u32 = 0x8323AC60;
    'dispatch: loop {
        match pc {
            0x8323AC60 => {
    //   block [0x8323AC60..0x8323ACD8)
	// 8323AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AC68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AC6C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AC70: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AC74: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AC78: 38CAE5EC  addi r6, r10, -0x1a14
	ctx.r[6].s64 = ctx.r[10].s64 + -6676;
	// 8323AC7C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AC80: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AC84: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AC8C: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323AC90: 3888E64C  addi r4, r8, -0x19b4
	ctx.r[4].s64 = ctx.r[8].s64 + -6580;
	// 8323AC94: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AC98: 38679EF4  addi r3, r7, -0x610c
	ctx.r[3].s64 = ctx.r[7].s64 + -24844;
	// 8323AC9C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323ACA0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323ACA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323ACA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323ACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323ACB0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323ACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323ACB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323ACBC: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 8323ACC0: 4BC6D509  bl 0x82ea81c8
	ctx.lr = 0x8323ACC4;
	sub_82EA81C8(ctx, base);
	// 8323ACC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323ACC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323ACCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323ACD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323ACD8 size=116
    let mut pc: u32 = 0x8323ACD8;
    'dispatch: loop {
        match pc {
            0x8323ACD8 => {
    //   block [0x8323ACD8..0x8323AD4C)
	// 8323ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323ACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323ACE4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323ACE8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323ACEC: 38AAE6C0  addi r5, r10, -0x1940
	ctx.r[5].s64 = ctx.r[10].s64 + -6464;
	// 8323ACF0: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323ACF4: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 8323ACF8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323ACFC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8323AD00: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323AD04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AD08: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323AD0C: 38A89464  addi r5, r8, -0x6b9c
	ctx.r[5].s64 = ctx.r[8].s64 + -27548;
	// 8323AD10: 3887E734  addi r4, r7, -0x18cc
	ctx.r[4].s64 = ctx.r[7].s64 + -6348;
	// 8323AD14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AD18: 38669F24  addi r3, r6, -0x60dc
	ctx.r[3].s64 = ctx.r[6].s64 + -24796;
	// 8323AD1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AD20: 3929E6AC  addi r9, r9, -0x1954
	ctx.r[9].s64 = ctx.r[9].s64 + -6484;
	// 8323AD24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AD28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323AD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AD30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AD34: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323AD38: 4BC6D491  bl 0x82ea81c8
	ctx.lr = 0x8323AD3C;
	sub_82EA81C8(ctx, base);
	// 8323AD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323AD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AD50 size=120
    let mut pc: u32 = 0x8323AD50;
    'dispatch: loop {
        match pc {
            0x8323AD50 => {
    //   block [0x8323AD50..0x8323ADC8)
	// 8323AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AD58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AD5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AD60: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AD64: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AD68: 38CAE758  addi r6, r10, -0x18a8
	ctx.r[6].s64 = ctx.r[10].s64 + -6312;
	// 8323AD6C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AD70: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AD74: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AD7C: 38A99344  addi r5, r9, -0x6cbc
	ctx.r[5].s64 = ctx.r[9].s64 + -27836;
	// 8323AD80: 3888E7B0  addi r4, r8, -0x1850
	ctx.r[4].s64 = ctx.r[8].s64 + -6224;
	// 8323AD84: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AD88: 38679F54  addi r3, r7, -0x60ac
	ctx.r[3].s64 = ctx.r[7].s64 + -24748;
	// 8323AD8C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AD90: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323AD94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323ADA0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323ADA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323ADA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323ADAC: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323ADB0: 4BC6D419  bl 0x82ea81c8
	ctx.lr = 0x8323ADB4;
	sub_82EA81C8(ctx, base);
	// 8323ADB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323ADB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323ADBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323ADC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323ADC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323ADC8 size=120
    let mut pc: u32 = 0x8323ADC8;
    'dispatch: loop {
        match pc {
            0x8323ADC8 => {
    //   block [0x8323ADC8..0x8323AE40)
	// 8323ADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323ADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323ADD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323ADD4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323ADD8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323ADDC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323ADE0: 38CAE7C8  addi r6, r10, -0x1838
	ctx.r[6].s64 = ctx.r[10].s64 + -6200;
	// 8323ADE4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323ADE8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323ADEC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323ADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323ADF4: 38A99464  addi r5, r9, -0x6b9c
	ctx.r[5].s64 = ctx.r[9].s64 + -27548;
	// 8323ADF8: 3888E824  addi r4, r8, -0x17dc
	ctx.r[4].s64 = ctx.r[8].s64 + -6108;
	// 8323ADFC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AE00: 38679F84  addi r3, r7, -0x607c
	ctx.r[3].s64 = ctx.r[7].s64 + -24700;
	// 8323AE04: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AE08: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323AE0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AE18: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323AE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AE20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AE24: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323AE28: 4BC6D3A1  bl 0x82ea81c8
	ctx.lr = 0x8323AE2C;
	sub_82EA81C8(ctx, base);
	// 8323AE2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AE38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323AE40 size=12
    let mut pc: u32 = 0x8323AE40;
    'dispatch: loop {
        match pc {
            0x8323AE40 => {
    //   block [0x8323AE40..0x8323AE4C)
	// 8323AE40: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323AE44: 386B1EF0  addi r3, r11, 0x1ef0
	ctx.r[3].s64 = ctx.r[11].s64 + 7920;
	// 8323AE48: 4BF6D690  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AE50 size=108
    let mut pc: u32 = 0x8323AE50;
    'dispatch: loop {
        match pc {
            0x8323AE50 => {
    //   block [0x8323AE50..0x8323AEBC)
	// 8323AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AE5C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AE60: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323AE64: 38EA0228  addi r7, r10, 0x228
	ctx.r[7].s64 = ctx.r[10].s64 + 552;
	// 8323AE68: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323AE6C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323AE70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323AE74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AE78: 388902F4  addi r4, r9, 0x2f4
	ctx.r[4].s64 = ctx.r[9].s64 + 756;
	// 8323AE7C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323AE80: 3868A308  addi r3, r8, -0x5cf8
	ctx.r[3].s64 = ctx.r[8].s64 + -23800;
	// 8323AE84: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AE8C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AE90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AE94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AE98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AEA0: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323AEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323AEA8: 4BC6D321  bl 0x82ea81c8
	ctx.lr = 0x8323AEAC;
	sub_82EA81C8(ctx, base);
	// 8323AEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323AEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AEC0 size=120
    let mut pc: u32 = 0x8323AEC0;
    'dispatch: loop {
        match pc {
            0x8323AEC0 => {
    //   block [0x8323AEC0..0x8323AF38)
	// 8323AEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AEC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323AECC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AED0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323AED4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323AED8: 38CA0258  addi r6, r10, 0x258
	ctx.r[6].s64 = ctx.r[10].s64 + 600;
	// 8323AEDC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323AEE0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323AEE4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323AEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AEEC: 38A9A308  addi r5, r9, -0x5cf8
	ctx.r[5].s64 = ctx.r[9].s64 + -23800;
	// 8323AEF0: 38880310  addi r4, r8, 0x310
	ctx.r[4].s64 = ctx.r[8].s64 + 784;
	// 8323AEF4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323AEF8: 3867A2C8  addi r3, r7, -0x5d38
	ctx.r[3].s64 = ctx.r[7].s64 + -23864;
	// 8323AEFC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323AF00: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323AF04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323AF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323AF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323AF10: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323AF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323AF18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323AF1C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323AF20: 4BC6D2A9  bl 0x82ea81c8
	ctx.lr = 0x8323AF24;
	sub_82EA81C8(ctx, base);
	// 8323AF24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323AF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AF30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323AF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AF38 size=112
    let mut pc: u32 = 0x8323AF38;
    'dispatch: loop {
        match pc {
            0x8323AF38 => {
    //   block [0x8323AF38..0x8323AFA8)
	// 8323AF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AF40: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AF44: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323AF48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323AF4C: 392B02E8  addi r9, r11, 0x2e8
	ctx.r[9].s64 = ctx.r[11].s64 + 744;
	// 8323AF50: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8323AF54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323AF58: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8323AF5C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8323AF60: 4BCB2619  bl 0x82eed578
	ctx.lr = 0x8323AF64;
	sub_82EED578(ctx, base);
	// 8323AF64: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323AF68: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323AF6C: 38E8A2F8  addi r7, r8, -0x5d08
	ctx.r[7].s64 = ctx.r[8].s64 + -23816;
	// 8323AF70: 396B032C  addi r11, r11, 0x32c
	ctx.r[11].s64 = ctx.r[11].s64 + 812;
	// 8323AF74: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 8323AF78: 9168A2F8  stw r11, -0x5d08(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-23816 as u32), ctx.r[11].u32 ) };
	// 8323AF7C: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 8323AF80: 394AC178  addi r10, r10, -0x3e88
	ctx.r[10].s64 = ctx.r[10].s64 + -16008;
	// 8323AF84: 3929C140  addi r9, r9, -0x3ec0
	ctx.r[9].s64 = ctx.r[9].s64 + -16064;
	// 8323AF88: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323AF8C: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323AF90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323AF94: 9167000C  stw r11, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323AF98: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8323AF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323AFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323AFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323AFA8 size=24
    let mut pc: u32 = 0x8323AFA8;
    'dispatch: loop {
        match pc {
            0x8323AFA8 => {
    //   block [0x8323AFA8..0x8323AFC0)
	// 8323AFA8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323AFAC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323AFB0: 392A1448  addi r9, r10, 0x1448
	ctx.r[9].s64 = ctx.r[10].s64 + 5192;
	// 8323AFB4: 816B1420  lwz r11, 0x1420(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5152 as u32) ) } as u64;
	// 8323AFB8: 91690080  stw r11, 0x80(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8323AFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323AFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323AFC0 size=116
    let mut pc: u32 = 0x8323AFC0;
    'dispatch: loop {
        match pc {
            0x8323AFC0 => {
    //   block [0x8323AFC0..0x8323B034)
	// 8323AFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323AFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323AFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323AFCC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323AFD0: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323AFD4: 38AA1448  addi r5, r10, 0x1448
	ctx.r[5].s64 = ctx.r[10].s64 + 5192;
	// 8323AFD8: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323AFDC: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 8323AFE0: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323AFE4: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 8323AFE8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323AFEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323AFF0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323AFF4: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 8323AFF8: 3887032C  addi r4, r7, 0x32c
	ctx.r[4].s64 = ctx.r[7].s64 + 812;
	// 8323AFFC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B000: 3866A338  addi r3, r6, -0x5cc8
	ctx.r[3].s64 = ctx.r[6].s64 + -23752;
	// 8323B004: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B008: 392902D0  addi r9, r9, 0x2d0
	ctx.r[9].s64 = ctx.r[9].s64 + 720;
	// 8323B00C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B010: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323B014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B018: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B01C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8323B020: 4BC6D1A9  bl 0x82ea81c8
	ctx.lr = 0x8323B024;
	sub_82EA81C8(ctx, base);
	// 8323B024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B038 size=108
    let mut pc: u32 = 0x8323B038;
    'dispatch: loop {
        match pc {
            0x8323B038 => {
    //   block [0x8323B038..0x8323B0A4)
	// 8323B038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B044: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B048: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B04C: 38EA03B8  addi r7, r10, 0x3b8
	ctx.r[7].s64 = ctx.r[10].s64 + 952;
	// 8323B050: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B054: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323B058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B05C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B060: 38890410  addi r4, r9, 0x410
	ctx.r[4].s64 = ctx.r[9].s64 + 1040;
	// 8323B064: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B068: 3868A398  addi r3, r8, -0x5c68
	ctx.r[3].s64 = ctx.r[8].s64 + -23656;
	// 8323B06C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B074: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B078: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B07C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B080: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B088: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323B08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B090: 4BC6D139  bl 0x82ea81c8
	ctx.lr = 0x8323B094;
	sub_82EA81C8(ctx, base);
	// 8323B094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B0A8 size=120
    let mut pc: u32 = 0x8323B0A8;
    'dispatch: loop {
        match pc {
            0x8323B0A8 => {
    //   block [0x8323B0A8..0x8323B120)
	// 8323B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B0B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B0B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B0B8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B0BC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B0C0: 38CA03E8  addi r6, r10, 0x3e8
	ctx.r[6].s64 = ctx.r[10].s64 + 1000;
	// 8323B0C4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B0C8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B0CC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B0D4: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323B0D8: 38880448  addi r4, r8, 0x448
	ctx.r[4].s64 = ctx.r[8].s64 + 1096;
	// 8323B0DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B0E0: 3867A368  addi r3, r7, -0x5c98
	ctx.r[3].s64 = ctx.r[7].s64 + -23704;
	// 8323B0E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B0E8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323B0EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B0F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B0F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B0FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B104: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323B108: 4BC6D0C1  bl 0x82ea81c8
	ctx.lr = 0x8323B10C;
	sub_82EA81C8(ctx, base);
	// 8323B10C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B120 size=108
    let mut pc: u32 = 0x8323B120;
    'dispatch: loop {
        match pc {
            0x8323B120 => {
    //   block [0x8323B120..0x8323B18C)
	// 8323B120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B12C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B130: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B134: 38EA04E8  addi r7, r10, 0x4e8
	ctx.r[7].s64 = ctx.r[10].s64 + 1256;
	// 8323B138: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B13C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323B140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B144: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B148: 38890590  addi r4, r9, 0x590
	ctx.r[4].s64 = ctx.r[9].s64 + 1424;
	// 8323B14C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B150: 3868A3C8  addi r3, r8, -0x5c38
	ctx.r[3].s64 = ctx.r[8].s64 + -23608;
	// 8323B154: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B15C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B160: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B164: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B168: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B170: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323B174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B178: 4BC6D051  bl 0x82ea81c8
	ctx.lr = 0x8323B17C;
	sub_82EA81C8(ctx, base);
	// 8323B17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B190 size=108
    let mut pc: u32 = 0x8323B190;
    'dispatch: loop {
        match pc {
            0x8323B190 => {
    //   block [0x8323B190..0x8323B1FC)
	// 8323B190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B19C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B1A0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B1A4: 38EA0530  addi r7, r10, 0x530
	ctx.r[7].s64 = ctx.r[10].s64 + 1328;
	// 8323B1A8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B1AC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323B1B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B1B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B1B8: 388905AC  addi r4, r9, 0x5ac
	ctx.r[4].s64 = ctx.r[9].s64 + 1452;
	// 8323B1BC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B1C0: 3868A3F8  addi r3, r8, -0x5c08
	ctx.r[3].s64 = ctx.r[8].s64 + -23560;
	// 8323B1C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B1CC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B1D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B1D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B1D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B1E0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323B1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B1E8: 4BC6CFE1  bl 0x82ea81c8
	ctx.lr = 0x8323B1EC;
	sub_82EA81C8(ctx, base);
	// 8323B1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B200 size=108
    let mut pc: u32 = 0x8323B200;
    'dispatch: loop {
        match pc {
            0x8323B200 => {
    //   block [0x8323B200..0x8323B26C)
	// 8323B200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B20C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B210: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B214: 38EA0560  addi r7, r10, 0x560
	ctx.r[7].s64 = ctx.r[10].s64 + 1376;
	// 8323B218: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B21C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323B220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B224: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B228: 388905CC  addi r4, r9, 0x5cc
	ctx.r[4].s64 = ctx.r[9].s64 + 1484;
	// 8323B22C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B230: 3868A428  addi r3, r8, -0x5bd8
	ctx.r[3].s64 = ctx.r[8].s64 + -23512;
	// 8323B234: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B23C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B240: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B244: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B248: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B250: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323B254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B258: 4BC6CF71  bl 0x82ea81c8
	ctx.lr = 0x8323B25C;
	sub_82EA81C8(ctx, base);
	// 8323B25C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B270 size=108
    let mut pc: u32 = 0x8323B270;
    'dispatch: loop {
        match pc {
            0x8323B270 => {
    //   block [0x8323B270..0x8323B2DC)
	// 8323B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B27C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B280: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B284: 38EA0638  addi r7, r10, 0x638
	ctx.r[7].s64 = ctx.r[10].s64 + 1592;
	// 8323B288: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B28C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323B290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B294: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B298: 38890708  addi r4, r9, 0x708
	ctx.r[4].s64 = ctx.r[9].s64 + 1800;
	// 8323B29C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B2A0: 3868A4B8  addi r3, r8, -0x5b48
	ctx.r[3].s64 = ctx.r[8].s64 + -23368;
	// 8323B2A4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B2AC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B2B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B2B4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B2B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B2C0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323B2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B2C8: 4BC6CF01  bl 0x82ea81c8
	ctx.lr = 0x8323B2CC;
	sub_82EA81C8(ctx, base);
	// 8323B2CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B2E0 size=108
    let mut pc: u32 = 0x8323B2E0;
    'dispatch: loop {
        match pc {
            0x8323B2E0 => {
    //   block [0x8323B2E0..0x8323B34C)
	// 8323B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B2EC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B2F0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B2F4: 38EA0668  addi r7, r10, 0x668
	ctx.r[7].s64 = ctx.r[10].s64 + 1640;
	// 8323B2F8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B2FC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323B300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B304: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B308: 38890724  addi r4, r9, 0x724
	ctx.r[4].s64 = ctx.r[9].s64 + 1828;
	// 8323B30C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B310: 3868A488  addi r3, r8, -0x5b78
	ctx.r[3].s64 = ctx.r[8].s64 + -23416;
	// 8323B314: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B31C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B320: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B324: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B328: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B330: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323B334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B338: 4BC6CE91  bl 0x82ea81c8
	ctx.lr = 0x8323B33C;
	sub_82EA81C8(ctx, base);
	// 8323B33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B350 size=120
    let mut pc: u32 = 0x8323B350;
    'dispatch: loop {
        match pc {
            0x8323B350 => {
    //   block [0x8323B350..0x8323B3C8)
	// 8323B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B35C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B360: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B364: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B368: 38CA06B0  addi r6, r10, 0x6b0
	ctx.r[6].s64 = ctx.r[10].s64 + 1712;
	// 8323B36C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B370: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B374: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B37C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323B380: 38880744  addi r4, r8, 0x744
	ctx.r[4].s64 = ctx.r[8].s64 + 1860;
	// 8323B384: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B388: 3867A458  addi r3, r7, -0x5ba8
	ctx.r[3].s64 = ctx.r[7].s64 + -23464;
	// 8323B38C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B390: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323B394: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B3A0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B3A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B3AC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8323B3B0: 4BC6CE19  bl 0x82ea81c8
	ctx.lr = 0x8323B3B4;
	sub_82EA81C8(ctx, base);
	// 8323B3B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B3C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B3C8 size=108
    let mut pc: u32 = 0x8323B3C8;
    'dispatch: loop {
        match pc {
            0x8323B3C8 => {
    //   block [0x8323B3C8..0x8323B434)
	// 8323B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B3D4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B3D8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B3DC: 38EA076C  addi r7, r10, 0x76c
	ctx.r[7].s64 = ctx.r[10].s64 + 1900;
	// 8323B3E0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B3E4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323B3E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323B3EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B3F0: 38890818  addi r4, r9, 0x818
	ctx.r[4].s64 = ctx.r[9].s64 + 2072;
	// 8323B3F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B3F8: 3868A4F8  addi r3, r8, -0x5b08
	ctx.r[3].s64 = ctx.r[8].s64 + -23304;
	// 8323B3FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B404: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B408: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B40C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B410: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B418: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323B41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B420: 4BC6CDA9  bl 0x82ea81c8
	ctx.lr = 0x8323B424;
	sub_82EA81C8(ctx, base);
	// 8323B424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B438 size=92
    let mut pc: u32 = 0x8323B438;
    'dispatch: loop {
        match pc {
            0x8323B438 => {
    //   block [0x8323B438..0x8323B494)
	// 8323B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B440: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323B448: 4BCE2579  bl 0x82f1d9c0
	ctx.lr = 0x8323B44C;
	sub_82F1D9C0(ctx, base);
	// 8323B44C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323B450: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323B454: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B458: 396B0840  addi r11, r11, 0x840
	ctx.r[11].s64 = ctx.r[11].s64 + 2112;
	// 8323B45C: 38C9A4E8  addi r6, r9, -0x5b18
	ctx.r[6].s64 = ctx.r[9].s64 + -23320;
	// 8323B460: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 8323B464: 9169A4E8  stw r11, -0x5b18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-23320 as u32), ctx.r[11].u32 ) };
	// 8323B468: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 8323B46C: 392A07FC  addi r9, r10, 0x7fc
	ctx.r[9].s64 = ctx.r[10].s64 + 2044;
	// 8323B470: 3947C688  addi r10, r7, -0x3978
	ctx.r[10].s64 = ctx.r[7].s64 + -14712;
	// 8323B474: 3968C670  addi r11, r8, -0x3990
	ctx.r[11].s64 = ctx.r[8].s64 + -14736;
	// 8323B478: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323B47C: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323B480: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323B484: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8323B488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B498 size=120
    let mut pc: u32 = 0x8323B498;
    'dispatch: loop {
        match pc {
            0x8323B498 => {
    //   block [0x8323B498..0x8323B510)
	// 8323B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B4A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B4A8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B4AC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323B4B0: 38CA079C  addi r6, r10, 0x79c
	ctx.r[6].s64 = ctx.r[10].s64 + 1948;
	// 8323B4B4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B4B8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B4BC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B4C4: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323B4C8: 38880840  addi r4, r8, 0x840
	ctx.r[4].s64 = ctx.r[8].s64 + 2112;
	// 8323B4CC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B4D0: 3867A528  addi r3, r7, -0x5ad8
	ctx.r[3].s64 = ctx.r[7].s64 + -23256;
	// 8323B4D4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B4D8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323B4DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B4E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B4EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323B4F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B4F4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323B4F8: 4BC6CCD1  bl 0x82ea81c8
	ctx.lr = 0x8323B4FC;
	sub_82EA81C8(ctx, base);
	// 8323B4FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B510 size=108
    let mut pc: u32 = 0x8323B510;
    'dispatch: loop {
        match pc {
            0x8323B510 => {
    //   block [0x8323B510..0x8323B57C)
	// 8323B510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B51C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323B520: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323B524: 396B09A8  addi r11, r11, 0x9a8
	ctx.r[11].s64 = ctx.r[11].s64 + 2472;
	// 8323B528: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323B52C: 38EB01C8  addi r7, r11, 0x1c8
	ctx.r[7].s64 = ctx.r[11].s64 + 456;
	// 8323B530: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323B534: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 8323B538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B53C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8323B540: 38890BC4  addi r4, r9, 0xbc4
	ctx.r[4].s64 = ctx.r[9].s64 + 3012;
	// 8323B544: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323B548: 3868A558  addi r3, r8, -0x5aa8
	ctx.r[3].s64 = ctx.r[8].s64 + -23208;
	// 8323B54C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8323B550: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B554: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 8323B558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B560: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8323B564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B568: 4BC6CC61  bl 0x82ea81c8
	ctx.lr = 0x8323B56C;
	sub_82EA81C8(ctx, base);
	// 8323B56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323B570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B580 size=120
    let mut pc: u32 = 0x8323B580;
    'dispatch: loop {
        match pc {
            0x8323B580 => {
    //   block [0x8323B580..0x8323B5F8)
	// 8323B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B58C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B590: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B594: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B598: 38CA0C30  addi r6, r10, 0xc30
	ctx.r[6].s64 = ctx.r[10].s64 + 3120;
	// 8323B59C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B5A0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B5A4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B5AC: 38A97CF4  addi r5, r9, 0x7cf4
	ctx.r[5].s64 = ctx.r[9].s64 + 31988;
	// 8323B5B0: 38880D38  addi r4, r8, 0xd38
	ctx.r[4].s64 = ctx.r[8].s64 + 3384;
	// 8323B5B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B5B8: 3867A588  addi r3, r7, -0x5a78
	ctx.r[3].s64 = ctx.r[7].s64 + -23160;
	// 8323B5BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B5C0: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 8323B5C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B5D0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B5D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B5DC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323B5E0: 4BC6CBE9  bl 0x82ea81c8
	ctx.lr = 0x8323B5E4;
	sub_82EA81C8(ctx, base);
	// 8323B5E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B5E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B5EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B5F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B5F8 size=120
    let mut pc: u32 = 0x8323B5F8;
    'dispatch: loop {
        match pc {
            0x8323B5F8 => {
    //   block [0x8323B5F8..0x8323B670)
	// 8323B5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B604: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B608: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B60C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B610: 38CA0D50  addi r6, r10, 0xd50
	ctx.r[6].s64 = ctx.r[10].s64 + 3408;
	// 8323B614: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B618: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B61C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B624: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323B628: 38880D78  addi r4, r8, 0xd78
	ctx.r[4].s64 = ctx.r[8].s64 + 3448;
	// 8323B62C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B630: 3867A5B8  addi r3, r7, -0x5a48
	ctx.r[3].s64 = ctx.r[7].s64 + -23112;
	// 8323B634: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B638: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323B63C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B648: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B650: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B654: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323B658: 4BC6CB71  bl 0x82ea81c8
	ctx.lr = 0x8323B65C;
	sub_82EA81C8(ctx, base);
	// 8323B65C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B668: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B670 size=120
    let mut pc: u32 = 0x8323B670;
    'dispatch: loop {
        match pc {
            0x8323B670 => {
    //   block [0x8323B670..0x8323B6E8)
	// 8323B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B67C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B680: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B684: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B688: 38CA0D98  addi r6, r10, 0xd98
	ctx.r[6].s64 = ctx.r[10].s64 + 3480;
	// 8323B68C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B690: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B694: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B69C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323B6A0: 38880DC0  addi r4, r8, 0xdc0
	ctx.r[4].s64 = ctx.r[8].s64 + 3520;
	// 8323B6A4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B6A8: 3867A5E8  addi r3, r7, -0x5a18
	ctx.r[3].s64 = ctx.r[7].s64 + -23064;
	// 8323B6AC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B6B0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323B6B4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B6C0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B6C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B6CC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323B6D0: 4BC6CAF9  bl 0x82ea81c8
	ctx.lr = 0x8323B6D4;
	sub_82EA81C8(ctx, base);
	// 8323B6D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B6E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B6E8 size=120
    let mut pc: u32 = 0x8323B6E8;
    'dispatch: loop {
        match pc {
            0x8323B6E8 => {
    //   block [0x8323B6E8..0x8323B760)
	// 8323B6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B6F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B6F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B6F8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B6FC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B700: 38CA0DF8  addi r6, r10, 0xdf8
	ctx.r[6].s64 = ctx.r[10].s64 + 3576;
	// 8323B704: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B708: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B70C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B714: 38A97AE4  addi r5, r9, 0x7ae4
	ctx.r[5].s64 = ctx.r[9].s64 + 31460;
	// 8323B718: 38880EA0  addi r4, r8, 0xea0
	ctx.r[4].s64 = ctx.r[8].s64 + 3744;
	// 8323B71C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B720: 3867A618  addi r3, r7, -0x59e8
	ctx.r[3].s64 = ctx.r[7].s64 + -23016;
	// 8323B724: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B728: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323B72C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B738: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B740: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B744: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323B748: 4BC6CA81  bl 0x82ea81c8
	ctx.lr = 0x8323B74C;
	sub_82EA81C8(ctx, base);
	// 8323B74C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B760 size=120
    let mut pc: u32 = 0x8323B760;
    'dispatch: loop {
        match pc {
            0x8323B760 => {
    //   block [0x8323B760..0x8323B7D8)
	// 8323B760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B768: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B76C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B770: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B774: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323B778: 38CA0EC0  addi r6, r10, 0xec0
	ctx.r[6].s64 = ctx.r[10].s64 + 3776;
	// 8323B77C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B780: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B784: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B78C: 38A98E64  addi r5, r9, -0x719c
	ctx.r[5].s64 = ctx.r[9].s64 + -29084;
	// 8323B790: 38880EF0  addi r4, r8, 0xef0
	ctx.r[4].s64 = ctx.r[8].s64 + 3824;
	// 8323B794: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B798: 3867A648  addi r3, r7, -0x59b8
	ctx.r[3].s64 = ctx.r[7].s64 + -22968;
	// 8323B79C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B7A0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323B7A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B7B0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B7B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B7BC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323B7C0: 4BC6CA09  bl 0x82ea81c8
	ctx.lr = 0x8323B7C4;
	sub_82EA81C8(ctx, base);
	// 8323B7C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B7D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B7D8 size=120
    let mut pc: u32 = 0x8323B7D8;
    'dispatch: loop {
        match pc {
            0x8323B7D8 => {
    //   block [0x8323B7D8..0x8323B850)
	// 8323B7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B7E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B7E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B7E8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B7EC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B7F0: 38CA0F24  addi r6, r10, 0xf24
	ctx.r[6].s64 = ctx.r[10].s64 + 3876;
	// 8323B7F4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B7F8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B7FC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B804: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323B808: 38880F64  addi r4, r8, 0xf64
	ctx.r[4].s64 = ctx.r[8].s64 + 3940;
	// 8323B80C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B810: 3867A678  addi r3, r7, -0x5988
	ctx.r[3].s64 = ctx.r[7].s64 + -22920;
	// 8323B814: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B818: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323B81C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B828: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B834: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323B838: 4BC6C991  bl 0x82ea81c8
	ctx.lr = 0x8323B83C;
	sub_82EA81C8(ctx, base);
	// 8323B83C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B850 size=100
    let mut pc: u32 = 0x8323B850;
    'dispatch: loop {
        match pc {
            0x8323B850 => {
    //   block [0x8323B850..0x8323B8B4)
	// 8323B850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B858: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323B860: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323B864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323B868: 4BFC5E49  bl 0x832016b0
	ctx.lr = 0x8323B86C;
	sub_832016B0(ctx, base);
	// 8323B86C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323B870: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323B874: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B878: 396B10CC  addi r11, r11, 0x10cc
	ctx.r[11].s64 = ctx.r[11].s64 + 4300;
	// 8323B87C: 38C9A6D8  addi r6, r9, -0x5928
	ctx.r[6].s64 = ctx.r[9].s64 + -22824;
	// 8323B880: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 8323B884: 9169A6D8  stw r11, -0x5928(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22824 as u32), ctx.r[11].u32 ) };
	// 8323B888: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 8323B88C: 392A10AC  addi r9, r10, 0x10ac
	ctx.r[9].s64 = ctx.r[10].s64 + 4268;
	// 8323B890: 3947CC90  addi r10, r7, -0x3370
	ctx.r[10].s64 = ctx.r[7].s64 + -13168;
	// 8323B894: 3968CC78  addi r11, r8, -0x3388
	ctx.r[11].s64 = ctx.r[8].s64 + -13192;
	// 8323B898: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323B89C: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323B8A0: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323B8A4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8323B8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B8B8 size=120
    let mut pc: u32 = 0x8323B8B8;
    'dispatch: loop {
        match pc {
            0x8323B8B8 => {
    //   block [0x8323B8B8..0x8323B930)
	// 8323B8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B8C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B8C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B8C8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B8CC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B8D0: 38CA0FE8  addi r6, r10, 0xfe8
	ctx.r[6].s64 = ctx.r[10].s64 + 4072;
	// 8323B8D4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B8D8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B8DC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B8E4: 38A97AE4  addi r5, r9, 0x7ae4
	ctx.r[5].s64 = ctx.r[9].s64 + 31460;
	// 8323B8E8: 388810CC  addi r4, r8, 0x10cc
	ctx.r[4].s64 = ctx.r[8].s64 + 4300;
	// 8323B8EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B8F0: 3867A6A8  addi r3, r7, -0x5958
	ctx.r[3].s64 = ctx.r[7].s64 + -22872;
	// 8323B8F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B8F8: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 8323B8FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B908: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B910: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B914: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323B918: 4BC6C8B1  bl 0x82ea81c8
	ctx.lr = 0x8323B91C;
	sub_82EA81C8(ctx, base);
	// 8323B91C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B930 size=120
    let mut pc: u32 = 0x8323B930;
    'dispatch: loop {
        match pc {
            0x8323B930 => {
    //   block [0x8323B930..0x8323B9A8)
	// 8323B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323B93C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B940: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B944: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323B948: 38CA1100  addi r6, r10, 0x1100
	ctx.r[6].s64 = ctx.r[10].s64 + 4352;
	// 8323B94C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323B950: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323B954: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323B95C: 38A97AE4  addi r5, r9, 0x7ae4
	ctx.r[5].s64 = ctx.r[9].s64 + 31460;
	// 8323B960: 38881184  addi r4, r8, 0x1184
	ctx.r[4].s64 = ctx.r[8].s64 + 4484;
	// 8323B964: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323B968: 3867A6E8  addi r3, r7, -0x5918
	ctx.r[3].s64 = ctx.r[7].s64 + -22808;
	// 8323B96C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323B970: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323B974: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323B978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323B97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323B980: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323B988: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323B98C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323B990: 4BC6C839  bl 0x82ea81c8
	ctx.lr = 0x8323B994;
	sub_82EA81C8(ctx, base);
	// 8323B994: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323B998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323B9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323B9A8 size=92
    let mut pc: u32 = 0x8323B9A8;
    'dispatch: loop {
        match pc {
            0x8323B9A8 => {
    //   block [0x8323B9A8..0x8323BA04)
	// 8323B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323B9B0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323B9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323B9B8: 4BCE2009  bl 0x82f1d9c0
	ctx.lr = 0x8323B9BC;
	sub_82F1D9C0(ctx, base);
	// 8323B9BC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323B9C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323B9C4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323B9C8: 396B1234  addi r11, r11, 0x1234
	ctx.r[11].s64 = ctx.r[11].s64 + 4660;
	// 8323B9CC: 38C9A718  addi r6, r9, -0x58e8
	ctx.r[6].s64 = ctx.r[9].s64 + -22760;
	// 8323B9D0: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 8323B9D4: 9169A718  stw r11, -0x58e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22760 as u32), ctx.r[11].u32 ) };
	// 8323B9D8: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 8323B9DC: 392A1228  addi r9, r10, 0x1228
	ctx.r[9].s64 = ctx.r[10].s64 + 4648;
	// 8323B9E0: 3947CE20  addi r10, r7, -0x31e0
	ctx.r[10].s64 = ctx.r[7].s64 + -12768;
	// 8323B9E4: 3968CE08  addi r11, r8, -0x31f8
	ctx.r[11].s64 = ctx.r[8].s64 + -12792;
	// 8323B9E8: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323B9EC: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323B9F0: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323B9F4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8323B9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323B9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BA08 size=120
    let mut pc: u32 = 0x8323BA08;
    'dispatch: loop {
        match pc {
            0x8323BA08 => {
    //   block [0x8323BA08..0x8323BA80)
	// 8323BA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BA14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BA18: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BA1C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BA20: 38CA11BC  addi r6, r10, 0x11bc
	ctx.r[6].s64 = ctx.r[10].s64 + 4540;
	// 8323BA24: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BA28: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BA2C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BA34: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323BA38: 38881234  addi r4, r8, 0x1234
	ctx.r[4].s64 = ctx.r[8].s64 + 4660;
	// 8323BA3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BA40: 3867A728  addi r3, r7, -0x58d8
	ctx.r[3].s64 = ctx.r[7].s64 + -22744;
	// 8323BA44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BA48: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323BA4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BA58: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BA60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BA64: 38C000B4  li r6, 0xb4
	ctx.r[6].s64 = 180;
	// 8323BA68: 4BC6C761  bl 0x82ea81c8
	ctx.lr = 0x8323BA6C;
	sub_82EA81C8(ctx, base);
	// 8323BA6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BA78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BA80 size=92
    let mut pc: u32 = 0x8323BA80;
    'dispatch: loop {
        match pc {
            0x8323BA80 => {
    //   block [0x8323BA80..0x8323BADC)
	// 8323BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BA88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BA8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323BA90: 4BCE1F31  bl 0x82f1d9c0
	ctx.lr = 0x8323BA94;
	sub_82F1D9C0(ctx, base);
	// 8323BA94: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BA98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323BA9C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BAA0: 396B12D8  addi r11, r11, 0x12d8
	ctx.r[11].s64 = ctx.r[11].s64 + 4824;
	// 8323BAA4: 38C9A758  addi r6, r9, -0x58a8
	ctx.r[6].s64 = ctx.r[9].s64 + -22696;
	// 8323BAA8: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 8323BAAC: 9169A758  stw r11, -0x58a8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22696 as u32), ctx.r[11].u32 ) };
	// 8323BAB0: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 8323BAB4: 392A12C0  addi r9, r10, 0x12c0
	ctx.r[9].s64 = ctx.r[10].s64 + 4800;
	// 8323BAB8: 3947CF68  addi r10, r7, -0x3098
	ctx.r[10].s64 = ctx.r[7].s64 + -12440;
	// 8323BABC: 3968CF50  addi r11, r8, -0x30b0
	ctx.r[11].s64 = ctx.r[8].s64 + -12464;
	// 8323BAC0: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323BAC4: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323BAC8: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323BACC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8323BAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BAE0 size=120
    let mut pc: u32 = 0x8323BAE0;
    'dispatch: loop {
        match pc {
            0x8323BAE0 => {
    //   block [0x8323BAE0..0x8323BB58)
	// 8323BAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BAE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BAEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BAF0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BAF4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BAF8: 38CA1260  addi r6, r10, 0x1260
	ctx.r[6].s64 = ctx.r[10].s64 + 4704;
	// 8323BAFC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BB00: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BB04: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BB0C: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323BB10: 388812D8  addi r4, r8, 0x12d8
	ctx.r[4].s64 = ctx.r[8].s64 + 4824;
	// 8323BB14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BB18: 3867A768  addi r3, r7, -0x5898
	ctx.r[3].s64 = ctx.r[7].s64 + -22680;
	// 8323BB1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BB20: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323BB24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BB30: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BB34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323BB38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BB3C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323BB40: 4BC6C689  bl 0x82ea81c8
	ctx.lr = 0x8323BB44;
	sub_82EA81C8(ctx, base);
	// 8323BB44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BB50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BB58 size=120
    let mut pc: u32 = 0x8323BB58;
    'dispatch: loop {
        match pc {
            0x8323BB58 => {
    //   block [0x8323BB58..0x8323BBD0)
	// 8323BB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BB60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BB64: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BB68: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BB6C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323BB70: 38CA1300  addi r6, r10, 0x1300
	ctx.r[6].s64 = ctx.r[10].s64 + 4864;
	// 8323BB74: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BB78: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BB7C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BB84: 38A97CF4  addi r5, r9, 0x7cf4
	ctx.r[5].s64 = ctx.r[9].s64 + 31988;
	// 8323BB88: 38881384  addi r4, r8, 0x1384
	ctx.r[4].s64 = ctx.r[8].s64 + 4996;
	// 8323BB8C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BB90: 3867A798  addi r3, r7, -0x5868
	ctx.r[3].s64 = ctx.r[7].s64 + -22632;
	// 8323BB94: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BB98: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323BB9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BBA8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BBB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BBB4: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323BBB8: 4BC6C611  bl 0x82ea81c8
	ctx.lr = 0x8323BBBC;
	sub_82EA81C8(ctx, base);
	// 8323BBBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BBC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BBD0 size=92
    let mut pc: u32 = 0x8323BBD0;
    'dispatch: loop {
        match pc {
            0x8323BBD0 => {
    //   block [0x8323BBD0..0x8323BC2C)
	// 8323BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BBD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BBDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8323BBE0: 4BCE1DE1  bl 0x82f1d9c0
	ctx.lr = 0x8323BBE4;
	sub_82F1D9C0(ctx, base);
	// 8323BBE4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BBE8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323BBEC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BBF0: 396B1424  addi r11, r11, 0x1424
	ctx.r[11].s64 = ctx.r[11].s64 + 5156;
	// 8323BBF4: 38C9A7C8  addi r6, r9, -0x5838
	ctx.r[6].s64 = ctx.r[9].s64 + -22584;
	// 8323BBF8: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 8323BBFC: 9169A7C8  stw r11, -0x5838(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22584 as u32), ctx.r[11].u32 ) };
	// 8323BC00: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 8323BC04: 392A1418  addi r9, r10, 0x1418
	ctx.r[9].s64 = ctx.r[10].s64 + 5144;
	// 8323BC08: 3947D180  addi r10, r7, -0x2e80
	ctx.r[10].s64 = ctx.r[7].s64 + -11904;
	// 8323BC0C: 3968D168  addi r11, r8, -0x2e98
	ctx.r[11].s64 = ctx.r[8].s64 + -11928;
	// 8323BC10: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8323BC14: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323BC18: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323BC1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8323BC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BC30 size=120
    let mut pc: u32 = 0x8323BC30;
    'dispatch: loop {
        match pc {
            0x8323BC30 => {
    //   block [0x8323BC30..0x8323BCA8)
	// 8323BC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BC38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BC3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BC40: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BC44: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BC48: 38CA13A4  addi r6, r10, 0x13a4
	ctx.r[6].s64 = ctx.r[10].s64 + 5028;
	// 8323BC4C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BC50: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BC54: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BC5C: 38A994C4  addi r5, r9, -0x6b3c
	ctx.r[5].s64 = ctx.r[9].s64 + -27452;
	// 8323BC60: 38881424  addi r4, r8, 0x1424
	ctx.r[4].s64 = ctx.r[8].s64 + 5156;
	// 8323BC64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BC68: 3867A7D8  addi r3, r7, -0x5828
	ctx.r[3].s64 = ctx.r[7].s64 + -22568;
	// 8323BC6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BC70: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323BC74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BC80: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BC84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323BC88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BC8C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 8323BC90: 4BC6C539  bl 0x82ea81c8
	ctx.lr = 0x8323BC94;
	sub_82EA81C8(ctx, base);
	// 8323BC94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BCA8 size=120
    let mut pc: u32 = 0x8323BCA8;
    'dispatch: loop {
        match pc {
            0x8323BCA8 => {
    //   block [0x8323BCA8..0x8323BD20)
	// 8323BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BCB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BCB4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BCB8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BCBC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323BCC0: 38CA1440  addi r6, r10, 0x1440
	ctx.r[6].s64 = ctx.r[10].s64 + 5184;
	// 8323BCC4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BCC8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BCCC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BCD4: 38A97CF4  addi r5, r9, 0x7cf4
	ctx.r[5].s64 = ctx.r[9].s64 + 31988;
	// 8323BCD8: 388814AC  addi r4, r8, 0x14ac
	ctx.r[4].s64 = ctx.r[8].s64 + 5292;
	// 8323BCDC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BCE0: 3867A808  addi r3, r7, -0x57f8
	ctx.r[3].s64 = ctx.r[7].s64 + -22520;
	// 8323BCE4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BCE8: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323BCEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BCF8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BD00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BD04: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323BD08: 4BC6C4C1  bl 0x82ea81c8
	ctx.lr = 0x8323BD0C;
	sub_82EA81C8(ctx, base);
	// 8323BD0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BD20 size=108
    let mut pc: u32 = 0x8323BD20;
    'dispatch: loop {
        match pc {
            0x8323BD20 => {
    //   block [0x8323BD20..0x8323BD8C)
	// 8323BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BD2C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BD30: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323BD34: 38EA1980  addi r7, r10, 0x1980
	ctx.r[7].s64 = ctx.r[10].s64 + 6528;
	// 8323BD38: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323BD3C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323BD40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323BD44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BD48: 388919FC  addi r4, r9, 0x19fc
	ctx.r[4].s64 = ctx.r[9].s64 + 6652;
	// 8323BD4C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323BD50: 3868A838  addi r3, r8, -0x57c8
	ctx.r[3].s64 = ctx.r[8].s64 + -22472;
	// 8323BD54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BD5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BD60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BD64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BD68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BD70: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323BD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323BD78: 4BC6C451  bl 0x82ea81c8
	ctx.lr = 0x8323BD7C;
	sub_82EA81C8(ctx, base);
	// 8323BD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323BD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BD90 size=108
    let mut pc: u32 = 0x8323BD90;
    'dispatch: loop {
        match pc {
            0x8323BD90 => {
    //   block [0x8323BD90..0x8323BDFC)
	// 8323BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BD9C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BDA0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323BDA4: 38EA19D4  addi r7, r10, 0x19d4
	ctx.r[7].s64 = ctx.r[10].s64 + 6612;
	// 8323BDA8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323BDAC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8323BDB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323BDB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BDB8: 38891A1C  addi r4, r9, 0x1a1c
	ctx.r[4].s64 = ctx.r[9].s64 + 6684;
	// 8323BDBC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323BDC0: 3868A868  addi r3, r8, -0x5798
	ctx.r[3].s64 = ctx.r[8].s64 + -22424;
	// 8323BDC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BDCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BDD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BDD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BDD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BDE0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323BDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323BDE8: 4BC6C3E1  bl 0x82ea81c8
	ctx.lr = 0x8323BDEC;
	sub_82EA81C8(ctx, base);
	// 8323BDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323BDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323BE00 size=24
    let mut pc: u32 = 0x8323BE00;
    'dispatch: loop {
        match pc {
            0x8323BE00 => {
    //   block [0x8323BE00..0x8323BE18)
	// 8323BE00: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323BE04: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323BE08: 392A1AB8  addi r9, r10, 0x1ab8
	ctx.r[9].s64 = ctx.r[10].s64 + 6840;
	// 8323BE0C: 816B1A80  lwz r11, 0x1a80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6784 as u32) ) } as u64;
	// 8323BE10: 91690038  stw r11, 0x38(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8323BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BE18 size=116
    let mut pc: u32 = 0x8323BE18;
    'dispatch: loop {
        match pc {
            0x8323BE18 => {
    //   block [0x8323BE18..0x8323BE8C)
	// 8323BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BE24: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323BE28: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323BE2C: 38AA1AB8  addi r5, r10, 0x1ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 6840;
	// 8323BE30: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323BE34: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 8323BE38: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323BE3C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8323BE40: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323BE44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BE48: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323BE4C: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 8323BE50: 38871A30  addi r4, r7, 0x1a30
	ctx.r[4].s64 = ctx.r[7].s64 + 6704;
	// 8323BE54: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BE58: 3866A898  addi r3, r6, -0x5768
	ctx.r[3].s64 = ctx.r[6].s64 + -22376;
	// 8323BE5C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BE60: 392919C0  addi r9, r9, 0x19c0
	ctx.r[9].s64 = ctx.r[9].s64 + 6592;
	// 8323BE64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BE68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323BE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BE70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BE74: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323BE78: 4BC6C351  bl 0x82ea81c8
	ctx.lr = 0x8323BE7C;
	sub_82EA81C8(ctx, base);
	// 8323BE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323BE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BE90 size=120
    let mut pc: u32 = 0x8323BE90;
    'dispatch: loop {
        match pc {
            0x8323BE90 => {
    //   block [0x8323BE90..0x8323BF08)
	// 8323BE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BE9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BEA0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BEA4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323BEA8: 38CA1A50  addi r6, r10, 0x1a50
	ctx.r[6].s64 = ctx.r[10].s64 + 6736;
	// 8323BEAC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323BEB0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323BEB4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323BEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BEBC: 38A99974  addi r5, r9, -0x668c
	ctx.r[5].s64 = ctx.r[9].s64 + -26252;
	// 8323BEC0: 38881A68  addi r4, r8, 0x1a68
	ctx.r[4].s64 = ctx.r[8].s64 + 6760;
	// 8323BEC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BEC8: 3867A8C8  addi r3, r7, -0x5738
	ctx.r[3].s64 = ctx.r[7].s64 + -22328;
	// 8323BECC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BED0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323BED4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BEE0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323BEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BEE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BEEC: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 8323BEF0: 4BC6C2D9  bl 0x82ea81c8
	ctx.lr = 0x8323BEF4;
	sub_82EA81C8(ctx, base);
	// 8323BEF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323BEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BF00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323BF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BF08 size=108
    let mut pc: u32 = 0x8323BF08;
    'dispatch: loop {
        match pc {
            0x8323BF08 => {
    //   block [0x8323BF08..0x8323BF74)
	// 8323BF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BF14: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BF18: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323BF1C: 38EA1A7C  addi r7, r10, 0x1a7c
	ctx.r[7].s64 = ctx.r[10].s64 + 6780;
	// 8323BF20: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323BF24: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8323BF28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323BF2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BF30: 38891A94  addi r4, r9, 0x1a94
	ctx.r[4].s64 = ctx.r[9].s64 + 6804;
	// 8323BF34: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323BF38: 3868A8F8  addi r3, r8, -0x5708
	ctx.r[3].s64 = ctx.r[8].s64 + -22280;
	// 8323BF3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BF44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BF48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BF4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BF50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BF58: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8323BF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323BF60: 4BC6C269  bl 0x82ea81c8
	ctx.lr = 0x8323BF64;
	sub_82EA81C8(ctx, base);
	// 8323BF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323BF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BF78 size=108
    let mut pc: u32 = 0x8323BF78;
    'dispatch: loop {
        match pc {
            0x8323BF78 => {
    //   block [0x8323BF78..0x8323BFE4)
	// 8323BF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BF84: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BF88: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323BF8C: 38EA1AD0  addi r7, r10, 0x1ad0
	ctx.r[7].s64 = ctx.r[10].s64 + 6864;
	// 8323BF90: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323BF94: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8323BF98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323BF9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323BFA0: 38891B48  addi r4, r9, 0x1b48
	ctx.r[4].s64 = ctx.r[9].s64 + 6984;
	// 8323BFA4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323BFA8: 3868A928  addi r3, r8, -0x56d8
	ctx.r[3].s64 = ctx.r[8].s64 + -22232;
	// 8323BFAC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323BFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323BFB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323BFB8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323BFBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323BFC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323BFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323BFC8: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8323BFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323BFD0: 4BC6C1F9  bl 0x82ea81c8
	ctx.lr = 0x8323BFD4;
	sub_82EA81C8(ctx, base);
	// 8323BFD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323BFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323BFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323BFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323BFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323BFE8 size=120
    let mut pc: u32 = 0x8323BFE8;
    'dispatch: loop {
        match pc {
            0x8323BFE8 => {
    //   block [0x8323BFE8..0x8323C060)
	// 8323BFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323BFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323BFF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323BFF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323BFF8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323BFFC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323C000: 38CA1B90  addi r6, r10, 0x1b90
	ctx.r[6].s64 = ctx.r[10].s64 + 7056;
	// 8323C004: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C008: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C00C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C014: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323C018: 38881BE8  addi r4, r8, 0x1be8
	ctx.r[4].s64 = ctx.r[8].s64 + 7144;
	// 8323C01C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C020: 3867A958  addi r3, r7, -0x56a8
	ctx.r[3].s64 = ctx.r[7].s64 + -22184;
	// 8323C024: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C028: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323C02C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C038: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C040: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C044: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8323C048: 4BC6C181  bl 0x82ea81c8
	ctx.lr = 0x8323C04C;
	sub_82EA81C8(ctx, base);
	// 8323C04C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C060 size=120
    let mut pc: u32 = 0x8323C060;
    'dispatch: loop {
        match pc {
            0x8323C060 => {
    //   block [0x8323C060..0x8323C0D8)
	// 8323C060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323C06C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C070: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C074: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323C078: 38CA2678  addi r6, r10, 0x2678
	ctx.r[6].s64 = ctx.r[10].s64 + 9848;
	// 8323C07C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C080: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C084: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C08C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323C090: 38882690  addi r4, r8, 0x2690
	ctx.r[4].s64 = ctx.r[8].s64 + 9872;
	// 8323C094: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C098: 3867AB34  addi r3, r7, -0x54cc
	ctx.r[3].s64 = ctx.r[7].s64 + -21708;
	// 8323C09C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C0A0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323C0A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C0B0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C0B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C0BC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323C0C0: 4BC6C109  bl 0x82ea81c8
	ctx.lr = 0x8323C0C4;
	sub_82EA81C8(ctx, base);
	// 8323C0C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C0D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C0D8 size=100
    let mut pc: u32 = 0x8323C0D8;
    'dispatch: loop {
        match pc {
            0x8323C0D8 => {
    //   block [0x8323C0D8..0x8323C13C)
	// 8323C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C0E4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C0E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C0EC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C0F4: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C0F8: 388926A8  addi r4, r9, 0x26a8
	ctx.r[4].s64 = ctx.r[9].s64 + 9896;
	// 8323C0FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C100: 3868AB64  addi r3, r8, -0x549c
	ctx.r[3].s64 = ctx.r[8].s64 + -21660;
	// 8323C104: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C10C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C110: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C114: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C118: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C11C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C120: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C124: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C128: 4BC6C0A1  bl 0x82ea81c8
	ctx.lr = 0x8323C12C;
	sub_82EA81C8(ctx, base);
	// 8323C12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C140 size=100
    let mut pc: u32 = 0x8323C140;
    'dispatch: loop {
        match pc {
            0x8323C140 => {
    //   block [0x8323C140..0x8323C1A4)
	// 8323C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C14C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C150: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C154: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C15C: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C160: 388926C4  addi r4, r9, 0x26c4
	ctx.r[4].s64 = ctx.r[9].s64 + 9924;
	// 8323C164: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C168: 3868AB94  addi r3, r8, -0x546c
	ctx.r[3].s64 = ctx.r[8].s64 + -21612;
	// 8323C16C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C174: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C178: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C17C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C180: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C184: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C18C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C190: 4BC6C039  bl 0x82ea81c8
	ctx.lr = 0x8323C194;
	sub_82EA81C8(ctx, base);
	// 8323C194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C1A8 size=108
    let mut pc: u32 = 0x8323C1A8;
    'dispatch: loop {
        match pc {
            0x8323C1A8 => {
    //   block [0x8323C1A8..0x8323C214)
	// 8323C1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C1B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C1B8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C1BC: 38EA2718  addi r7, r10, 0x2718
	ctx.r[7].s64 = ctx.r[10].s64 + 10008;
	// 8323C1C0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C1C4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323C1C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C1CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C1D0: 38892778  addi r4, r9, 0x2778
	ctx.r[4].s64 = ctx.r[9].s64 + 10104;
	// 8323C1D4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C1D8: 3868ABC4  addi r3, r8, -0x543c
	ctx.r[3].s64 = ctx.r[8].s64 + -21564;
	// 8323C1DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C1E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C1E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C1EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C1F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C1F8: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323C1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C200: 4BC6BFC9  bl 0x82ea81c8
	ctx.lr = 0x8323C204;
	sub_82EA81C8(ctx, base);
	// 8323C204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C218 size=120
    let mut pc: u32 = 0x8323C218;
    'dispatch: loop {
        match pc {
            0x8323C218 => {
    //   block [0x8323C218..0x8323C290)
	// 8323C218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C220: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323C224: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C228: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C22C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323C230: 38CA2760  addi r6, r10, 0x2760
	ctx.r[6].s64 = ctx.r[10].s64 + 10080;
	// 8323C234: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C238: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C23C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C244: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323C248: 388827A8  addi r4, r8, 0x27a8
	ctx.r[4].s64 = ctx.r[8].s64 + 10152;
	// 8323C24C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C250: 3867ABF4  addi r3, r7, -0x540c
	ctx.r[3].s64 = ctx.r[7].s64 + -21516;
	// 8323C254: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C258: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323C25C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C268: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C274: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323C278: 4BC6BF51  bl 0x82ea81c8
	ctx.lr = 0x8323C27C;
	sub_82EA81C8(ctx, base);
	// 8323C27C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C288: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C290 size=100
    let mut pc: u32 = 0x8323C290;
    'dispatch: loop {
        match pc {
            0x8323C290 => {
    //   block [0x8323C290..0x8323C2F4)
	// 8323C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C29C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C2A0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C2A4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C2AC: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C2B0: 388927C0  addi r4, r9, 0x27c0
	ctx.r[4].s64 = ctx.r[9].s64 + 10176;
	// 8323C2B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C2B8: 3868AC24  addi r3, r8, -0x53dc
	ctx.r[3].s64 = ctx.r[8].s64 + -21468;
	// 8323C2BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C2C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C2C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C2CC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C2D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C2D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C2D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C2DC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C2E0: 4BC6BEE9  bl 0x82ea81c8
	ctx.lr = 0x8323C2E4;
	sub_82EA81C8(ctx, base);
	// 8323C2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C2F8 size=100
    let mut pc: u32 = 0x8323C2F8;
    'dispatch: loop {
        match pc {
            0x8323C2F8 => {
    //   block [0x8323C2F8..0x8323C35C)
	// 8323C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C304: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C308: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C30C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C314: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C318: 388927D4  addi r4, r9, 0x27d4
	ctx.r[4].s64 = ctx.r[9].s64 + 10196;
	// 8323C31C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C320: 3868AC54  addi r3, r8, -0x53ac
	ctx.r[3].s64 = ctx.r[8].s64 + -21420;
	// 8323C324: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C32C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C330: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C334: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C338: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C33C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C340: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C344: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C348: 4BC6BE81  bl 0x82ea81c8
	ctx.lr = 0x8323C34C;
	sub_82EA81C8(ctx, base);
	// 8323C34C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C360 size=100
    let mut pc: u32 = 0x8323C360;
    'dispatch: loop {
        match pc {
            0x8323C360 => {
    //   block [0x8323C360..0x8323C3C4)
	// 8323C360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C36C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C370: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C374: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C37C: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C380: 388927E8  addi r4, r9, 0x27e8
	ctx.r[4].s64 = ctx.r[9].s64 + 10216;
	// 8323C384: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C388: 3868AC84  addi r3, r8, -0x537c
	ctx.r[3].s64 = ctx.r[8].s64 + -21372;
	// 8323C38C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C394: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C398: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C39C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C3A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C3A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C3A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C3AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C3B0: 4BC6BE19  bl 0x82ea81c8
	ctx.lr = 0x8323C3B4;
	sub_82EA81C8(ctx, base);
	// 8323C3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C3C8 size=100
    let mut pc: u32 = 0x8323C3C8;
    'dispatch: loop {
        match pc {
            0x8323C3C8 => {
    //   block [0x8323C3C8..0x8323C42C)
	// 8323C3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C3D4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C3D8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C3DC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C3E4: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C3E8: 38892804  addi r4, r9, 0x2804
	ctx.r[4].s64 = ctx.r[9].s64 + 10244;
	// 8323C3EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C3F0: 3868ACB4  addi r3, r8, -0x534c
	ctx.r[3].s64 = ctx.r[8].s64 + -21324;
	// 8323C3F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C3FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C400: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C404: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C40C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C410: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C414: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C418: 4BC6BDB1  bl 0x82ea81c8
	ctx.lr = 0x8323C41C;
	sub_82EA81C8(ctx, base);
	// 8323C41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C430 size=100
    let mut pc: u32 = 0x8323C430;
    'dispatch: loop {
        match pc {
            0x8323C430 => {
    //   block [0x8323C430..0x8323C494)
	// 8323C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C43C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C440: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C444: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C44C: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C450: 3889281C  addi r4, r9, 0x281c
	ctx.r[4].s64 = ctx.r[9].s64 + 10268;
	// 8323C454: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C458: 3868ACE4  addi r3, r8, -0x531c
	ctx.r[3].s64 = ctx.r[8].s64 + -21276;
	// 8323C45C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C464: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C468: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C46C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C474: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C47C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C480: 4BC6BD49  bl 0x82ea81c8
	ctx.lr = 0x8323C484;
	sub_82EA81C8(ctx, base);
	// 8323C484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C498 size=100
    let mut pc: u32 = 0x8323C498;
    'dispatch: loop {
        match pc {
            0x8323C498 => {
    //   block [0x8323C498..0x8323C4FC)
	// 8323C498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C4A4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323C4A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C4AC: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C4B4: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323C4B8: 3889282C  addi r4, r9, 0x282c
	ctx.r[4].s64 = ctx.r[9].s64 + 10284;
	// 8323C4BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C4C0: 3868AD14  addi r3, r8, -0x52ec
	ctx.r[3].s64 = ctx.r[8].s64 + -21228;
	// 8323C4C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C4CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C4D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C4D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323C4D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C4DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323C4E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C4E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323C4E8: 4BC6BCE1  bl 0x82ea81c8
	ctx.lr = 0x8323C4EC;
	sub_82EA81C8(ctx, base);
	// 8323C4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C500 size=108
    let mut pc: u32 = 0x8323C500;
    'dispatch: loop {
        match pc {
            0x8323C500 => {
    //   block [0x8323C500..0x8323C56C)
	// 8323C500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C50C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C510: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C514: 38EA2AC8  addi r7, r10, 0x2ac8
	ctx.r[7].s64 = ctx.r[10].s64 + 10952;
	// 8323C518: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C51C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8323C520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C528: 38892EF4  addi r4, r9, 0x2ef4
	ctx.r[4].s64 = ctx.r[9].s64 + 12020;
	// 8323C52C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C530: 3868AD44  addi r3, r8, -0x52bc
	ctx.r[3].s64 = ctx.r[8].s64 + -21180;
	// 8323C534: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C53C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C540: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C544: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C548: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C550: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8323C554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C558: 4BC6BC71  bl 0x82ea81c8
	ctx.lr = 0x8323C55C;
	sub_82EA81C8(ctx, base);
	// 8323C55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C570 size=120
    let mut pc: u32 = 0x8323C570;
    'dispatch: loop {
        match pc {
            0x8323C570 => {
    //   block [0x8323C570..0x8323C5E8)
	// 8323C570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323C57C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C580: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C584: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323C588: 38CA2C60  addi r6, r10, 0x2c60
	ctx.r[6].s64 = ctx.r[10].s64 + 11360;
	// 8323C58C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C590: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C594: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C59C: 38A97AE4  addi r5, r9, 0x7ae4
	ctx.r[5].s64 = ctx.r[9].s64 + 31460;
	// 8323C5A0: 38882F10  addi r4, r8, 0x2f10
	ctx.r[4].s64 = ctx.r[8].s64 + 12048;
	// 8323C5A4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C5A8: 3867AD74  addi r3, r7, -0x528c
	ctx.r[3].s64 = ctx.r[7].s64 + -21132;
	// 8323C5AC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C5B0: 3BE00019  li r31, 0x19
	ctx.r[31].s64 = 25;
	// 8323C5B4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C5C0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C5C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C5C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C5CC: 38C000D4  li r6, 0xd4
	ctx.r[6].s64 = 212;
	// 8323C5D0: 4BC6BBF9  bl 0x82ea81c8
	ctx.lr = 0x8323C5D4;
	sub_82EA81C8(ctx, base);
	// 8323C5D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C5E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C5E8 size=108
    let mut pc: u32 = 0x8323C5E8;
    'dispatch: loop {
        match pc {
            0x8323C5E8 => {
    //   block [0x8323C5E8..0x8323C654)
	// 8323C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C5F4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C5F8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C5FC: 38EA30A8  addi r7, r10, 0x30a8
	ctx.r[7].s64 = ctx.r[10].s64 + 12456;
	// 8323C600: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C604: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8323C608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C60C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C610: 388932D0  addi r4, r9, 0x32d0
	ctx.r[4].s64 = ctx.r[9].s64 + 13008;
	// 8323C614: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C618: 3868ADA4  addi r3, r8, -0x525c
	ctx.r[3].s64 = ctx.r[8].s64 + -21084;
	// 8323C61C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C624: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C628: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C62C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C630: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C638: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 8323C63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C640: 4BC6BB89  bl 0x82ea81c8
	ctx.lr = 0x8323C644;
	sub_82EA81C8(ctx, base);
	// 8323C644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C658 size=108
    let mut pc: u32 = 0x8323C658;
    'dispatch: loop {
        match pc {
            0x8323C658 => {
    //   block [0x8323C658..0x8323C6C4)
	// 8323C658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C664: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C668: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C66C: 38EA3198  addi r7, r10, 0x3198
	ctx.r[7].s64 = ctx.r[10].s64 + 12696;
	// 8323C670: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C674: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323C678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C67C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C680: 38893300  addi r4, r9, 0x3300
	ctx.r[4].s64 = ctx.r[9].s64 + 13056;
	// 8323C684: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C688: 3868ADD4  addi r3, r8, -0x522c
	ctx.r[3].s64 = ctx.r[8].s64 + -21036;
	// 8323C68C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C694: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C698: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C69C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C6A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C6A8: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8323C6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C6B0: 4BC6BB19  bl 0x82ea81c8
	ctx.lr = 0x8323C6B4;
	sub_82EA81C8(ctx, base);
	// 8323C6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C6C8 size=108
    let mut pc: u32 = 0x8323C6C8;
    'dispatch: loop {
        match pc {
            0x8323C6C8 => {
    //   block [0x8323C6C8..0x8323C734)
	// 8323C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C6D4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C6D8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C6DC: 38EA31E0  addi r7, r10, 0x31e0
	ctx.r[7].s64 = ctx.r[10].s64 + 12768;
	// 8323C6E0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C6E4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 8323C6E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C6F0: 38893320  addi r4, r9, 0x3320
	ctx.r[4].s64 = ctx.r[9].s64 + 13088;
	// 8323C6F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C6F8: 3868AE04  addi r3, r8, -0x51fc
	ctx.r[3].s64 = ctx.r[8].s64 + -20988;
	// 8323C6FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C704: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C708: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C70C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C718: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323C71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C720: 4BC6BAA9  bl 0x82ea81c8
	ctx.lr = 0x8323C724;
	sub_82EA81C8(ctx, base);
	// 8323C724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C738 size=108
    let mut pc: u32 = 0x8323C738;
    'dispatch: loop {
        match pc {
            0x8323C738 => {
    //   block [0x8323C738..0x8323C7A4)
	// 8323C738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C744: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C748: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C74C: 38EA32B8  addi r7, r10, 0x32b8
	ctx.r[7].s64 = ctx.r[10].s64 + 12984;
	// 8323C750: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C754: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8323C758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C75C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C760: 38893344  addi r4, r9, 0x3344
	ctx.r[4].s64 = ctx.r[9].s64 + 13124;
	// 8323C764: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C768: 3868AE34  addi r3, r8, -0x51cc
	ctx.r[3].s64 = ctx.r[8].s64 + -20940;
	// 8323C76C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C774: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C778: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C77C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C780: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C788: 38C00048  li r6, 0x48
	ctx.r[6].s64 = 72;
	// 8323C78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C790: 4BC6BA39  bl 0x82ea81c8
	ctx.lr = 0x8323C794;
	sub_82EA81C8(ctx, base);
	// 8323C794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C7A8 size=108
    let mut pc: u32 = 0x8323C7A8;
    'dispatch: loop {
        match pc {
            0x8323C7A8 => {
    //   block [0x8323C7A8..0x8323C814)
	// 8323C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C7B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C7B8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C7BC: 38EA33A0  addi r7, r10, 0x33a0
	ctx.r[7].s64 = ctx.r[10].s64 + 13216;
	// 8323C7C0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C7C4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323C7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C7CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C7D0: 38893414  addi r4, r9, 0x3414
	ctx.r[4].s64 = ctx.r[9].s64 + 13332;
	// 8323C7D4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C7D8: 3868AE64  addi r3, r8, -0x519c
	ctx.r[3].s64 = ctx.r[8].s64 + -20892;
	// 8323C7DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C7E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C7E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C7EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C7F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C7F8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323C7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C800: 4BC6B9C9  bl 0x82ea81c8
	ctx.lr = 0x8323C804;
	sub_82EA81C8(ctx, base);
	// 8323C804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C818 size=120
    let mut pc: u32 = 0x8323C818;
    'dispatch: loop {
        match pc {
            0x8323C818 => {
    //   block [0x8323C818..0x8323C890)
	// 8323C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323C824: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C828: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C82C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323C830: 38CA33E8  addi r6, r10, 0x33e8
	ctx.r[6].s64 = ctx.r[10].s64 + 13288;
	// 8323C834: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C838: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C83C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C844: 38A9ABF4  addi r5, r9, -0x540c
	ctx.r[5].s64 = ctx.r[9].s64 + -21516;
	// 8323C848: 38883450  addi r4, r8, 0x3450
	ctx.r[4].s64 = ctx.r[8].s64 + 13392;
	// 8323C84C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C850: 3867AE94  addi r3, r7, -0x516c
	ctx.r[3].s64 = ctx.r[7].s64 + -20844;
	// 8323C854: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C858: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323C85C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C868: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C870: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C874: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323C878: 4BC6B951  bl 0x82ea81c8
	ctx.lr = 0x8323C87C;
	sub_82EA81C8(ctx, base);
	// 8323C87C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C890 size=108
    let mut pc: u32 = 0x8323C890;
    'dispatch: loop {
        match pc {
            0x8323C890 => {
    //   block [0x8323C890..0x8323C8FC)
	// 8323C890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C89C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C8A0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C8A4: 38EA34E0  addi r7, r10, 0x34e0
	ctx.r[7].s64 = ctx.r[10].s64 + 13536;
	// 8323C8A8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C8AC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323C8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C8B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C8B8: 3889356C  addi r4, r9, 0x356c
	ctx.r[4].s64 = ctx.r[9].s64 + 13676;
	// 8323C8BC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C8C0: 3868AEC4  addi r3, r8, -0x513c
	ctx.r[3].s64 = ctx.r[8].s64 + -20796;
	// 8323C8C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C8CC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C8D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C8D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C8D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C8E0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323C8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C8E8: 4BC6B8E1  bl 0x82ea81c8
	ctx.lr = 0x8323C8EC;
	sub_82EA81C8(ctx, base);
	// 8323C8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C900 size=120
    let mut pc: u32 = 0x8323C900;
    'dispatch: loop {
        match pc {
            0x8323C900 => {
    //   block [0x8323C900..0x8323C978)
	// 8323C900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323C90C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C910: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C914: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323C918: 38CA3528  addi r6, r10, 0x3528
	ctx.r[6].s64 = ctx.r[10].s64 + 13608;
	// 8323C91C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323C920: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323C924: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323C928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C92C: 38A9ACE4  addi r5, r9, -0x531c
	ctx.r[5].s64 = ctx.r[9].s64 + -21276;
	// 8323C930: 3888359C  addi r4, r8, 0x359c
	ctx.r[4].s64 = ctx.r[8].s64 + 13724;
	// 8323C934: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C938: 3867AEF4  addi r3, r7, -0x510c
	ctx.r[3].s64 = ctx.r[7].s64 + -20748;
	// 8323C93C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C940: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323C944: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C950: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323C954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C958: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C95C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323C960: 4BC6B869  bl 0x82ea81c8
	ctx.lr = 0x8323C964;
	sub_82EA81C8(ctx, base);
	// 8323C964: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323C968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323C974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C978 size=108
    let mut pc: u32 = 0x8323C978;
    'dispatch: loop {
        match pc {
            0x8323C978 => {
    //   block [0x8323C978..0x8323C9E4)
	// 8323C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C984: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323C988: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323C98C: 38EA37A0  addi r7, r10, 0x37a0
	ctx.r[7].s64 = ctx.r[10].s64 + 14240;
	// 8323C990: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323C994: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8323C998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323C99C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323C9A0: 38893ABC  addi r4, r9, 0x3abc
	ctx.r[4].s64 = ctx.r[9].s64 + 15036;
	// 8323C9A4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323C9A8: 3868AF24  addi r3, r8, -0x50dc
	ctx.r[3].s64 = ctx.r[8].s64 + -20700;
	// 8323C9AC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323C9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323C9B4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323C9B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323C9BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323C9C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323C9C8: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8323C9CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323C9D0: 4BC6B7F9  bl 0x82ea81c8
	ctx.lr = 0x8323C9D4;
	sub_82EA81C8(ctx, base);
	// 8323C9D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323C9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323C9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323C9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323C9E8 size=112
    let mut pc: u32 = 0x8323C9E8;
    'dispatch: loop {
        match pc {
            0x8323C9E8 => {
    //   block [0x8323C9E8..0x8323CA58)
	// 8323C9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323C9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323C9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323C9F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323C9F8: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 8323C9FC: 396B3890  addi r11, r11, 0x3890
	ctx.r[11].s64 = ctx.r[11].s64 + 14480;
	// 8323CA00: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323CA04: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8323CA08: 38CB01C8  addi r6, r11, 0x1c8
	ctx.r[6].s64 = ctx.r[11].s64 + 456;
	// 8323CA0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323CA10: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CA14: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CA18: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 8323CA1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CA20: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323CA24: 38883AE0  addi r4, r8, 0x3ae0
	ctx.r[4].s64 = ctx.r[8].s64 + 15072;
	// 8323CA28: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8323CA2C: 3867AF54  addi r3, r7, -0x50ac
	ctx.r[3].s64 = ctx.r[7].s64 + -20652;
	// 8323CA30: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 8323CA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CA38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CA40: 38C001A0  li r6, 0x1a0
	ctx.r[6].s64 = 416;
	// 8323CA44: 4BC6B785  bl 0x82ea81c8
	ctx.lr = 0x8323CA48;
	sub_82EA81C8(ctx, base);
	// 8323CA48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323CA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CA58 size=108
    let mut pc: u32 = 0x8323CA58;
    'dispatch: loop {
        match pc {
            0x8323CA58 => {
    //   block [0x8323CA58..0x8323CAC4)
	// 8323CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CA64: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CA68: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323CA6C: 38EA3B70  addi r7, r10, 0x3b70
	ctx.r[7].s64 = ctx.r[10].s64 + 15216;
	// 8323CA70: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323CA74: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323CA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323CA7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CA80: 38893C58  addi r4, r9, 0x3c58
	ctx.r[4].s64 = ctx.r[9].s64 + 15448;
	// 8323CA84: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323CA88: 3868AF84  addi r3, r8, -0x507c
	ctx.r[3].s64 = ctx.r[8].s64 + -20604;
	// 8323CA8C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CA94: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CA98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CA9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CAA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CAA8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8323CAAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323CAB0: 4BC6B719  bl 0x82ea81c8
	ctx.lr = 0x8323CAB4;
	sub_82EA81C8(ctx, base);
	// 8323CAB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323CAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CAC8 size=120
    let mut pc: u32 = 0x8323CAC8;
    'dispatch: loop {
        match pc {
            0x8323CAC8 => {
    //   block [0x8323CAC8..0x8323CB40)
	// 8323CAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CAD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CAD4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CAD8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CADC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323CAE0: 38CA3BA0  addi r6, r10, 0x3ba0
	ctx.r[6].s64 = ctx.r[10].s64 + 15264;
	// 8323CAE4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CAE8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CAEC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CAF4: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323CAF8: 38883C6C  addi r4, r8, 0x3c6c
	ctx.r[4].s64 = ctx.r[8].s64 + 15468;
	// 8323CAFC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CB00: 3867AFB4  addi r3, r7, -0x504c
	ctx.r[3].s64 = ctx.r[7].s64 + -20556;
	// 8323CB04: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CB08: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323CB0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CB18: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CB20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CB24: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323CB28: 4BC6B6A1  bl 0x82ea81c8
	ctx.lr = 0x8323CB2C;
	sub_82EA81C8(ctx, base);
	// 8323CB2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CB38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CB40 size=120
    let mut pc: u32 = 0x8323CB40;
    'dispatch: loop {
        match pc {
            0x8323CB40 => {
    //   block [0x8323CB40..0x8323CBB8)
	// 8323CB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CB48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CB4C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CB50: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CB54: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323CB58: 38CA3BE8  addi r6, r10, 0x3be8
	ctx.r[6].s64 = ctx.r[10].s64 + 15336;
	// 8323CB5C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323CB60: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CB64: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CB6C: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323CB70: 38880C50  addi r4, r8, 0xc50
	ctx.r[4].s64 = ctx.r[8].s64 + 3152;
	// 8323CB74: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CB78: 3867AFE4  addi r3, r7, -0x501c
	ctx.r[3].s64 = ctx.r[7].s64 + -20508;
	// 8323CB7C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CB80: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323CB84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CB90: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CB98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CB9C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323CBA0: 4BC6B629  bl 0x82ea81c8
	ctx.lr = 0x8323CBA4;
	sub_82EA81C8(ctx, base);
	// 8323CBA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CBB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CBB8 size=120
    let mut pc: u32 = 0x8323CBB8;
    'dispatch: loop {
        match pc {
            0x8323CBB8 => {
    //   block [0x8323CBB8..0x8323CC30)
	// 8323CBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CBC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CBC4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CBC8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CBCC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323CBD0: 38CA3CC8  addi r6, r10, 0x3cc8
	ctx.r[6].s64 = ctx.r[10].s64 + 15560;
	// 8323CBD4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CBD8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CBDC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CBE4: 38A96DFC  addi r5, r9, 0x6dfc
	ctx.r[5].s64 = ctx.r[9].s64 + 28156;
	// 8323CBE8: 38883D78  addi r4, r8, 0x3d78
	ctx.r[4].s64 = ctx.r[8].s64 + 15736;
	// 8323CBEC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CBF0: 3867B014  addi r3, r7, -0x4fec
	ctx.r[3].s64 = ctx.r[7].s64 + -20460;
	// 8323CBF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CBF8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323CBFC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CC08: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CC0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8323CC10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CC14: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8323CC18: 4BC6B5B1  bl 0x82ea81c8
	ctx.lr = 0x8323CC1C;
	sub_82EA81C8(ctx, base);
	// 8323CC1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CC28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CC30 size=108
    let mut pc: u32 = 0x8323CC30;
    'dispatch: loop {
        match pc {
            0x8323CC30 => {
    //   block [0x8323CC30..0x8323CC9C)
	// 8323CC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CC38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CC3C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323CC40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323CC44: 392B3D4C  addi r9, r11, 0x3d4c
	ctx.r[9].s64 = ctx.r[11].s64 + 15692;
	// 8323CC48: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8323CC4C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8323CC50: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8323CC54: 4BD6E135  bl 0x82faad88
	ctx.lr = 0x8323CC58;
	sub_82FAAD88(ctx, base);
	// 8323CC58: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323CC5C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323CC60: 38E8B044  addi r7, r8, -0x4fbc
	ctx.r[7].s64 = ctx.r[8].s64 + -20412;
	// 8323CC64: 396B3D94  addi r11, r11, 0x3d94
	ctx.r[11].s64 = ctx.r[11].s64 + 15764;
	// 8323CC68: 3D4082FA  lis r10, -0x7d06
	ctx.r[10].s64 = -2097545216;
	// 8323CC6C: 9168B044  stw r11, -0x4fbc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-20412 as u32), ctx.r[11].u32 ) };
	// 8323CC70: 3D2082FA  lis r9, -0x7d06
	ctx.r[9].s64 = -2097545216;
	// 8323CC74: 394A68A8  addi r10, r10, 0x68a8
	ctx.r[10].s64 = ctx.r[10].s64 + 26792;
	// 8323CC78: 39296848  addi r9, r9, 0x6848
	ctx.r[9].s64 = ctx.r[9].s64 + 26696;
	// 8323CC7C: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8323CC80: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8323CC84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8323CC88: 9167000C  stw r11, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8323CC8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CCA0 size=120
    let mut pc: u32 = 0x8323CCA0;
    'dispatch: loop {
        match pc {
            0x8323CCA0 => {
    //   block [0x8323CCA0..0x8323CD18)
	// 8323CCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CCA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CCAC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CCB0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CCB4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CCB8: 38CA3CE0  addi r6, r10, 0x3ce0
	ctx.r[6].s64 = ctx.r[10].s64 + 15584;
	// 8323CCBC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CCC0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CCC4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CCCC: 38A9AB34  addi r5, r9, -0x54cc
	ctx.r[5].s64 = ctx.r[9].s64 + -21708;
	// 8323CCD0: 38883D94  addi r4, r8, 0x3d94
	ctx.r[4].s64 = ctx.r[8].s64 + 15764;
	// 8323CCD4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CCD8: 3867B054  addi r3, r7, -0x4fac
	ctx.r[3].s64 = ctx.r[7].s64 + -20396;
	// 8323CCDC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CCE0: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323CCE4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CCF0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CCF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CCFC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323CD00: 4BC6B4C9  bl 0x82ea81c8
	ctx.lr = 0x8323CD04;
	sub_82EA81C8(ctx, base);
	// 8323CD04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CD10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CD18 size=120
    let mut pc: u32 = 0x8323CD18;
    'dispatch: loop {
        match pc {
            0x8323CD18 => {
    //   block [0x8323CD18..0x8323CD90)
	// 8323CD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CD20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CD24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CD28: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CD2C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CD30: 38CA3E38  addi r6, r10, 0x3e38
	ctx.r[6].s64 = ctx.r[10].s64 + 15928;
	// 8323CD34: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CD38: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CD3C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CD44: 38A9AC84  addi r5, r9, -0x537c
	ctx.r[5].s64 = ctx.r[9].s64 + -21372;
	// 8323CD48: 38883F30  addi r4, r8, 0x3f30
	ctx.r[4].s64 = ctx.r[8].s64 + 16176;
	// 8323CD4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CD50: 3867B084  addi r3, r7, -0x4f7c
	ctx.r[3].s64 = ctx.r[7].s64 + -20348;
	// 8323CD54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CD58: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323CD5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CD68: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CD70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CD74: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323CD78: 4BC6B451  bl 0x82ea81c8
	ctx.lr = 0x8323CD7C;
	sub_82EA81C8(ctx, base);
	// 8323CD7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CD88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CD90 size=120
    let mut pc: u32 = 0x8323CD90;
    'dispatch: loop {
        match pc {
            0x8323CD90 => {
    //   block [0x8323CD90..0x8323CE08)
	// 8323CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CD98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CD9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CDA0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CDA4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CDA8: 38CA3E98  addi r6, r10, 0x3e98
	ctx.r[6].s64 = ctx.r[10].s64 + 16024;
	// 8323CDAC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CDB0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CDB4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CDBC: 38A9ACB4  addi r5, r9, -0x534c
	ctx.r[5].s64 = ctx.r[9].s64 + -21324;
	// 8323CDC0: 38883F54  addi r4, r8, 0x3f54
	ctx.r[4].s64 = ctx.r[8].s64 + 16212;
	// 8323CDC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CDC8: 3867B0B4  addi r3, r7, -0x4f4c
	ctx.r[3].s64 = ctx.r[7].s64 + -20300;
	// 8323CDCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CDD0: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323CDD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CDDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CDE0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CDE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CDEC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323CDF0: 4BC6B3D9  bl 0x82ea81c8
	ctx.lr = 0x8323CDF4;
	sub_82EA81C8(ctx, base);
	// 8323CDF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CE00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CE08 size=120
    let mut pc: u32 = 0x8323CE08;
    'dispatch: loop {
        match pc {
            0x8323CE08 => {
    //   block [0x8323CE08..0x8323CE80)
	// 8323CE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CE10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CE14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CE18: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CE1C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CE20: 38CA3FB8  addi r6, r10, 0x3fb8
	ctx.r[6].s64 = ctx.r[10].s64 + 16312;
	// 8323CE24: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CE28: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CE2C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CE34: 38A9AB64  addi r5, r9, -0x549c
	ctx.r[5].s64 = ctx.r[9].s64 + -21660;
	// 8323CE38: 38884014  addi r4, r8, 0x4014
	ctx.r[4].s64 = ctx.r[8].s64 + 16404;
	// 8323CE3C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CE40: 3867B0E4  addi r3, r7, -0x4f1c
	ctx.r[3].s64 = ctx.r[7].s64 + -20252;
	// 8323CE44: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CE48: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323CE4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CE58: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CE60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CE64: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323CE68: 4BC6B361  bl 0x82ea81c8
	ctx.lr = 0x8323CE6C;
	sub_82EA81C8(ctx, base);
	// 8323CE6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CE78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CE80 size=120
    let mut pc: u32 = 0x8323CE80;
    'dispatch: loop {
        match pc {
            0x8323CE80 => {
    //   block [0x8323CE80..0x8323CEF8)
	// 8323CE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CE88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CE8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CE90: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CE94: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CE98: 38CA40B0  addi r6, r10, 0x40b0
	ctx.r[6].s64 = ctx.r[10].s64 + 16560;
	// 8323CE9C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CEA0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CEA4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CEAC: 38A9AB94  addi r5, r9, -0x546c
	ctx.r[5].s64 = ctx.r[9].s64 + -21612;
	// 8323CEB0: 38884180  addi r4, r8, 0x4180
	ctx.r[4].s64 = ctx.r[8].s64 + 16768;
	// 8323CEB4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CEB8: 3867B114  addi r3, r7, -0x4eec
	ctx.r[3].s64 = ctx.r[7].s64 + -20204;
	// 8323CEBC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CEC0: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 8323CEC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CED0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CED8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CEDC: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8323CEE0: 4BC6B2E9  bl 0x82ea81c8
	ctx.lr = 0x8323CEE4;
	sub_82EA81C8(ctx, base);
	// 8323CEE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CEF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CEF8 size=120
    let mut pc: u32 = 0x8323CEF8;
    'dispatch: loop {
        match pc {
            0x8323CEF8 => {
    //   block [0x8323CEF8..0x8323CF70)
	// 8323CEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CF00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CF04: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CF08: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CF0C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CF10: 38CA41E0  addi r6, r10, 0x41e0
	ctx.r[6].s64 = ctx.r[10].s64 + 16864;
	// 8323CF14: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CF18: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CF1C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CF24: 38A9AC24  addi r5, r9, -0x53dc
	ctx.r[5].s64 = ctx.r[9].s64 + -21468;
	// 8323CF28: 38884240  addi r4, r8, 0x4240
	ctx.r[4].s64 = ctx.r[8].s64 + 16960;
	// 8323CF2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CF30: 3867B144  addi r3, r7, -0x4ebc
	ctx.r[3].s64 = ctx.r[7].s64 + -20156;
	// 8323CF34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CF38: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8323CF3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CF48: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CF50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CF54: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323CF58: 4BC6B271  bl 0x82ea81c8
	ctx.lr = 0x8323CF5C;
	sub_82EA81C8(ctx, base);
	// 8323CF5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CF70 size=120
    let mut pc: u32 = 0x8323CF70;
    'dispatch: loop {
        match pc {
            0x8323CF70 => {
    //   block [0x8323CF70..0x8323CFE8)
	// 8323CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CF78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CF7C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CF80: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CF84: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323CF88: 38CA4318  addi r6, r10, 0x4318
	ctx.r[6].s64 = ctx.r[10].s64 + 17176;
	// 8323CF8C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323CF90: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323CF94: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323CF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323CF9C: 38A9AC54  addi r5, r9, -0x53ac
	ctx.r[5].s64 = ctx.r[9].s64 + -21420;
	// 8323CFA0: 3888441C  addi r4, r8, 0x441c
	ctx.r[4].s64 = ctx.r[8].s64 + 17436;
	// 8323CFA4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323CFA8: 3867B174  addi r3, r7, -0x4e8c
	ctx.r[3].s64 = ctx.r[7].s64 + -20108;
	// 8323CFAC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323CFB0: 3BE0000A  li r31, 0xa
	ctx.r[31].s64 = 10;
	// 8323CFB4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323CFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323CFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323CFC0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323CFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323CFC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323CFCC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323CFD0: 4BC6B1F9  bl 0x82ea81c8
	ctx.lr = 0x8323CFD4;
	sub_82EA81C8(ctx, base);
	// 8323CFD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323CFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323CFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323CFE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323CFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323CFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323CFE8 size=120
    let mut pc: u32 = 0x8323CFE8;
    'dispatch: loop {
        match pc {
            0x8323CFE8 => {
    //   block [0x8323CFE8..0x8323D060)
	// 8323CFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323CFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323CFF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323CFF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323CFF8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323CFFC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323D000: 38CA4480  addi r6, r10, 0x4480
	ctx.r[6].s64 = ctx.r[10].s64 + 17536;
	// 8323D004: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323D008: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323D00C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323D010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D014: 38A9AD14  addi r5, r9, -0x52ec
	ctx.r[5].s64 = ctx.r[9].s64 + -21228;
	// 8323D018: 38884514  addi r4, r8, 0x4514
	ctx.r[4].s64 = ctx.r[8].s64 + 17684;
	// 8323D01C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D020: 3867B1A4  addi r3, r7, -0x4e5c
	ctx.r[3].s64 = ctx.r[7].s64 + -20060;
	// 8323D024: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D028: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 8323D02C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D038: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323D03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D040: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D044: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323D048: 4BC6B181  bl 0x82ea81c8
	ctx.lr = 0x8323D04C;
	sub_82EA81C8(ctx, base);
	// 8323D04C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323D050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323D05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323D060 size=24
    let mut pc: u32 = 0x8323D060;
    'dispatch: loop {
        match pc {
            0x8323D060 => {
    //   block [0x8323D060..0x8323D078)
	// 8323D060: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323D064: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323D068: 392A2548  addi r9, r10, 0x2548
	ctx.r[9].s64 = ctx.r[10].s64 + 9544;
	// 8323D06C: 816B2540  lwz r11, 0x2540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9536 as u32) ) } as u64;
	// 8323D070: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8323D074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D078 size=116
    let mut pc: u32 = 0x8323D078;
    'dispatch: loop {
        match pc {
            0x8323D078 => {
    //   block [0x8323D078..0x8323D0EC)
	// 8323D078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D084: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323D088: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 8323D08C: 38AA2548  addi r5, r10, 0x2548
	ctx.r[5].s64 = ctx.r[10].s64 + 9544;
	// 8323D090: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 8323D094: 3CC08339  lis r6, -0x7cc7
	ctx.r[6].s64 = -2093416448;
	// 8323D098: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8323D09C: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8323D0A0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D0A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D0A8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323D0AC: 38A86DFC  addi r5, r8, 0x6dfc
	ctx.r[5].s64 = ctx.r[8].s64 + 28156;
	// 8323D0B0: 38874740  addi r4, r7, 0x4740
	ctx.r[4].s64 = ctx.r[7].s64 + 18240;
	// 8323D0B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D0B8: 3866B1D4  addi r3, r6, -0x4e2c
	ctx.r[3].s64 = ctx.r[6].s64 + -20012;
	// 8323D0BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D0C0: 392946D0  addi r9, r9, 0x46d0
	ctx.r[9].s64 = ctx.r[9].s64 + 18128;
	// 8323D0C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D0C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323D0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D0D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D0D4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323D0D8: 4BC6B0F1  bl 0x82ea81c8
	ctx.lr = 0x8323D0DC;
	sub_82EA81C8(ctx, base);
	// 8323D0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D0F0 size=100
    let mut pc: u32 = 0x8323D0F0;
    'dispatch: loop {
        match pc {
            0x8323D0F0 => {
    //   block [0x8323D0F0..0x8323D154)
	// 8323D0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D0FC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323D100: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D104: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D10C: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323D110: 38894758  addi r4, r9, 0x4758
	ctx.r[4].s64 = ctx.r[9].s64 + 18264;
	// 8323D114: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D118: 3868B204  addi r3, r8, -0x4dfc
	ctx.r[3].s64 = ctx.r[8].s64 + -19964;
	// 8323D11C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D124: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D128: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D12C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8323D130: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D134: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8323D138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D13C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323D140: 4BC6B089  bl 0x82ea81c8
	ctx.lr = 0x8323D144;
	sub_82EA81C8(ctx, base);
	// 8323D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D158 size=108
    let mut pc: u32 = 0x8323D158;
    'dispatch: loop {
        match pc {
            0x8323D158 => {
    //   block [0x8323D158..0x8323D1C4)
	// 8323D158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D164: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D168: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D16C: 38EA4908  addi r7, r10, 0x4908
	ctx.r[7].s64 = ctx.r[10].s64 + 18696;
	// 8323D170: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D174: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8323D178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D17C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D180: 38894CA8  addi r4, r9, 0x4ca8
	ctx.r[4].s64 = ctx.r[9].s64 + 19624;
	// 8323D184: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D188: 3868B234  addi r3, r8, -0x4dcc
	ctx.r[3].s64 = ctx.r[8].s64 + -19916;
	// 8323D18C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D194: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D198: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D19C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D1A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D1A8: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323D1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D1B0: 4BC6B019  bl 0x82ea81c8
	ctx.lr = 0x8323D1B4;
	sub_82EA81C8(ctx, base);
	// 8323D1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D1C8 size=108
    let mut pc: u32 = 0x8323D1C8;
    'dispatch: loop {
        match pc {
            0x8323D1C8 => {
    //   block [0x8323D1C8..0x8323D234)
	// 8323D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D1D4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D1D8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D1DC: 38EA49F8  addi r7, r10, 0x49f8
	ctx.r[7].s64 = ctx.r[10].s64 + 18936;
	// 8323D1E0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D1E4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8323D1E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D1EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D1F0: 38894CD8  addi r4, r9, 0x4cd8
	ctx.r[4].s64 = ctx.r[9].s64 + 19672;
	// 8323D1F4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D1F8: 3868B264  addi r3, r8, -0x4d9c
	ctx.r[3].s64 = ctx.r[8].s64 + -19868;
	// 8323D1FC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D204: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D208: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D20C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D218: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323D21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D220: 4BC6AFA9  bl 0x82ea81c8
	ctx.lr = 0x8323D224;
	sub_82EA81C8(ctx, base);
	// 8323D224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D238 size=120
    let mut pc: u32 = 0x8323D238;
    'dispatch: loop {
        match pc {
            0x8323D238 => {
    //   block [0x8323D238..0x8323D2B0)
	// 8323D238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323D244: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D248: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D24C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323D250: 38CA4A88  addi r6, r10, 0x4a88
	ctx.r[6].s64 = ctx.r[10].s64 + 19080;
	// 8323D254: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323D258: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323D25C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323D260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D264: 38A9B1D4  addi r5, r9, -0x4e2c
	ctx.r[5].s64 = ctx.r[9].s64 + -20012;
	// 8323D268: 38884D08  addi r4, r8, 0x4d08
	ctx.r[4].s64 = ctx.r[8].s64 + 19720;
	// 8323D26C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D270: 3867B294  addi r3, r7, -0x4d6c
	ctx.r[3].s64 = ctx.r[7].s64 + -19820;
	// 8323D274: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D278: 3BE0000A  li r31, 0xa
	ctx.r[31].s64 = 10;
	// 8323D27C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D288: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323D28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D290: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D294: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8323D298: 4BC6AF31  bl 0x82ea81c8
	ctx.lr = 0x8323D29C;
	sub_82EA81C8(ctx, base);
	// 8323D29C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323D2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323D2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323D2B0 size=44
    let mut pc: u32 = 0x8323D2B0;
    'dispatch: loop {
        match pc {
            0x8323D2B0 => {
    //   block [0x8323D2B0..0x8323D2DC)
	// 8323D2B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323D2B4: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 8323D2B8: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 8323D2BC: 38E82690  addi r7, r8, 0x2690
	ctx.r[7].s64 = ctx.r[8].s64 + 9872;
	// 8323D2C0: 814B2658  lwz r10, 0x2658(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8323D2C4: 8169265C  lwz r11, 0x265c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9820 as u32) ) } as u64;
	// 8323D2C8: 914700C8  stw r10, 0xc8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 8323D2CC: 916700E0  stw r11, 0xe0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8323D2D0: 916700F8  stw r11, 0xf8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 8323D2D4: 91670110  stw r11, 0x110(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8323D2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D2E0 size=112
    let mut pc: u32 = 0x8323D2E0;
    'dispatch: loop {
        match pc {
            0x8323D2E0 => {
    //   block [0x8323D2E0..0x8323D350)
	// 8323D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D2EC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323D2F0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323D2F4: 38CA2690  addi r6, r10, 0x2690
	ctx.r[6].s64 = ctx.r[10].s64 + 9872;
	// 8323D2F8: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323D2FC: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8323D300: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323D304: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D30C: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8323D310: 38885120  addi r4, r8, 0x5120
	ctx.r[4].s64 = ctx.r[8].s64 + 20768;
	// 8323D314: 3867B2C4  addi r3, r7, -0x4d3c
	ctx.r[3].s64 = ctx.r[7].s64 + -19772;
	// 8323D318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D31C: 39294F58  addi r9, r9, 0x4f58
	ctx.r[9].s64 = ctx.r[9].s64 + 20312;
	// 8323D320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D324: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8323D328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D330: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D334: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8323D338: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D33C: 4BC6AE8D  bl 0x82ea81c8
	ctx.lr = 0x8323D340;
	sub_82EA81C8(ctx, base);
	// 8323D340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D350 size=108
    let mut pc: u32 = 0x8323D350;
    'dispatch: loop {
        match pc {
            0x8323D350 => {
    //   block [0x8323D350..0x8323D3BC)
	// 8323D350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D35C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D360: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D364: 38EA4F80  addi r7, r10, 0x4f80
	ctx.r[7].s64 = ctx.r[10].s64 + 20352;
	// 8323D368: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D36C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323D370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D374: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D378: 388951A0  addi r4, r9, 0x51a0
	ctx.r[4].s64 = ctx.r[9].s64 + 20896;
	// 8323D37C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D380: 3868B2F4  addi r3, r8, -0x4d0c
	ctx.r[3].s64 = ctx.r[8].s64 + -19724;
	// 8323D384: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D38C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D390: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D394: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D3A0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8323D3A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D3A8: 4BC6AE21  bl 0x82ea81c8
	ctx.lr = 0x8323D3AC;
	sub_82EA81C8(ctx, base);
	// 8323D3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D3C0 size=120
    let mut pc: u32 = 0x8323D3C0;
    'dispatch: loop {
        match pc {
            0x8323D3C0 => {
    //   block [0x8323D3C0..0x8323D438)
	// 8323D3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D3C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323D3CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D3D0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D3D4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323D3D8: 38CA4FB0  addi r6, r10, 0x4fb0
	ctx.r[6].s64 = ctx.r[10].s64 + 20400;
	// 8323D3DC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323D3E0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323D3E4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323D3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D3EC: 38A9B1D4  addi r5, r9, -0x4e2c
	ctx.r[5].s64 = ctx.r[9].s64 + -20012;
	// 8323D3F0: 388851D8  addi r4, r8, 0x51d8
	ctx.r[4].s64 = ctx.r[8].s64 + 20952;
	// 8323D3F4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D3F8: 3867B324  addi r3, r7, -0x4cdc
	ctx.r[3].s64 = ctx.r[7].s64 + -19676;
	// 8323D3FC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D400: 3BE0000D  li r31, 0xd
	ctx.r[31].s64 = 13;
	// 8323D404: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D410: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323D414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D418: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D41C: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8323D420: 4BC6ADA9  bl 0x82ea81c8
	ctx.lr = 0x8323D424;
	sub_82EA81C8(ctx, base);
	// 8323D424: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323D428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323D434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D438 size=108
    let mut pc: u32 = 0x8323D438;
    'dispatch: loop {
        match pc {
            0x8323D438 => {
    //   block [0x8323D438..0x8323D4A4)
	// 8323D438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D444: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D448: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D44C: 38EA52D0  addi r7, r10, 0x52d0
	ctx.r[7].s64 = ctx.r[10].s64 + 21200;
	// 8323D450: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D454: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323D458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D45C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D460: 38895438  addi r4, r9, 0x5438
	ctx.r[4].s64 = ctx.r[9].s64 + 21560;
	// 8323D464: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D468: 3868B354  addi r3, r8, -0x4cac
	ctx.r[3].s64 = ctx.r[8].s64 + -19628;
	// 8323D46C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D474: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D478: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D47C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D480: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D488: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323D48C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D490: 4BC6AD39  bl 0x82ea81c8
	ctx.lr = 0x8323D494;
	sub_82EA81C8(ctx, base);
	// 8323D494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D4A8 size=108
    let mut pc: u32 = 0x8323D4A8;
    'dispatch: loop {
        match pc {
            0x8323D4A8 => {
    //   block [0x8323D4A8..0x8323D514)
	// 8323D4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D4B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D4B8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D4BC: 38EA5318  addi r7, r10, 0x5318
	ctx.r[7].s64 = ctx.r[10].s64 + 21272;
	// 8323D4C0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D4C4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8323D4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D4CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D4D0: 3889545C  addi r4, r9, 0x545c
	ctx.r[4].s64 = ctx.r[9].s64 + 21596;
	// 8323D4D4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D4D8: 3868B384  addi r3, r8, -0x4c7c
	ctx.r[3].s64 = ctx.r[8].s64 + -19580;
	// 8323D4DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D4E4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D4E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D4EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D4F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D4F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D4F8: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8323D4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D500: 4BC6ACC9  bl 0x82ea81c8
	ctx.lr = 0x8323D504;
	sub_82EA81C8(ctx, base);
	// 8323D504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D518 size=108
    let mut pc: u32 = 0x8323D518;
    'dispatch: loop {
        match pc {
            0x8323D518 => {
    //   block [0x8323D518..0x8323D584)
	// 8323D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D524: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D528: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D52C: 38EA53A8  addi r7, r10, 0x53a8
	ctx.r[7].s64 = ctx.r[10].s64 + 21416;
	// 8323D530: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D534: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8323D538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D53C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D540: 38895480  addi r4, r9, 0x5480
	ctx.r[4].s64 = ctx.r[9].s64 + 21632;
	// 8323D544: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D548: 3868B3B4  addi r3, r8, -0x4c4c
	ctx.r[3].s64 = ctx.r[8].s64 + -19532;
	// 8323D54C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D554: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D558: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D55C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D568: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323D56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D570: 4BC6AC59  bl 0x82ea81c8
	ctx.lr = 0x8323D574;
	sub_82EA81C8(ctx, base);
	// 8323D574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D588 size=108
    let mut pc: u32 = 0x8323D588;
    'dispatch: loop {
        match pc {
            0x8323D588 => {
    //   block [0x8323D588..0x8323D5F4)
	// 8323D588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D594: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D598: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D59C: 38EA54D0  addi r7, r10, 0x54d0
	ctx.r[7].s64 = ctx.r[10].s64 + 21712;
	// 8323D5A0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D5A4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8323D5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D5AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D5B0: 38895548  addi r4, r9, 0x5548
	ctx.r[4].s64 = ctx.r[9].s64 + 21832;
	// 8323D5B4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D5B8: 3868B3E4  addi r3, r8, -0x4c1c
	ctx.r[3].s64 = ctx.r[8].s64 + -19484;
	// 8323D5BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D5C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D5C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D5C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D5CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D5D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D5D8: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323D5DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D5E0: 4BC6ABE9  bl 0x82ea81c8
	ctx.lr = 0x8323D5E4;
	sub_82EA81C8(ctx, base);
	// 8323D5E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D5E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D5EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D5F8 size=108
    let mut pc: u32 = 0x8323D5F8;
    'dispatch: loop {
        match pc {
            0x8323D5F8 => {
    //   block [0x8323D5F8..0x8323D664)
	// 8323D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D604: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D608: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D60C: 38EA5580  addi r7, r10, 0x5580
	ctx.r[7].s64 = ctx.r[10].s64 + 21888;
	// 8323D610: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D614: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8323D618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D61C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D620: 388955F8  addi r4, r9, 0x55f8
	ctx.r[4].s64 = ctx.r[9].s64 + 22008;
	// 8323D624: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D628: 3868B414  addi r3, r8, -0x4bec
	ctx.r[3].s64 = ctx.r[8].s64 + -19436;
	// 8323D62C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D634: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D638: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D63C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D640: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D648: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323D64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D650: 4BC6AB79  bl 0x82ea81c8
	ctx.lr = 0x8323D654;
	sub_82EA81C8(ctx, base);
	// 8323D654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D668 size=108
    let mut pc: u32 = 0x8323D668;
    'dispatch: loop {
        match pc {
            0x8323D668 => {
    //   block [0x8323D668..0x8323D6D4)
	// 8323D668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D674: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D678: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D67C: 38EA5598  addi r7, r10, 0x5598
	ctx.r[7].s64 = ctx.r[10].s64 + 21912;
	// 8323D680: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D684: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8323D688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D68C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D690: 38895610  addi r4, r9, 0x5610
	ctx.r[4].s64 = ctx.r[9].s64 + 22032;
	// 8323D694: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D698: 3868B444  addi r3, r8, -0x4bbc
	ctx.r[3].s64 = ctx.r[8].s64 + -19388;
	// 8323D69C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D6A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D6A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D6AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D6B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D6B8: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323D6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D6C0: 4BC6AB09  bl 0x82ea81c8
	ctx.lr = 0x8323D6C4;
	sub_82EA81C8(ctx, base);
	// 8323D6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D6D8 size=108
    let mut pc: u32 = 0x8323D6D8;
    'dispatch: loop {
        match pc {
            0x8323D6D8 => {
    //   block [0x8323D6D8..0x8323D744)
	// 8323D6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D6E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D6E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D6EC: 38EA5660  addi r7, r10, 0x5660
	ctx.r[7].s64 = ctx.r[10].s64 + 22112;
	// 8323D6F0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D6F4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8323D6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D6FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D700: 3889584C  addi r4, r9, 0x584c
	ctx.r[4].s64 = ctx.r[9].s64 + 22604;
	// 8323D704: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D708: 3868B474  addi r3, r8, -0x4b8c
	ctx.r[3].s64 = ctx.r[8].s64 + -19340;
	// 8323D70C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D714: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D718: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D71C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D728: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323D72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D730: 4BC6AA99  bl 0x82ea81c8
	ctx.lr = 0x8323D734;
	sub_82EA81C8(ctx, base);
	// 8323D734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D748 size=120
    let mut pc: u32 = 0x8323D748;
    'dispatch: loop {
        match pc {
            0x8323D748 => {
    //   block [0x8323D748..0x8323D7C0)
	// 8323D748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323D754: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D758: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D75C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323D760: 38CA56F0  addi r6, r10, 0x56f0
	ctx.r[6].s64 = ctx.r[10].s64 + 22256;
	// 8323D764: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323D768: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323D76C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323D770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D774: 38A9B1D4  addi r5, r9, -0x4e2c
	ctx.r[5].s64 = ctx.r[9].s64 + -20012;
	// 8323D778: 38885884  addi r4, r8, 0x5884
	ctx.r[4].s64 = ctx.r[8].s64 + 22660;
	// 8323D77C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D780: 3867B4A4  addi r3, r7, -0x4b5c
	ctx.r[3].s64 = ctx.r[7].s64 + -19292;
	// 8323D784: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D788: 3BE0000C  li r31, 0xc
	ctx.r[31].s64 = 12;
	// 8323D78C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D798: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323D79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D7A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D7A4: 38C00068  li r6, 0x68
	ctx.r[6].s64 = 104;
	// 8323D7A8: 4BC6AA21  bl 0x82ea81c8
	ctx.lr = 0x8323D7AC;
	sub_82EA81C8(ctx, base);
	// 8323D7AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323D7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D7B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D7C0 size=108
    let mut pc: u32 = 0x8323D7C0;
    'dispatch: loop {
        match pc {
            0x8323D7C0 => {
    //   block [0x8323D7C0..0x8323D82C)
	// 8323D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D7CC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D7D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D7D4: 38EA58D8  addi r7, r10, 0x58d8
	ctx.r[7].s64 = ctx.r[10].s64 + 22744;
	// 8323D7D8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D7DC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8323D7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D7E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D7E8: 38895938  addi r4, r9, 0x5938
	ctx.r[4].s64 = ctx.r[9].s64 + 22840;
	// 8323D7EC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D7F0: 3868B4D4  addi r3, r8, -0x4b2c
	ctx.r[3].s64 = ctx.r[8].s64 + -19244;
	// 8323D7F4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D7FC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D800: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D804: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D810: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8323D814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D818: 4BC6A9B1  bl 0x82ea81c8
	ctx.lr = 0x8323D81C;
	sub_82EA81C8(ctx, base);
	// 8323D81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D830 size=108
    let mut pc: u32 = 0x8323D830;
    'dispatch: loop {
        match pc {
            0x8323D830 => {
    //   block [0x8323D830..0x8323D89C)
	// 8323D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D83C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D840: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D844: 38EA595C  addi r7, r10, 0x595c
	ctx.r[7].s64 = ctx.r[10].s64 + 22876;
	// 8323D848: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D84C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323D850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D854: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D858: 3889598C  addi r4, r9, 0x598c
	ctx.r[4].s64 = ctx.r[9].s64 + 22924;
	// 8323D85C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D860: 3868B504  addi r3, r8, -0x4afc
	ctx.r[3].s64 = ctx.r[8].s64 + -19196;
	// 8323D864: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D86C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D870: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D874: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D878: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D880: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323D884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D888: 4BC6A941  bl 0x82ea81c8
	ctx.lr = 0x8323D88C;
	sub_82EA81C8(ctx, base);
	// 8323D88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D8A0 size=108
    let mut pc: u32 = 0x8323D8A0;
    'dispatch: loop {
        match pc {
            0x8323D8A0 => {
    //   block [0x8323D8A0..0x8323D90C)
	// 8323D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D8AC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D8B0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D8B4: 38EA599C  addi r7, r10, 0x599c
	ctx.r[7].s64 = ctx.r[10].s64 + 22940;
	// 8323D8B8: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D8BC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323D8C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D8C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D8C8: 388959FC  addi r4, r9, 0x59fc
	ctx.r[4].s64 = ctx.r[9].s64 + 23036;
	// 8323D8CC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D8D0: 3868B534  addi r3, r8, -0x4acc
	ctx.r[3].s64 = ctx.r[8].s64 + -19148;
	// 8323D8D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D8DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D8E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D8E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D8E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D8F0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323D8F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D8F8: 4BC6A8D1  bl 0x82ea81c8
	ctx.lr = 0x8323D8FC;
	sub_82EA81C8(ctx, base);
	// 8323D8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D910 size=108
    let mut pc: u32 = 0x8323D910;
    'dispatch: loop {
        match pc {
            0x8323D910 => {
    //   block [0x8323D910..0x8323D97C)
	// 8323D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D91C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D920: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D924: 38EA59CC  addi r7, r10, 0x59cc
	ctx.r[7].s64 = ctx.r[10].s64 + 22988;
	// 8323D928: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D92C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323D930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D934: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D938: 38895A1C  addi r4, r9, 0x5a1c
	ctx.r[4].s64 = ctx.r[9].s64 + 23068;
	// 8323D93C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D940: 3868B564  addi r3, r8, -0x4a9c
	ctx.r[3].s64 = ctx.r[8].s64 + -19100;
	// 8323D944: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D94C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D950: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D954: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D958: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D960: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323D964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D968: 4BC6A861  bl 0x82ea81c8
	ctx.lr = 0x8323D96C;
	sub_82EA81C8(ctx, base);
	// 8323D96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323D980 size=108
    let mut pc: u32 = 0x8323D980;
    'dispatch: loop {
        match pc {
            0x8323D980 => {
    //   block [0x8323D980..0x8323D9EC)
	// 8323D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323D988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323D98C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323D990: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323D994: 38EA5A60  addi r7, r10, 0x5a60
	ctx.r[7].s64 = ctx.r[10].s64 + 23136;
	// 8323D998: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 8323D99C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8323D9A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323D9A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323D9A8: 38895AD8  addi r4, r9, 0x5ad8
	ctx.r[4].s64 = ctx.r[9].s64 + 23256;
	// 8323D9AC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323D9B0: 3868B594  addi r3, r8, -0x4a6c
	ctx.r[3].s64 = ctx.r[8].s64 + -19052;
	// 8323D9B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323D9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323D9BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323D9C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323D9C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323D9C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323D9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323D9D0: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8323D9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323D9D8: 4BC6A7F1  bl 0x82ea81c8
	ctx.lr = 0x8323D9DC;
	sub_82EA81C8(ctx, base);
	// 8323D9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323D9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323D9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323D9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323D9F0 size=24
    let mut pc: u32 = 0x8323D9F0;
    'dispatch: loop {
        match pc {
            0x8323D9F0 => {
    //   block [0x8323D9F0..0x8323DA08)
	// 8323D9F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8323D9F4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323D9F8: 392A28F8  addi r9, r10, 0x28f8
	ctx.r[9].s64 = ctx.r[10].s64 + 10488;
	// 8323D9FC: 816B28E0  lwz r11, 0x28e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10464 as u32) ) } as u64;
	// 8323DA00: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8323DA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DA08 size=112
    let mut pc: u32 = 0x8323DA08;
    'dispatch: loop {
        match pc {
            0x8323DA08 => {
    //   block [0x8323DA08..0x8323DA78)
	// 8323DA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DA14: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8323DA18: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323DA1C: 38CA28F8  addi r6, r10, 0x28f8
	ctx.r[6].s64 = ctx.r[10].s64 + 10488;
	// 8323DA20: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323DA24: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8323DA28: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323DA2C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8323DA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323DA34: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8323DA38: 38885B80  addi r4, r8, 0x5b80
	ctx.r[4].s64 = ctx.r[8].s64 + 23424;
	// 8323DA3C: 3867B5C4  addi r3, r7, -0x4a3c
	ctx.r[3].s64 = ctx.r[7].s64 + -19004;
	// 8323DA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323DA44: 39295B6C  addi r9, r9, 0x5b6c
	ctx.r[9].s64 = ctx.r[9].s64 + 23404;
	// 8323DA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323DA4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323DA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323DA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323DA58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323DA5C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323DA60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323DA64: 4BC6A765  bl 0x82ea81c8
	ctx.lr = 0x8323DA68;
	sub_82EA81C8(ctx, base);
	// 8323DA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323DA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323DA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323DA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DA78 size=120
    let mut pc: u32 = 0x8323DA78;
    'dispatch: loop {
        match pc {
            0x8323DA78 => {
    //   block [0x8323DA78..0x8323DAF0)
	// 8323DA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DA80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323DA84: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DA88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8323DA8C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8323DA90: 392B5BF0  addi r9, r11, 0x5bf0
	ctx.r[9].s64 = ctx.r[11].s64 + 23536;
	// 8323DA94: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323DA98: 38C90014  addi r6, r9, 0x14
	ctx.r[6].s64 = ctx.r[9].s64 + 20;
	// 8323DA9C: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323DAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323DAA4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323DAA8: 38AA6DFC  addi r5, r10, 0x6dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 28156;
	// 8323DAAC: 38885C2C  addi r4, r8, 0x5c2c
	ctx.r[4].s64 = ctx.r[8].s64 + 23596;
	// 8323DAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323DAB4: 3867B5F4  addi r3, r7, -0x4a0c
	ctx.r[3].s64 = ctx.r[7].s64 + -18956;
	// 8323DAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323DABC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8323DAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323DAC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8323DAC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323DACC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323DAD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323DAD4: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 8323DAD8: 4BC6A6F1  bl 0x82ea81c8
	ctx.lr = 0x8323DADC;
	sub_82EA81C8(ctx, base);
	// 8323DADC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323DAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323DAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323DAE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323DAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DAF0 size=120
    let mut pc: u32 = 0x8323DAF0;
    'dispatch: loop {
        match pc {
            0x8323DAF0 => {
    //   block [0x8323DAF0..0x8323DB68)
	// 8323DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323DAFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DB00: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323DB04: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323DB08: 38CA5C40  addi r6, r10, 0x5c40
	ctx.r[6].s64 = ctx.r[10].s64 + 23616;
	// 8323DB0C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323DB10: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323DB14: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323DB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323DB1C: 38A9B1D4  addi r5, r9, -0x4e2c
	ctx.r[5].s64 = ctx.r[9].s64 + -20012;
	// 8323DB20: 38885CA8  addi r4, r8, 0x5ca8
	ctx.r[4].s64 = ctx.r[8].s64 + 23720;
	// 8323DB24: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323DB28: 3867B624  addi r3, r7, -0x49dc
	ctx.r[3].s64 = ctx.r[7].s64 + -18908;
	// 8323DB2C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323DB30: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8323DB34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323DB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323DB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323DB40: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323DB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323DB48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323DB4C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8323DB50: 4BC6A679  bl 0x82ea81c8
	ctx.lr = 0x8323DB54;
	sub_82EA81C8(ctx, base);
	// 8323DB54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323DB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323DB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323DB60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323DB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DB68 size=120
    let mut pc: u32 = 0x8323DB68;
    'dispatch: loop {
        match pc {
            0x8323DB68 => {
    //   block [0x8323DB68..0x8323DBE0)
	// 8323DB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DB70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323DB74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DB78: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8323DB7C: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 8323DB80: 38CA5CE8  addi r6, r10, 0x5ce8
	ctx.r[6].s64 = ctx.r[10].s64 + 23784;
	// 8323DB84: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 8323DB88: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 8323DB8C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323DB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323DB94: 38A9B204  addi r5, r9, -0x4dfc
	ctx.r[5].s64 = ctx.r[9].s64 + -19964;
	// 8323DB98: 38885D64  addi r4, r8, 0x5d64
	ctx.r[4].s64 = ctx.r[8].s64 + 23908;
	// 8323DB9C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323DBA0: 3867B654  addi r3, r7, -0x49ac
	ctx.r[3].s64 = ctx.r[7].s64 + -18860;
	// 8323DBA4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323DBA8: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8323DBAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323DBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323DBB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323DBB8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8323DBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323DBC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323DBC4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8323DBC8: 4BC6A601  bl 0x82ea81c8
	ctx.lr = 0x8323DBCC;
	sub_82EA81C8(ctx, base);
	// 8323DBCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8323DBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323DBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323DBD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323DBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DBE0 size=12
    let mut pc: u32 = 0x8323DBE0;
    'dispatch: loop {
        match pc {
            0x8323DBE0 => {
    //   block [0x8323DBE0..0x8323DBEC)
	// 8323DBE0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DBE4: 386B1F08  addi r3, r11, 0x1f08
	ctx.r[3].s64 = ctx.r[11].s64 + 7944;
	// 8323DBE8: 4BF6A8F0  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DBF0 size=12
    let mut pc: u32 = 0x8323DBF0;
    'dispatch: loop {
        match pc {
            0x8323DBF0 => {
    //   block [0x8323DBF0..0x8323DBFC)
	// 8323DBF0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DBF4: 386BB804  addi r3, r11, -0x47fc
	ctx.r[3].s64 = ctx.r[11].s64 + -18428;
	// 8323DBF8: 4BDBA050  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC00 size=12
    let mut pc: u32 = 0x8323DC00;
    'dispatch: loop {
        match pc {
            0x8323DC00 => {
    //   block [0x8323DC00..0x8323DC0C)
	// 8323DC00: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC04: 386BB7F8  addi r3, r11, -0x4808
	ctx.r[3].s64 = ctx.r[11].s64 + -18440;
	// 8323DC08: 4BDBA040  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC10 size=12
    let mut pc: u32 = 0x8323DC10;
    'dispatch: loop {
        match pc {
            0x8323DC10 => {
    //   block [0x8323DC10..0x8323DC1C)
	// 8323DC10: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC14: 386BB81C  addi r3, r11, -0x47e4
	ctx.r[3].s64 = ctx.r[11].s64 + -18404;
	// 8323DC18: 4BDBA030  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC20 size=12
    let mut pc: u32 = 0x8323DC20;
    'dispatch: loop {
        match pc {
            0x8323DC20 => {
    //   block [0x8323DC20..0x8323DC2C)
	// 8323DC20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC24: 386BB828  addi r3, r11, -0x47d8
	ctx.r[3].s64 = ctx.r[11].s64 + -18392;
	// 8323DC28: 4BDBA020  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC30 size=12
    let mut pc: u32 = 0x8323DC30;
    'dispatch: loop {
        match pc {
            0x8323DC30 => {
    //   block [0x8323DC30..0x8323DC3C)
	// 8323DC30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC34: 386BB840  addi r3, r11, -0x47c0
	ctx.r[3].s64 = ctx.r[11].s64 + -18368;
	// 8323DC38: 4BDBA010  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC40 size=12
    let mut pc: u32 = 0x8323DC40;
    'dispatch: loop {
        match pc {
            0x8323DC40 => {
    //   block [0x8323DC40..0x8323DC4C)
	// 8323DC40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC44: 386BB8A0  addi r3, r11, -0x4760
	ctx.r[3].s64 = ctx.r[11].s64 + -18272;
	// 8323DC48: 4BDBA000  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC50 size=12
    let mut pc: u32 = 0x8323DC50;
    'dispatch: loop {
        match pc {
            0x8323DC50 => {
    //   block [0x8323DC50..0x8323DC5C)
	// 8323DC50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DC54: 386BB894  addi r3, r11, -0x476c
	ctx.r[3].s64 = ctx.r[11].s64 + -18284;
	// 8323DC58: 4BDB9FF0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC60 size=12
    let mut pc: u32 = 0x8323DC60;
    'dispatch: loop {
        match pc {
            0x8323DC60 => {
    //   block [0x8323DC60..0x8323DC6C)
	// 8323DC60: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DC64: 386B20D8  addi r3, r11, 0x20d8
	ctx.r[3].s64 = ctx.r[11].s64 + 8408;
	// 8323DC68: 4BF6A870  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC70 size=12
    let mut pc: u32 = 0x8323DC70;
    'dispatch: loop {
        match pc {
            0x8323DC70 => {
    //   block [0x8323DC70..0x8323DC7C)
	// 8323DC70: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DC74: 386B1F20  addi r3, r11, 0x1f20
	ctx.r[3].s64 = ctx.r[11].s64 + 7968;
	// 8323DC78: 4BF6A860  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC80 size=12
    let mut pc: u32 = 0x8323DC80;
    'dispatch: loop {
        match pc {
            0x8323DC80 => {
    //   block [0x8323DC80..0x8323DC8C)
	// 8323DC80: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DC84: 386B1F48  addi r3, r11, 0x1f48
	ctx.r[3].s64 = ctx.r[11].s64 + 8008;
	// 8323DC88: 4BF6A850  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DC90 size=12
    let mut pc: u32 = 0x8323DC90;
    'dispatch: loop {
        match pc {
            0x8323DC90 => {
    //   block [0x8323DC90..0x8323DC9C)
	// 8323DC90: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DC94: 386B1F70  addi r3, r11, 0x1f70
	ctx.r[3].s64 = ctx.r[11].s64 + 8048;
	// 8323DC98: 4BF6A840  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCA0 size=12
    let mut pc: u32 = 0x8323DCA0;
    'dispatch: loop {
        match pc {
            0x8323DCA0 => {
    //   block [0x8323DCA0..0x8323DCAC)
	// 8323DCA0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCA4: 386B1F98  addi r3, r11, 0x1f98
	ctx.r[3].s64 = ctx.r[11].s64 + 8088;
	// 8323DCA8: 4BF6A830  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCB0 size=12
    let mut pc: u32 = 0x8323DCB0;
    'dispatch: loop {
        match pc {
            0x8323DCB0 => {
    //   block [0x8323DCB0..0x8323DCBC)
	// 8323DCB0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCB4: 386B1FC0  addi r3, r11, 0x1fc0
	ctx.r[3].s64 = ctx.r[11].s64 + 8128;
	// 8323DCB8: 4BF6A820  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCC0 size=12
    let mut pc: u32 = 0x8323DCC0;
    'dispatch: loop {
        match pc {
            0x8323DCC0 => {
    //   block [0x8323DCC0..0x8323DCCC)
	// 8323DCC0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCC4: 386B1FE8  addi r3, r11, 0x1fe8
	ctx.r[3].s64 = ctx.r[11].s64 + 8168;
	// 8323DCC8: 4BF6A810  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCD0 size=12
    let mut pc: u32 = 0x8323DCD0;
    'dispatch: loop {
        match pc {
            0x8323DCD0 => {
    //   block [0x8323DCD0..0x8323DCDC)
	// 8323DCD0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCD4: 386B2010  addi r3, r11, 0x2010
	ctx.r[3].s64 = ctx.r[11].s64 + 8208;
	// 8323DCD8: 4BF6A800  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


