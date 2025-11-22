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


pub fn sub_826B3340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3340 size=116
    let mut pc: u32 = 0x826B3340;
    'dispatch: loop {
        match pc {
            0x826B3340 => {
    //   block [0x826B3340..0x826B33B4)
	// 826B3340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B334C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B3350: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3354: 392BF474  addi r9, r11, -0xb8c
	ctx.r[9].s64 = ctx.r[11].s64 + -2956;
	// 826B3358: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B335C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3360: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B3364: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826B3368: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B336C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826B3370: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3374: 396B81B8  addi r11, r11, -0x7e48
	ctx.r[11].s64 = ctx.r[11].s64 + -32328;
	// 826B3378: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B337C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3380: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B3384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3388: 386ADC44  addi r3, r10, -0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9148;
	// 826B338C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B3390: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B3394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3398: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B339C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B33A0: 4BDB3A81  bl 0x82466e20
	ctx.lr = 0x826B33A4;
	sub_82466E20(ctx, base);
	// 826B33A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B33A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B33AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B33B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B33B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B33B8 size=100
    let mut pc: u32 = 0x826B33B8;
    'dispatch: loop {
        match pc {
            0x826B33B8 => {
    //   block [0x826B33B8..0x826B341C)
	// 826B33B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B33BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B33C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B33C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B33C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B33CC: 38AADD94  addi r5, r10, -0x226c
	ctx.r[5].s64 = ctx.r[10].s64 + -8812;
	// 826B33D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B33D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B33D8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826B33DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B33E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B33E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B33E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B33EC: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 826B33F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B33F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B33F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B33FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3408: 4BDB3A19  bl 0x82466e20
	ctx.lr = 0x826B340C;
	sub_82466E20(ctx, base);
	// 826B340C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3420 size=100
    let mut pc: u32 = 0x826B3420;
    'dispatch: loop {
        match pc {
            0x826B3420 => {
    //   block [0x826B3420..0x826B3484)
	// 826B3420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B342C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3434: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B343C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3440: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826B3444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B344C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3454: 386ADCA4  addi r3, r10, -0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + -9052;
	// 826B3458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B345C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3470: 4BDB39B1  bl 0x82466e20
	ctx.lr = 0x826B3474;
	sub_82466E20(ctx, base);
	// 826B3474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B347C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3488 size=108
    let mut pc: u32 = 0x826B3488;
    'dispatch: loop {
        match pc {
            0x826B3488 => {
    //   block [0x826B3488..0x826B34F4)
	// 826B3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3494: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B349C: 38EB8248  addi r7, r11, -0x7db8
	ctx.r[7].s64 = ctx.r[11].s64 + -32184;
	// 826B34A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B34A4: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826B34A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B34AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B34B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B34B8: 386ADCD4  addi r3, r10, -0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + -9004;
	// 826B34BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B34C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B34C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B34C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B34CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B34D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B34D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B34D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B34DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B34E0: 4BDB3941  bl 0x82466e20
	ctx.lr = 0x826B34E4;
	sub_82466E20(ctx, base);
	// 826B34E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B34E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B34EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B34F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B34F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B34F8 size=112
    let mut pc: u32 = 0x826B34F8;
    'dispatch: loop {
        match pc {
            0x826B34F8 => {
    //   block [0x826B34F8..0x826B3568)
	// 826B34F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B34FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B350C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3514: 390B8278  addi r8, r11, -0x7d88
	ctx.r[8].s64 = ctx.r[11].s64 + -32136;
	// 826B3518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B351C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826B3520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B352C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3530: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 826B3534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B353C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3554: 4BDB38CD  bl 0x82466e20
	ctx.lr = 0x826B3558;
	sub_82466E20(ctx, base);
	// 826B3558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B355C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3568 size=108
    let mut pc: u32 = 0x826B3568;
    'dispatch: loop {
        match pc {
            0x826B3568 => {
    //   block [0x826B3568..0x826B35D4)
	// 826B3568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3574: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B357C: 38EB8290  addi r7, r11, -0x7d70
	ctx.r[7].s64 = ctx.r[11].s64 + -32112;
	// 826B3580: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B3584: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826B3588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B358C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3598: 386ADD34  addi r3, r10, -0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8908;
	// 826B359C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B35A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B35A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B35A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B35AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B35B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B35B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B35B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B35BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B35C0: 4BDB3861  bl 0x82466e20
	ctx.lr = 0x826B35C4;
	sub_82466E20(ctx, base);
	// 826B35C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B35C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B35CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B35D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B35D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B35D8 size=28
    let mut pc: u32 = 0x826B35D8;
    'dispatch: loop {
        match pc {
            0x826B35D8 => {
    //   block [0x826B35D8..0x826B35F4)
	// 826B35D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B35DC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B35E0: 394ABE08  addi r10, r10, -0x41f8
	ctx.r[10].s64 = ctx.r[10].s64 + -16888;
	// 826B35E4: 816B81B4  lwz r11, -0x7e4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32332 as u32) ) } as u64;
	// 826B35E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B35EC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826B35F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B35F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B35F8 size=108
    let mut pc: u32 = 0x826B35F8;
    'dispatch: loop {
        match pc {
            0x826B35F8 => {
    //   block [0x826B35F8..0x826B3664)
	// 826B35F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B35FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3604: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B360C: 38EBBE08  addi r7, r11, -0x41f8
	ctx.r[7].s64 = ctx.r[11].s64 + -16888;
	// 826B3610: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826B3614: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826B3618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B361C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3628: 386ADD64  addi r3, r10, -0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + -8860;
	// 826B362C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B363C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B364C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3650: 4BDB37D1  bl 0x82466e20
	ctx.lr = 0x826B3654;
	sub_82466E20(ctx, base);
	// 826B3654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B365C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3668 size=116
    let mut pc: u32 = 0x826B3668;
    'dispatch: loop {
        match pc {
            0x826B3668 => {
    //   block [0x826B3668..0x826B36DC)
	// 826B3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3674: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3678: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B367C: 390B82B0  addi r8, r11, -0x7d50
	ctx.r[8].s64 = ctx.r[11].s64 + -32080;
	// 826B3680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3684: 392AF4E4  addi r9, r10, -0xb1c
	ctx.r[9].s64 = ctx.r[10].s64 + -2844;
	// 826B3688: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B368C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B3690: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3694: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B369C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B36A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B36A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B36A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B36AC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B36B0: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826B36B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B36B8: 386BDD94  addi r3, r11, -0x226c
	ctx.r[3].s64 = ctx.r[11].s64 + -8812;
	// 826B36BC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826B36C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B36C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B36C8: 4BDB3759  bl 0x82466e20
	ctx.lr = 0x826B36CC;
	sub_82466E20(ctx, base);
	// 826B36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B36D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B36D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B36D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B36E0 size=112
    let mut pc: u32 = 0x826B36E0;
    'dispatch: loop {
        match pc {
            0x826B36E0 => {
    //   block [0x826B36E0..0x826B3750)
	// 826B36E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B36E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B36E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B36EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B36F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B36F4: 38AADA64  addi r5, r10, -0x259c
	ctx.r[5].s64 = ctx.r[10].s64 + -9628;
	// 826B36F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B36FC: 390B8328  addi r8, r11, -0x7cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -31960;
	// 826B3700: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3704: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826B3708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B370C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3718: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 826B371C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B372C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B373C: 4BDB36E5  bl 0x82466e20
	ctx.lr = 0x826B3740;
	sub_82466E20(ctx, base);
	// 826B3740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B374C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3750 size=108
    let mut pc: u32 = 0x826B3750;
    'dispatch: loop {
        match pc {
            0x826B3750 => {
    //   block [0x826B3750..0x826B37BC)
	// 826B3750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B375C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3764: 38EB8340  addi r7, r11, -0x7cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -31936;
	// 826B3768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B376C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826B3770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B377C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3780: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 826B3784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B378C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B379C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B37A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B37A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B37A8: 4BDB3679  bl 0x82466e20
	ctx.lr = 0x826B37AC;
	sub_82466E20(ctx, base);
	// 826B37AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B37B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B37B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B37B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B37C0 size=112
    let mut pc: u32 = 0x826B37C0;
    'dispatch: loop {
        match pc {
            0x826B37C0 => {
    //   block [0x826B37C0..0x826B3830)
	// 826B37C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B37C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B37C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B37CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B37D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B37D4: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B37D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B37DC: 390B8370  addi r8, r11, -0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + -31888;
	// 826B37E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B37E4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826B37E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B37EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B37F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B37F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B37F8: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 826B37FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B380C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B381C: 4BDB3605  bl 0x82466e20
	ctx.lr = 0x826B3820;
	sub_82466E20(ctx, base);
	// 826B3820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B382C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3830 size=112
    let mut pc: u32 = 0x826B3830;
    'dispatch: loop {
        match pc {
            0x826B3830 => {
    //   block [0x826B3830..0x826B38A0)
	// 826B3830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B383C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3840: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3844: 38AADFA4  addi r5, r10, -0x205c
	ctx.r[5].s64 = ctx.r[10].s64 + -8284;
	// 826B3848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B384C: 390B83A0  addi r8, r11, -0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + -31840;
	// 826B3850: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3854: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826B3858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B385C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3868: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 826B386C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B387C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B388C: 4BDB3595  bl 0x82466e20
	ctx.lr = 0x826B3890;
	sub_82466E20(ctx, base);
	// 826B3890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B389C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B38A0 size=100
    let mut pc: u32 = 0x826B38A0;
    'dispatch: loop {
        match pc {
            0x826B38A0 => {
    //   block [0x826B38A0..0x826B3904)
	// 826B38A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B38A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B38A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B38AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B38B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B38B4: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B38B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B38BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B38C0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826B38C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B38C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B38CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B38D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B38D4: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 826B38D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B38DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B38E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B38E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B38E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B38EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B38F0: 4BDB3531  bl 0x82466e20
	ctx.lr = 0x826B38F4;
	sub_82466E20(ctx, base);
	// 826B38F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B38F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B38FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3908 size=112
    let mut pc: u32 = 0x826B3908;
    'dispatch: loop {
        match pc {
            0x826B3908 => {
    //   block [0x826B3908..0x826B3978)
	// 826B3908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B390C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3918: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B391C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 826B3920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3924: 390B83D0  addi r8, r11, -0x7c30
	ctx.r[8].s64 = ctx.r[11].s64 + -31792;
	// 826B3928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B392C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826B3930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B393C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3940: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 826B3944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B394C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B395C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3964: 4BDB34BD  bl 0x82466e20
	ctx.lr = 0x826B3968;
	sub_82466E20(ctx, base);
	// 826B3968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B396C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3978 size=112
    let mut pc: u32 = 0x826B3978;
    'dispatch: loop {
        match pc {
            0x826B3978 => {
    //   block [0x826B3978..0x826B39E8)
	// 826B3978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3988: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B398C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 826B3990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3994: 390B8418  addi r8, r11, -0x7be8
	ctx.r[8].s64 = ctx.r[11].s64 + -31720;
	// 826B3998: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B399C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826B39A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B39A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B39A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B39AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B39B0: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 826B39B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B39B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B39BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B39C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B39C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B39C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B39CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B39D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B39D4: 4BDB344D  bl 0x82466e20
	ctx.lr = 0x826B39D8;
	sub_82466E20(ctx, base);
	// 826B39D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B39DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B39E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B39E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B39E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B39E8 size=108
    let mut pc: u32 = 0x826B39E8;
    'dispatch: loop {
        match pc {
            0x826B39E8 => {
    //   block [0x826B39E8..0x826B3A54)
	// 826B39E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B39EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B39F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B39F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B39F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B39FC: 38EB84C0  addi r7, r11, -0x7b40
	ctx.r[7].s64 = ctx.r[11].s64 + -31552;
	// 826B3A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B3A04: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826B3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3A18: 386ADF14  addi r3, r10, -0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8428;
	// 826B3A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3A40: 4BDB33E1  bl 0x82466e20
	ctx.lr = 0x826B3A44;
	sub_82466E20(ctx, base);
	// 826B3A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3A58 size=112
    let mut pc: u32 = 0x826B3A58;
    'dispatch: loop {
        match pc {
            0x826B3A58 => {
    //   block [0x826B3A58..0x826B3AC8)
	// 826B3A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3A64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3A6C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3A74: 390B8508  addi r8, r11, -0x7af8
	ctx.r[8].s64 = ctx.r[11].s64 + -31480;
	// 826B3A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B3A7C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826B3A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3A90: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 826B3A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3AB4: 4BDB336D  bl 0x82466e20
	ctx.lr = 0x826B3AB8;
	sub_82466E20(ctx, base);
	// 826B3AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3AC8 size=100
    let mut pc: u32 = 0x826B3AC8;
    'dispatch: loop {
        match pc {
            0x826B3AC8 => {
    //   block [0x826B3AC8..0x826B3B2C)
	// 826B3AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3AD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3ADC: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3AE8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826B3AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3AFC: 386ADF74  addi r3, r10, -0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + -8332;
	// 826B3B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3B08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3B10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3B18: 4BDB3309  bl 0x82466e20
	ctx.lr = 0x826B3B1C;
	sub_82466E20(ctx, base);
	// 826B3B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3B30 size=100
    let mut pc: u32 = 0x826B3B30;
    'dispatch: loop {
        match pc {
            0x826B3B30 => {
    //   block [0x826B3B30..0x826B3B94)
	// 826B3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3B3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3B44: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3B50: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826B3B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3B64: 386ADFA4  addi r3, r10, -0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + -8284;
	// 826B3B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3B80: 4BDB32A1  bl 0x82466e20
	ctx.lr = 0x826B3B84;
	sub_82466E20(ctx, base);
	// 826B3B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3B98 size=112
    let mut pc: u32 = 0x826B3B98;
    'dispatch: loop {
        match pc {
            0x826B3B98 => {
    //   block [0x826B3B98..0x826B3C08)
	// 826B3B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3BA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3BA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3BAC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B3BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3BB4: 390B8568  addi r8, r11, -0x7a98
	ctx.r[8].s64 = ctx.r[11].s64 + -31384;
	// 826B3BB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B3BBC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826B3BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3BD0: 386ADFD4  addi r3, r10, -0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + -8236;
	// 826B3BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3BF4: 4BDB322D  bl 0x82466e20
	ctx.lr = 0x826B3BF8;
	sub_82466E20(ctx, base);
	// 826B3BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3C08 size=112
    let mut pc: u32 = 0x826B3C08;
    'dispatch: loop {
        match pc {
            0x826B3C08 => {
    //   block [0x826B3C08..0x826B3C78)
	// 826B3C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3C1C: 38AADD94  addi r5, r10, -0x226c
	ctx.r[5].s64 = ctx.r[10].s64 + -8812;
	// 826B3C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3C24: 390B85F8  addi r8, r11, -0x7a08
	ctx.r[8].s64 = ctx.r[11].s64 + -31240;
	// 826B3C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3C2C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826B3C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3C40: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 826B3C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3C64: 4BDB31BD  bl 0x82466e20
	ctx.lr = 0x826B3C68;
	sub_82466E20(ctx, base);
	// 826B3C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3C78 size=112
    let mut pc: u32 = 0x826B3C78;
    'dispatch: loop {
        match pc {
            0x826B3C78 => {
    //   block [0x826B3C78..0x826B3CE8)
	// 826B3C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3C8C: 38AADEE4  addi r5, r10, -0x211c
	ctx.r[5].s64 = ctx.r[10].s64 + -8476;
	// 826B3C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3C94: 390B8610  addi r8, r11, -0x79f0
	ctx.r[8].s64 = ctx.r[11].s64 + -31216;
	// 826B3C98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3C9C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826B3CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3CB0: 386AE034  addi r3, r10, -0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -8140;
	// 826B3CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3CD4: 4BDB314D  bl 0x82466e20
	ctx.lr = 0x826B3CD8;
	sub_82466E20(ctx, base);
	// 826B3CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3CE8 size=112
    let mut pc: u32 = 0x826B3CE8;
    'dispatch: loop {
        match pc {
            0x826B3CE8 => {
    //   block [0x826B3CE8..0x826B3D58)
	// 826B3CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3CFC: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3D04: 390B8640  addi r8, r11, -0x79c0
	ctx.r[8].s64 = ctx.r[11].s64 + -31168;
	// 826B3D08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B3D0C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826B3D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3D20: 386AE064  addi r3, r10, -0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -8092;
	// 826B3D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3D44: 4BDB30DD  bl 0x82466e20
	ctx.lr = 0x826B3D48;
	sub_82466E20(ctx, base);
	// 826B3D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3D58 size=112
    let mut pc: u32 = 0x826B3D58;
    'dispatch: loop {
        match pc {
            0x826B3D58 => {
    //   block [0x826B3D58..0x826B3DC8)
	// 826B3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3D64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3D6C: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3D74: 390B8688  addi r8, r11, -0x7978
	ctx.r[8].s64 = ctx.r[11].s64 + -31096;
	// 826B3D78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B3D7C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826B3D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3D90: 386AE094  addi r3, r10, -0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -8044;
	// 826B3D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3DB4: 4BDB306D  bl 0x82466e20
	ctx.lr = 0x826B3DB8;
	sub_82466E20(ctx, base);
	// 826B3DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3DC8 size=112
    let mut pc: u32 = 0x826B3DC8;
    'dispatch: loop {
        match pc {
            0x826B3DC8 => {
    //   block [0x826B3DC8..0x826B3E38)
	// 826B3DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3DDC: 38AADA64  addi r5, r10, -0x259c
	ctx.r[5].s64 = ctx.r[10].s64 + -9628;
	// 826B3DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3DE4: 390B86D0  addi r8, r11, -0x7930
	ctx.r[8].s64 = ctx.r[11].s64 + -31024;
	// 826B3DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3DEC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826B3DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3E00: 386AE0C4  addi r3, r10, -0x1f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7996;
	// 826B3E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3E24: 4BDB2FFD  bl 0x82466e20
	ctx.lr = 0x826B3E28;
	sub_82466E20(ctx, base);
	// 826B3E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3E38 size=112
    let mut pc: u32 = 0x826B3E38;
    'dispatch: loop {
        match pc {
            0x826B3E38 => {
    //   block [0x826B3E38..0x826B3EA8)
	// 826B3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3E48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3E4C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3E54: 390B86E8  addi r8, r11, -0x7918
	ctx.r[8].s64 = ctx.r[11].s64 + -31000;
	// 826B3E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3E5C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826B3E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3E70: 386AE0F4  addi r3, r10, -0x1f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7948;
	// 826B3E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3E94: 4BDB2F8D  bl 0x82466e20
	ctx.lr = 0x826B3E98;
	sub_82466E20(ctx, base);
	// 826B3E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B3EA8 size=24
    let mut pc: u32 = 0x826B3EA8;
    'dispatch: loop {
        match pc {
            0x826B3EA8 => {
    //   block [0x826B3EA8..0x826B3EC0)
	// 826B3EA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3EAC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B3EB0: 394ABF40  addi r10, r10, -0x40c0
	ctx.r[10].s64 = ctx.r[10].s64 + -16576;
	// 826B3EB4: 816B82AC  lwz r11, -0x7d54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32084 as u32) ) } as u64;
	// 826B3EB8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B3EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3EC0 size=112
    let mut pc: u32 = 0x826B3EC0;
    'dispatch: loop {
        match pc {
            0x826B3EC0 => {
    //   block [0x826B3EC0..0x826B3F30)
	// 826B3EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3ECC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B3ED0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3ED4: 392AF608  addi r9, r10, -0x9f8
	ctx.r[9].s64 = ctx.r[10].s64 + -2552;
	// 826B3ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3EDC: 390BBF40  addi r8, r11, -0x40c0
	ctx.r[8].s64 = ctx.r[11].s64 + -16576;
	// 826B3EE0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B3EE4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826B3EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3EEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3EF8: 386AE124  addi r3, r10, -0x1edc
	ctx.r[3].s64 = ctx.r[10].s64 + -7900;
	// 826B3EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B3F00: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826B3F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3F1C: 4BDB2F05  bl 0x82466e20
	ctx.lr = 0x826B3F20;
	sub_82466E20(ctx, base);
	// 826B3F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3F30 size=112
    let mut pc: u32 = 0x826B3F30;
    'dispatch: loop {
        match pc {
            0x826B3F30 => {
    //   block [0x826B3F30..0x826B3FA0)
	// 826B3F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3F40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3F44: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B3F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3F4C: 390B871C  addi r8, r11, -0x78e4
	ctx.r[8].s64 = ctx.r[11].s64 + -30948;
	// 826B3F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3F54: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826B3F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3F5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3F68: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 826B3F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3F8C: 4BDB2E95  bl 0x82466e20
	ctx.lr = 0x826B3F90;
	sub_82466E20(ctx, base);
	// 826B3F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3FA0 size=108
    let mut pc: u32 = 0x826B3FA0;
    'dispatch: loop {
        match pc {
            0x826B3FA0 => {
    //   block [0x826B3FA0..0x826B400C)
	// 826B3FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3FAC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3FB4: 38EB874C  addi r7, r11, -0x78b4
	ctx.r[7].s64 = ctx.r[11].s64 + -30900;
	// 826B3FB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B3FBC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826B3FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3FD0: 386AE184  addi r3, r10, -0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7804;
	// 826B3FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3FF8: 4BDB2E29  bl 0x82466e20
	ctx.lr = 0x826B3FFC;
	sub_82466E20(ctx, base);
	// 826B3FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4010 size=100
    let mut pc: u32 = 0x826B4010;
    'dispatch: loop {
        match pc {
            0x826B4010 => {
    //   block [0x826B4010..0x826B4074)
	// 826B4010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B401C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4024: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4030: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826B4034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B403C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4044: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 826B4048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B404C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B4054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B405C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4060: 4BDB2DC1  bl 0x82466e20
	ctx.lr = 0x826B4064;
	sub_82466E20(ctx, base);
	// 826B4064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4078 size=112
    let mut pc: u32 = 0x826B4078;
    'dispatch: loop {
        match pc {
            0x826B4078 => {
    //   block [0x826B4078..0x826B40E8)
	// 826B4078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B407C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4088: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B408C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4094: 390B8764  addi r8, r11, -0x789c
	ctx.r[8].s64 = ctx.r[11].s64 + -30876;
	// 826B4098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B409C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826B40A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B40A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B40A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B40AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B40B0: 386AE1E4  addi r3, r10, -0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7708;
	// 826B40B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B40B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B40BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B40C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B40C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B40C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B40CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B40D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B40D4: 4BDB2D4D  bl 0x82466e20
	ctx.lr = 0x826B40D8;
	sub_82466E20(ctx, base);
	// 826B40D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B40DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B40E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B40E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B40E8 size=112
    let mut pc: u32 = 0x826B40E8;
    'dispatch: loop {
        match pc {
            0x826B40E8 => {
    //   block [0x826B40E8..0x826B4158)
	// 826B40E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B40EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B40F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B40F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B40F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B40FC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4104: 390B877C  addi r8, r11, -0x7884
	ctx.r[8].s64 = ctx.r[11].s64 + -30852;
	// 826B4108: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B410C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826B4110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B411C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4120: 386AE214  addi r3, r10, -0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + -7660;
	// 826B4124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B412C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B413C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4144: 4BDB2CDD  bl 0x82466e20
	ctx.lr = 0x826B4148;
	sub_82466E20(ctx, base);
	// 826B4148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B414C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4158 size=112
    let mut pc: u32 = 0x826B4158;
    'dispatch: loop {
        match pc {
            0x826B4158 => {
    //   block [0x826B4158..0x826B41C8)
	// 826B4158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B416C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4174: 390B87AC  addi r8, r11, -0x7854
	ctx.r[8].s64 = ctx.r[11].s64 + -30804;
	// 826B4178: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B417C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826B4180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4190: 386AE244  addi r3, r10, -0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7612;
	// 826B4194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B419C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B41A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B41A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B41A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B41AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B41B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B41B4: 4BDB2C6D  bl 0x82466e20
	ctx.lr = 0x826B41B8;
	sub_82466E20(ctx, base);
	// 826B41B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B41BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B41C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B41C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B41C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B41C8 size=112
    let mut pc: u32 = 0x826B41C8;
    'dispatch: loop {
        match pc {
            0x826B41C8 => {
    //   block [0x826B41C8..0x826B4238)
	// 826B41C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B41CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B41D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B41D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B41D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B41DC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B41E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B41E4: 390B87DC  addi r8, r11, -0x7824
	ctx.r[8].s64 = ctx.r[11].s64 + -30756;
	// 826B41E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B41EC: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826B41F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B41F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B41F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B41FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4200: 386AE274  addi r3, r10, -0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7564;
	// 826B4204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B420C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B421C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4224: 4BDB2BFD  bl 0x82466e20
	ctx.lr = 0x826B4228;
	sub_82466E20(ctx, base);
	// 826B4228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B422C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4238 size=112
    let mut pc: u32 = 0x826B4238;
    'dispatch: loop {
        match pc {
            0x826B4238 => {
    //   block [0x826B4238..0x826B42A8)
	// 826B4238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4248: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B424C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4254: 390B880C  addi r8, r11, -0x77f4
	ctx.r[8].s64 = ctx.r[11].s64 + -30708;
	// 826B4258: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B425C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826B4260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B426C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4270: 386AE2A4  addi r3, r10, -0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7516;
	// 826B4274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B427C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4294: 4BDB2B8D  bl 0x82466e20
	ctx.lr = 0x826B4298;
	sub_82466E20(ctx, base);
	// 826B4298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B429C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B42A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B42A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B42A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B42A8 size=112
    let mut pc: u32 = 0x826B42A8;
    'dispatch: loop {
        match pc {
            0x826B42A8 => {
    //   block [0x826B42A8..0x826B4318)
	// 826B42A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B42AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B42B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B42B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B42B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B42BC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B42C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B42C4: 390B8824  addi r8, r11, -0x77dc
	ctx.r[8].s64 = ctx.r[11].s64 + -30684;
	// 826B42C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B42CC: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826B42D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B42D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B42D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B42DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B42E0: 386AE2D4  addi r3, r10, -0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7468;
	// 826B42E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B42E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B42EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B42F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B42F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B42F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B42FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4304: 4BDB2B1D  bl 0x82466e20
	ctx.lr = 0x826B4308;
	sub_82466E20(ctx, base);
	// 826B4308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B430C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4318 size=112
    let mut pc: u32 = 0x826B4318;
    'dispatch: loop {
        match pc {
            0x826B4318 => {
    //   block [0x826B4318..0x826B4388)
	// 826B4318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4328: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B432C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4334: 390B8840  addi r8, r11, -0x77c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30656;
	// 826B4338: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B433C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826B4340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B434C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4350: 386AE304  addi r3, r10, -0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7420;
	// 826B4354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B435C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B436C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4374: 4BDB2AAD  bl 0x82466e20
	ctx.lr = 0x826B4378;
	sub_82466E20(ctx, base);
	// 826B4378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B437C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4388 size=112
    let mut pc: u32 = 0x826B4388;
    'dispatch: loop {
        match pc {
            0x826B4388 => {
    //   block [0x826B4388..0x826B43F8)
	// 826B4388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B438C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B439C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B43A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B43A4: 390B8888  addi r8, r11, -0x7778
	ctx.r[8].s64 = ctx.r[11].s64 + -30584;
	// 826B43A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B43AC: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826B43B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B43B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B43B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B43BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B43C0: 386AE334  addi r3, r10, -0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -7372;
	// 826B43C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B43C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B43CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B43D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B43D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B43D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B43DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B43E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B43E4: 4BDB2A3D  bl 0x82466e20
	ctx.lr = 0x826B43E8;
	sub_82466E20(ctx, base);
	// 826B43E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B43EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B43F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B43F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B43F8 size=112
    let mut pc: u32 = 0x826B43F8;
    'dispatch: loop {
        match pc {
            0x826B43F8 => {
    //   block [0x826B43F8..0x826B4468)
	// 826B43F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B43FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4408: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B440C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4414: 390B88D0  addi r8, r11, -0x7730
	ctx.r[8].s64 = ctx.r[11].s64 + -30512;
	// 826B4418: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B441C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826B4420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B442C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4430: 386AE364  addi r3, r10, -0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7324;
	// 826B4434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B443C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4454: 4BDB29CD  bl 0x82466e20
	ctx.lr = 0x826B4458;
	sub_82466E20(ctx, base);
	// 826B4458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B445C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4468 size=112
    let mut pc: u32 = 0x826B4468;
    'dispatch: loop {
        match pc {
            0x826B4468 => {
    //   block [0x826B4468..0x826B44D8)
	// 826B4468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B446C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B447C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4484: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 826B4488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B448C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826B4490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B44A0: 386AE394  addi r3, r10, -0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -7276;
	// 826B44A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B44A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B44AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B44B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B44B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B44B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B44BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B44C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B44C4: 4BDB295D  bl 0x82466e20
	ctx.lr = 0x826B44C8;
	sub_82466E20(ctx, base);
	// 826B44C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B44CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B44D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B44D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B44D8 size=116
    let mut pc: u32 = 0x826B44D8;
    'dispatch: loop {
        match pc {
            0x826B44D8 => {
    //   block [0x826B44D8..0x826B454C)
	// 826B44D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B44DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B44E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B44E4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B44E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B44EC: 390A8918  addi r8, r10, -0x76e8
	ctx.r[8].s64 = ctx.r[10].s64 + -30440;
	// 826B44F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B44F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B44F8: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B44FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4500: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B450C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826B4510: 396BF630  addi r11, r11, -0x9d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2512;
	// 826B4514: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4518: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B451C: 386AE3C4  addi r3, r10, -0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7228;
	// 826B4520: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B4524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4528: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B452C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4538: 4BDB28E9  bl 0x82466e20
	ctx.lr = 0x826B453C;
	sub_82466E20(ctx, base);
	// 826B453C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4550 size=116
    let mut pc: u32 = 0x826B4550;
    'dispatch: loop {
        match pc {
            0x826B4550 => {
    //   block [0x826B4550..0x826B45C4)
	// 826B4550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B455C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4560: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B4564: 390A8990  addi r8, r10, -0x7670
	ctx.r[8].s64 = ctx.r[10].s64 + -30320;
	// 826B4568: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B456C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B4570: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4574: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4578: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B457C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4584: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826B4588: 396BF648  addi r11, r11, -0x9b8
	ctx.r[11].s64 = ctx.r[11].s64 + -2488;
	// 826B458C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4590: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4594: 386AE3F4  addi r3, r10, -0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7180;
	// 826B4598: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B459C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B45A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B45A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B45A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B45AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B45B0: 4BDB2871  bl 0x82466e20
	ctx.lr = 0x826B45B4;
	sub_82466E20(ctx, base);
	// 826B45B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B45B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B45BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B45C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B45C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B45C8 size=24
    let mut pc: u32 = 0x826B45C8;
    'dispatch: loop {
        match pc {
            0x826B45C8 => {
    //   block [0x826B45C8..0x826B45E0)
	// 826B45C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B45CC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B45D0: 394ABF58  addi r10, r10, -0x40a8
	ctx.r[10].s64 = ctx.r[10].s64 + -16552;
	// 826B45D4: 816B883C  lwz r11, -0x77c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30660 as u32) ) } as u64;
	// 826B45D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B45DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B45E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B45E0 size=116
    let mut pc: u32 = 0x826B45E0;
    'dispatch: loop {
        match pc {
            0x826B45E0 => {
    //   block [0x826B45E0..0x826B4654)
	// 826B45E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B45E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B45E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B45EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B45F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B45F4: 392BF674  addi r9, r11, -0x98c
	ctx.r[9].s64 = ctx.r[11].s64 + -2444;
	// 826B45F8: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B45FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4600: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B4604: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826B4608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B460C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826B4610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4614: 396BBF58  addi r11, r11, -0x40a8
	ctx.r[11].s64 = ctx.r[11].s64 + -16552;
	// 826B4618: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B461C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4620: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B4624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4628: 386AE424  addi r3, r10, -0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7132;
	// 826B462C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4630: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B4634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4638: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B463C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B4640: 4BDB27E1  bl 0x82466e20
	ctx.lr = 0x826B4644;
	sub_82466E20(ctx, base);
	// 826B4644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B464C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4658 size=112
    let mut pc: u32 = 0x826B4658;
    'dispatch: loop {
        match pc {
            0x826B4658 => {
    //   block [0x826B4658..0x826B46C8)
	// 826B4658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4664: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4668: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B466C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4674: 390B8A20  addi r8, r11, -0x75e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30176;
	// 826B4678: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B467C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826B4680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B468C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4690: 386AE454  addi r3, r10, -0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + -7084;
	// 826B4694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B469C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B46A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B46A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B46A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B46AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B46B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B46B4: 4BDB276D  bl 0x82466e20
	ctx.lr = 0x826B46B8;
	sub_82466E20(ctx, base);
	// 826B46B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B46BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B46C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B46C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B46C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B46C8 size=112
    let mut pc: u32 = 0x826B46C8;
    'dispatch: loop {
        match pc {
            0x826B46C8 => {
    //   block [0x826B46C8..0x826B4738)
	// 826B46C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B46CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B46D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B46D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B46D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B46DC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B46E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B46E4: 390B8A80  addi r8, r11, -0x7580
	ctx.r[8].s64 = ctx.r[11].s64 + -30080;
	// 826B46E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B46EC: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826B46F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B46F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B46F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B46FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4700: 386AE484  addi r3, r10, -0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7036;
	// 826B4704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B470C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B471C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4724: 4BDB26FD  bl 0x82466e20
	ctx.lr = 0x826B4728;
	sub_82466E20(ctx, base);
	// 826B4728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B472C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4738 size=112
    let mut pc: u32 = 0x826B4738;
    'dispatch: loop {
        match pc {
            0x826B4738 => {
    //   block [0x826B4738..0x826B47A8)
	// 826B4738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4748: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B474C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4754: 390B8B28  addi r8, r11, -0x74d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29912;
	// 826B4758: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B475C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826B4760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B476C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4770: 386AE4B4  addi r3, r10, -0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -6988;
	// 826B4774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B477C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B478C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4794: 4BDB268D  bl 0x82466e20
	ctx.lr = 0x826B4798;
	sub_82466E20(ctx, base);
	// 826B4798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B47A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B47A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B47A8 size=112
    let mut pc: u32 = 0x826B47A8;
    'dispatch: loop {
        match pc {
            0x826B47A8 => {
    //   block [0x826B47A8..0x826B4818)
	// 826B47A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B47AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B47B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B47B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B47B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B47BC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B47C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B47C4: 390B8BA0  addi r8, r11, -0x7460
	ctx.r[8].s64 = ctx.r[11].s64 + -29792;
	// 826B47C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B47CC: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826B47D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B47D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B47D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B47DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B47E0: 386AE4E4  addi r3, r10, -0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6940;
	// 826B47E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B47E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B47EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B47F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B47F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B47F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B47FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4804: 4BDB261D  bl 0x82466e20
	ctx.lr = 0x826B4808;
	sub_82466E20(ctx, base);
	// 826B4808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B480C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4818 size=112
    let mut pc: u32 = 0x826B4818;
    'dispatch: loop {
        match pc {
            0x826B4818 => {
    //   block [0x826B4818..0x826B4888)
	// 826B4818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B481C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4828: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B482C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4834: 390B8BE8  addi r8, r11, -0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + -29720;
	// 826B4838: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B483C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826B4840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B484C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4850: 386AE514  addi r3, r10, -0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + -6892;
	// 826B4854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B486C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4874: 4BDB25AD  bl 0x82466e20
	ctx.lr = 0x826B4878;
	sub_82466E20(ctx, base);
	// 826B4878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B487C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4888 size=112
    let mut pc: u32 = 0x826B4888;
    'dispatch: loop {
        match pc {
            0x826B4888 => {
    //   block [0x826B4888..0x826B48F8)
	// 826B4888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B488C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4894: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B489C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B48A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B48A4: 390B8C78  addi r8, r11, -0x7388
	ctx.r[8].s64 = ctx.r[11].s64 + -29576;
	// 826B48A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B48AC: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826B48B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B48B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B48B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B48BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B48C0: 386AE544  addi r3, r10, -0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + -6844;
	// 826B48C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B48C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B48CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B48D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B48D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B48D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B48DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B48E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B48E4: 4BDB253D  bl 0x82466e20
	ctx.lr = 0x826B48E8;
	sub_82466E20(ctx, base);
	// 826B48E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B48EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B48F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B48F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B48F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B48F8 size=112
    let mut pc: u32 = 0x826B48F8;
    'dispatch: loop {
        match pc {
            0x826B48F8 => {
    //   block [0x826B48F8..0x826B4968)
	// 826B48F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B48FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4908: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B490C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4914: 390B8CD8  addi r8, r11, -0x7328
	ctx.r[8].s64 = ctx.r[11].s64 + -29480;
	// 826B4918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B491C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826B4920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B492C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4930: 386AE574  addi r3, r10, -0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -6796;
	// 826B4934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B493C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B494C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4954: 4BDB24CD  bl 0x82466e20
	ctx.lr = 0x826B4958;
	sub_82466E20(ctx, base);
	// 826B4958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B495C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4968 size=112
    let mut pc: u32 = 0x826B4968;
    'dispatch: loop {
        match pc {
            0x826B4968 => {
    //   block [0x826B4968..0x826B49D8)
	// 826B4968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B496C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4974: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4978: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B497C: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4984: 390B8D20  addi r8, r11, -0x72e0
	ctx.r[8].s64 = ctx.r[11].s64 + -29408;
	// 826B4988: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B498C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826B4990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B499C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B49A0: 386AE5A4  addi r3, r10, -0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -6748;
	// 826B49A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B49A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B49AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B49B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B49B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B49B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B49BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B49C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B49C4: 4BDB245D  bl 0x82466e20
	ctx.lr = 0x826B49C8;
	sub_82466E20(ctx, base);
	// 826B49C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B49CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B49D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B49D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B49D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B49D8 size=112
    let mut pc: u32 = 0x826B49D8;
    'dispatch: loop {
        match pc {
            0x826B49D8 => {
    //   block [0x826B49D8..0x826B4A48)
	// 826B49D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B49DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B49E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B49E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B49E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B49EC: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B49F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B49F4: 390B8D50  addi r8, r11, -0x72b0
	ctx.r[8].s64 = ctx.r[11].s64 + -29360;
	// 826B49F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B49FC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826B4A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4A10: 386AE5D4  addi r3, r10, -0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6700;
	// 826B4A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4A34: 4BDB23ED  bl 0x82466e20
	ctx.lr = 0x826B4A38;
	sub_82466E20(ctx, base);
	// 826B4A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4A48 size=100
    let mut pc: u32 = 0x826B4A48;
    'dispatch: loop {
        match pc {
            0x826B4A48 => {
    //   block [0x826B4A48..0x826B4AAC)
	// 826B4A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4A5C: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4A68: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826B4A6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4A7C: 386AE604  addi r3, r10, -0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6652;
	// 826B4A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4A88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B4A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B4A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4A98: 4BDB2389  bl 0x82466e20
	ctx.lr = 0x826B4A9C;
	sub_82466E20(ctx, base);
	// 826B4A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4AB0 size=112
    let mut pc: u32 = 0x826B4AB0;
    'dispatch: loop {
        match pc {
            0x826B4AB0 => {
    //   block [0x826B4AB0..0x826B4B20)
	// 826B4AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4ABC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4AC0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4AC4: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4ACC: 390B8D80  addi r8, r11, -0x7280
	ctx.r[8].s64 = ctx.r[11].s64 + -29312;
	// 826B4AD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4AD4: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826B4AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4ADC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4AE8: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 826B4AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4B0C: 4BDB2315  bl 0x82466e20
	ctx.lr = 0x826B4B10;
	sub_82466E20(ctx, base);
	// 826B4B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4B20 size=112
    let mut pc: u32 = 0x826B4B20;
    'dispatch: loop {
        match pc {
            0x826B4B20 => {
    //   block [0x826B4B20..0x826B4B90)
	// 826B4B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4B2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4B30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4B34: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4B3C: 390B8D98  addi r8, r11, -0x7268
	ctx.r[8].s64 = ctx.r[11].s64 + -29288;
	// 826B4B40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B4B44: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826B4B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4B4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4B58: 386AE664  addi r3, r10, -0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + -6556;
	// 826B4B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4B7C: 4BDB22A5  bl 0x82466e20
	ctx.lr = 0x826B4B80;
	sub_82466E20(ctx, base);
	// 826B4B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4B90 size=112
    let mut pc: u32 = 0x826B4B90;
    'dispatch: loop {
        match pc {
            0x826B4B90 => {
    //   block [0x826B4B90..0x826B4C00)
	// 826B4B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4B9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4BA0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4BA4: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4BAC: 390B8DF8  addi r8, r11, -0x7208
	ctx.r[8].s64 = ctx.r[11].s64 + -29192;
	// 826B4BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4BB4: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826B4BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4BBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4BC8: 386AE694  addi r3, r10, -0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + -6508;
	// 826B4BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4BEC: 4BDB2235  bl 0x82466e20
	ctx.lr = 0x826B4BF0;
	sub_82466E20(ctx, base);
	// 826B4BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4C00 size=112
    let mut pc: u32 = 0x826B4C00;
    'dispatch: loop {
        match pc {
            0x826B4C00 => {
    //   block [0x826B4C00..0x826B4C70)
	// 826B4C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4C0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4C14: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4C1C: 390B8E10  addi r8, r11, -0x71f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29168;
	// 826B4C20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B4C24: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826B4C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4C2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4C38: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 826B4C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4C5C: 4BDB21C5  bl 0x82466e20
	ctx.lr = 0x826B4C60;
	sub_82466E20(ctx, base);
	// 826B4C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4C70 size=112
    let mut pc: u32 = 0x826B4C70;
    'dispatch: loop {
        match pc {
            0x826B4C70 => {
    //   block [0x826B4C70..0x826B4CE0)
	// 826B4C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4C7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4C84: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4C8C: 390B8E40  addi r8, r11, -0x71c0
	ctx.r[8].s64 = ctx.r[11].s64 + -29120;
	// 826B4C90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4C94: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826B4C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4C9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4CA8: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 826B4CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4CCC: 4BDB2155  bl 0x82466e20
	ctx.lr = 0x826B4CD0;
	sub_82466E20(ctx, base);
	// 826B4CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B4CE0 size=24
    let mut pc: u32 = 0x826B4CE0;
    'dispatch: loop {
        match pc {
            0x826B4CE0 => {
    //   block [0x826B4CE0..0x826B4CF8)
	// 826B4CE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4CE4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4CE8: 394AC000  addi r10, r10, -0x4000
	ctx.r[10].s64 = ctx.r[10].s64 + -16384;
	// 826B4CEC: 816B8E58  lwz r11, -0x71a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29096 as u32) ) } as u64;
	// 826B4CF0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B4CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4CF8 size=112
    let mut pc: u32 = 0x826B4CF8;
    'dispatch: loop {
        match pc {
            0x826B4CF8 => {
    //   block [0x826B4CF8..0x826B4D68)
	// 826B4CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4D04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4D08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4D0C: 392AF6D0  addi r9, r10, -0x930
	ctx.r[9].s64 = ctx.r[10].s64 + -2352;
	// 826B4D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4D14: 390BC000  addi r8, r11, -0x4000
	ctx.r[8].s64 = ctx.r[11].s64 + -16384;
	// 826B4D18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B4D1C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826B4D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4D24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4D30: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 826B4D34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4D38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4D54: 4BDB20CD  bl 0x82466e20
	ctx.lr = 0x826B4D58;
	sub_82466E20(ctx, base);
	// 826B4D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4D68 size=108
    let mut pc: u32 = 0x826B4D68;
    'dispatch: loop {
        match pc {
            0x826B4D68 => {
    //   block [0x826B4D68..0x826B4DD4)
	// 826B4D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4D74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4D7C: 38EB8E5C  addi r7, r11, -0x71a4
	ctx.r[7].s64 = ctx.r[11].s64 + -29092;
	// 826B4D80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B4D84: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826B4D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4D98: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 826B4D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4DC0: 4BDB2061  bl 0x82466e20
	ctx.lr = 0x826B4DC4;
	sub_82466E20(ctx, base);
	// 826B4DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4DD8 size=108
    let mut pc: u32 = 0x826B4DD8;
    'dispatch: loop {
        match pc {
            0x826B4DD8 => {
    //   block [0x826B4DD8..0x826B4E44)
	// 826B4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4DE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4DEC: 38EB8E78  addi r7, r11, -0x7188
	ctx.r[7].s64 = ctx.r[11].s64 + -29064;
	// 826B4DF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B4DF4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826B4DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4E08: 386AE784  addi r3, r10, -0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + -6268;
	// 826B4E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4E30: 4BDB1FF1  bl 0x82466e20
	ctx.lr = 0x826B4E34;
	sub_82466E20(ctx, base);
	// 826B4E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4E48 size=116
    let mut pc: u32 = 0x826B4E48;
    'dispatch: loop {
        match pc {
            0x826B4E48 => {
    //   block [0x826B4E48..0x826B4EBC)
	// 826B4E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4E54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4E58: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4E5C: 390B8EC0  addi r8, r11, -0x7140
	ctx.r[8].s64 = ctx.r[11].s64 + -28992;
	// 826B4E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4E64: 392AF788  addi r9, r10, -0x878
	ctx.r[9].s64 = ctx.r[10].s64 + -2168;
	// 826B4E68: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4E6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B4E70: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4E8C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B4E90: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826B4E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4E98: 386BE7B4  addi r3, r11, -0x184c
	ctx.r[3].s64 = ctx.r[11].s64 + -6220;
	// 826B4E9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4EA8: 4BDB1F79  bl 0x82466e20
	ctx.lr = 0x826B4EAC;
	sub_82466E20(ctx, base);
	// 826B4EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4EC0 size=108
    let mut pc: u32 = 0x826B4EC0;
    'dispatch: loop {
        match pc {
            0x826B4EC0 => {
    //   block [0x826B4EC0..0x826B4F2C)
	// 826B4EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4ECC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B4ED4: 38EB8ED8  addi r7, r11, -0x7128
	ctx.r[7].s64 = ctx.r[11].s64 + -28968;
	// 826B4ED8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B4EDC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826B4EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4EF0: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 826B4EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4F18: 4BDB1F09  bl 0x82466e20
	ctx.lr = 0x826B4F1C;
	sub_82466E20(ctx, base);
	// 826B4F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B4F30 size=24
    let mut pc: u32 = 0x826B4F30;
    'dispatch: loop {
        match pc {
            0x826B4F30 => {
    //   block [0x826B4F30..0x826B4F48)
	// 826B4F30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4F34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4F38: 394AC048  addi r10, r10, -0x3fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -16312;
	// 826B4F3C: 816B8F38  lwz r11, -0x70c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28872 as u32) ) } as u64;
	// 826B4F40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B4F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4F48 size=116
    let mut pc: u32 = 0x826B4F48;
    'dispatch: loop {
        match pc {
            0x826B4F48 => {
    //   block [0x826B4F48..0x826B4FBC)
	// 826B4F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4F54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4F58: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4F5C: 390BC048  addi r8, r11, -0x3fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -16312;
	// 826B4F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4F64: 392AF7E4  addi r9, r10, -0x81c
	ctx.r[9].s64 = ctx.r[10].s64 + -2076;
	// 826B4F68: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4F6C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826B4F70: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4F74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4F7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4F8C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B4F90: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826B4F94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4F98: 386BE814  addi r3, r11, -0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6124;
	// 826B4F9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826B4FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4FA8: 4BDB1E79  bl 0x82466e20
	ctx.lr = 0x826B4FAC;
	sub_82466E20(ctx, base);
	// 826B4FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4FC0 size=108
    let mut pc: u32 = 0x826B4FC0;
    'dispatch: loop {
        match pc {
            0x826B4FC0 => {
    //   block [0x826B4FC0..0x826B502C)
	// 826B4FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4FCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4FD4: 38EB8F44  addi r7, r11, -0x70bc
	ctx.r[7].s64 = ctx.r[11].s64 + -28860;
	// 826B4FD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B4FDC: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826B4FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4FF0: 386AE844  addi r3, r10, -0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6076;
	// 826B4FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B500C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5018: 4BDB1E09  bl 0x82466e20
	ctx.lr = 0x826B501C;
	sub_82466E20(ctx, base);
	// 826B501C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5030 size=112
    let mut pc: u32 = 0x826B5030;
    'dispatch: loop {
        match pc {
            0x826B5030 => {
    //   block [0x826B5030..0x826B50A0)
	// 826B5030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B503C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5040: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5044: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B504C: 390B8F74  addi r8, r11, -0x708c
	ctx.r[8].s64 = ctx.r[11].s64 + -28812;
	// 826B5050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5054: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826B5058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B505C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5068: 386AE874  addi r3, r10, -0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + -6028;
	// 826B506C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B507C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B508C: 4BDB1D95  bl 0x82466e20
	ctx.lr = 0x826B5090;
	sub_82466E20(ctx, base);
	// 826B5090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B50A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B50A0 size=112
    let mut pc: u32 = 0x826B50A0;
    'dispatch: loop {
        match pc {
            0x826B50A0 => {
    //   block [0x826B50A0..0x826B5110)
	// 826B50A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B50A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B50A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B50AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B50B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B50B4: 392AF828  addi r9, r10, -0x7d8
	ctx.r[9].s64 = ctx.r[10].s64 + -2008;
	// 826B50B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B50BC: 390B8F90  addi r8, r11, -0x7070
	ctx.r[8].s64 = ctx.r[11].s64 + -28784;
	// 826B50C0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B50C4: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826B50C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B50CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B50D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B50D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B50D8: 386AE8A4  addi r3, r10, -0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + -5980;
	// 826B50DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B50E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B50E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B50E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B50EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B50F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B50F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B50F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B50FC: 4BDB1D25  bl 0x82466e20
	ctx.lr = 0x826B5100;
	sub_82466E20(ctx, base);
	// 826B5100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5110 size=112
    let mut pc: u32 = 0x826B5110;
    'dispatch: loop {
        match pc {
            0x826B5110 => {
    //   block [0x826B5110..0x826B5180)
	// 826B5110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B511C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5120: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5124: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B512C: 390B8FD8  addi r8, r11, -0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + -28712;
	// 826B5130: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5134: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826B5138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B513C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5148: 386AE8D4  addi r3, r10, -0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + -5932;
	// 826B514C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B516C: 4BDB1CB5  bl 0x82466e20
	ctx.lr = 0x826B5170;
	sub_82466E20(ctx, base);
	// 826B5170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B517C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5180 size=112
    let mut pc: u32 = 0x826B5180;
    'dispatch: loop {
        match pc {
            0x826B5180 => {
    //   block [0x826B5180..0x826B51F0)
	// 826B5180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B518C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5190: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5194: 392AF854  addi r9, r10, -0x7ac
	ctx.r[9].s64 = ctx.r[10].s64 + -1964;
	// 826B5198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B519C: 390B8FF8  addi r8, r11, -0x7008
	ctx.r[8].s64 = ctx.r[11].s64 + -28680;
	// 826B51A0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B51A4: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826B51A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B51AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B51B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B51B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B51B8: 386AE904  addi r3, r10, -0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5884;
	// 826B51BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B51C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B51C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B51C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B51CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B51D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B51D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B51D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B51DC: 4BDB1C45  bl 0x82466e20
	ctx.lr = 0x826B51E0;
	sub_82466E20(ctx, base);
	// 826B51E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B51E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B51E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B51EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B51F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B51F0 size=112
    let mut pc: u32 = 0x826B51F0;
    'dispatch: loop {
        match pc {
            0x826B51F0 => {
    //   block [0x826B51F0..0x826B5260)
	// 826B51F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B51F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B51F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B51FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5204: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B520C: 390B9088  addi r8, r11, -0x6f78
	ctx.r[8].s64 = ctx.r[11].s64 + -28536;
	// 826B5210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5214: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826B5218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B521C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5228: 386AE934  addi r3, r10, -0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5836;
	// 826B522C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B523C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B524C: 4BDB1BD5  bl 0x82466e20
	ctx.lr = 0x826B5250;
	sub_82466E20(ctx, base);
	// 826B5250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B525C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5260 size=112
    let mut pc: u32 = 0x826B5260;
    'dispatch: loop {
        match pc {
            0x826B5260 => {
    //   block [0x826B5260..0x826B52D0)
	// 826B5260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B526C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5270: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5274: 38AAE994  addi r5, r10, -0x166c
	ctx.r[5].s64 = ctx.r[10].s64 + -5740;
	// 826B5278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B527C: 390B90A0  addi r8, r11, -0x6f60
	ctx.r[8].s64 = ctx.r[11].s64 + -28512;
	// 826B5280: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B5284: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826B5288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B528C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5298: 386AE964  addi r3, r10, -0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + -5788;
	// 826B529C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B52A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B52A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B52A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B52AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B52B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B52B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B52B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B52BC: 4BDB1B65  bl 0x82466e20
	ctx.lr = 0x826B52C0;
	sub_82466E20(ctx, base);
	// 826B52C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B52C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B52C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B52CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B52D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B52D0 size=100
    let mut pc: u32 = 0x826B52D0;
    'dispatch: loop {
        match pc {
            0x826B52D0 => {
    //   block [0x826B52D0..0x826B5334)
	// 826B52D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B52D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B52D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B52DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B52E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B52E4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B52E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B52EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B52F0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826B52F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B52F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B52FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5304: 386AE994  addi r3, r10, -0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + -5740;
	// 826B5308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B530C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B5314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B531C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5320: 4BDB1B01  bl 0x82466e20
	ctx.lr = 0x826B5324;
	sub_82466E20(ctx, base);
	// 826B5324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B532C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B5338 size=24
    let mut pc: u32 = 0x826B5338;
    'dispatch: loop {
        match pc {
            0x826B5338 => {
    //   block [0x826B5338..0x826B5350)
	// 826B5338: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B533C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5340: 394AC120  addi r10, r10, -0x3ee0
	ctx.r[10].s64 = ctx.r[10].s64 + -16096;
	// 826B5344: 816B8FF4  lwz r11, -0x700c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28684 as u32) ) } as u64;
	// 826B5348: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5350 size=116
    let mut pc: u32 = 0x826B5350;
    'dispatch: loop {
        match pc {
            0x826B5350 => {
    //   block [0x826B5350..0x826B53C4)
	// 826B5350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B535C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5360: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5364: 390BC120  addi r8, r11, -0x3ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -16096;
	// 826B5368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B536C: 392AF890  addi r9, r10, -0x770
	ctx.r[9].s64 = ctx.r[10].s64 + -1904;
	// 826B5370: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5374: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B5378: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B537C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B538C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5394: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B5398: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826B539C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B53A0: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 826B53A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B53A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B53AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B53B0: 4BDB1A71  bl 0x82466e20
	ctx.lr = 0x826B53B4;
	sub_82466E20(ctx, base);
	// 826B53B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B53B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B53BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B53C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B53C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B53C8 size=108
    let mut pc: u32 = 0x826B53C8;
    'dispatch: loop {
        match pc {
            0x826B53C8 => {
    //   block [0x826B53C8..0x826B5434)
	// 826B53C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B53CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B53D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B53D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B53D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B53DC: 38EB9118  addi r7, r11, -0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -28392;
	// 826B53E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B53E4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826B53E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B53EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B53F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B53F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B53F8: 386AE9F4  addi r3, r10, -0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + -5644;
	// 826B53FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B540C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B541C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5420: 4BDB1A01  bl 0x82466e20
	ctx.lr = 0x826B5424;
	sub_82466E20(ctx, base);
	// 826B5424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B542C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5438 size=112
    let mut pc: u32 = 0x826B5438;
    'dispatch: loop {
        match pc {
            0x826B5438 => {
    //   block [0x826B5438..0x826B54A8)
	// 826B5438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5448: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B544C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5454: 390B9148  addi r8, r11, -0x6eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -28344;
	// 826B5458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B545C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826B5460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B546C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5470: 386AEA24  addi r3, r10, -0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5596;
	// 826B5474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B547C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B548C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5494: 4BDB198D  bl 0x82466e20
	ctx.lr = 0x826B5498;
	sub_82466E20(ctx, base);
	// 826B5498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B549C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B54A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B54A8 size=112
    let mut pc: u32 = 0x826B54A8;
    'dispatch: loop {
        match pc {
            0x826B54A8 => {
    //   block [0x826B54A8..0x826B5518)
	// 826B54A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B54AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B54B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B54B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B54B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B54BC: 392AF8B4  addi r9, r10, -0x74c
	ctx.r[9].s64 = ctx.r[10].s64 + -1868;
	// 826B54C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B54C4: 390B9168  addi r8, r11, -0x6e98
	ctx.r[8].s64 = ctx.r[11].s64 + -28312;
	// 826B54C8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B54CC: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826B54D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B54D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B54D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B54DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B54E0: 386AEA54  addi r3, r10, -0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5548;
	// 826B54E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B54E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B54EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B54F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B54F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B54F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B54FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5504: 4BDB191D  bl 0x82466e20
	ctx.lr = 0x826B5508;
	sub_82466E20(ctx, base);
	// 826B5508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B550C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5518 size=112
    let mut pc: u32 = 0x826B5518;
    'dispatch: loop {
        match pc {
            0x826B5518 => {
    //   block [0x826B5518..0x826B5588)
	// 826B5518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B551C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5528: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B552C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5534: 390B9210  addi r8, r11, -0x6df0
	ctx.r[8].s64 = ctx.r[11].s64 + -28144;
	// 826B5538: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B553C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826B5540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B554C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5550: 386AEA84  addi r3, r10, -0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + -5500;
	// 826B5554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B555C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B556C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5574: 4BDB18AD  bl 0x82466e20
	ctx.lr = 0x826B5578;
	sub_82466E20(ctx, base);
	// 826B5578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B557C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5588 size=108
    let mut pc: u32 = 0x826B5588;
    'dispatch: loop {
        match pc {
            0x826B5588 => {
    //   block [0x826B5588..0x826B55F4)
	// 826B5588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5594: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B559C: 38EB9228  addi r7, r11, -0x6dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -28120;
	// 826B55A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B55A4: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826B55A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B55AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B55B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B55B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B55B8: 386AEAB4  addi r3, r10, -0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + -5452;
	// 826B55BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B55C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B55C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B55C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B55CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B55D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B55D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B55D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B55DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B55E0: 4BDB1841  bl 0x82466e20
	ctx.lr = 0x826B55E4;
	sub_82466E20(ctx, base);
	// 826B55E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B55E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B55EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B55F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B55F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B55F8 size=112
    let mut pc: u32 = 0x826B55F8;
    'dispatch: loop {
        match pc {
            0x826B55F8 => {
    //   block [0x826B55F8..0x826B5668)
	// 826B55F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B55FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B560C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5614: 390B9258  addi r8, r11, -0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + -28072;
	// 826B5618: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B561C: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826B5620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5624: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B562C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5630: 386AEAE4  addi r3, r10, -0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + -5404;
	// 826B5634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B563C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B564C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5654: 4BDB17CD  bl 0x82466e20
	ctx.lr = 0x826B5658;
	sub_82466E20(ctx, base);
	// 826B5658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5668 size=112
    let mut pc: u32 = 0x826B5668;
    'dispatch: loop {
        match pc {
            0x826B5668 => {
    //   block [0x826B5668..0x826B56D8)
	// 826B5668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5674: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5678: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B567C: 392AF8E8  addi r9, r10, -0x718
	ctx.r[9].s64 = ctx.r[10].s64 + -1816;
	// 826B5680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5684: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 826B5688: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B568C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826B5690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5694: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B569C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B56A0: 386AEB14  addi r3, r10, -0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5356;
	// 826B56A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B56A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B56AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B56B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B56B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B56B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B56BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B56C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B56C4: 4BDB175D  bl 0x82466e20
	ctx.lr = 0x826B56C8;
	sub_82466E20(ctx, base);
	// 826B56C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B56CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B56D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B56D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B56D8 size=112
    let mut pc: u32 = 0x826B56D8;
    'dispatch: loop {
        match pc {
            0x826B56D8 => {
    //   block [0x826B56D8..0x826B5748)
	// 826B56D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B56DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B56E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B56E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B56E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B56EC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B56F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B56F4: 390B9318  addi r8, r11, -0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -27880;
	// 826B56F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B56FC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826B5700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5710: 386AEB44  addi r3, r10, -0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5308;
	// 826B5714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B571C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5734: 4BDB16ED  bl 0x82466e20
	ctx.lr = 0x826B5738;
	sub_82466E20(ctx, base);
	// 826B5738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5748 size=112
    let mut pc: u32 = 0x826B5748;
    'dispatch: loop {
        match pc {
            0x826B5748 => {
    //   block [0x826B5748..0x826B57B8)
	// 826B5748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B575C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5764: 390B9360  addi r8, r11, -0x6ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -27808;
	// 826B5768: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B576C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826B5770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B577C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5780: 386AEB74  addi r3, r10, -0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + -5260;
	// 826B5784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B578C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B579C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B57A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B57A4: 4BDB167D  bl 0x82466e20
	ctx.lr = 0x826B57A8;
	sub_82466E20(ctx, base);
	// 826B57A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B57AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B57B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B57B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B57B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B57B8 size=100
    let mut pc: u32 = 0x826B57B8;
    'dispatch: loop {
        match pc {
            0x826B57B8 => {
    //   block [0x826B57B8..0x826B581C)
	// 826B57B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B57BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B57C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B57C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B57C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B57CC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B57D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B57D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B57D8: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826B57DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B57E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B57E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B57E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B57EC: 386AEBA4  addi r3, r10, -0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + -5212;
	// 826B57F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B57F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B57F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B57FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B5804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5808: 4BDB1619  bl 0x82466e20
	ctx.lr = 0x826B580C;
	sub_82466E20(ctx, base);
	// 826B580C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5820 size=112
    let mut pc: u32 = 0x826B5820;
    'dispatch: loop {
        match pc {
            0x826B5820 => {
    //   block [0x826B5820..0x826B5890)
	// 826B5820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B582C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5830: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5834: 38AAE814  addi r5, r10, -0x17ec
	ctx.r[5].s64 = ctx.r[10].s64 + -6124;
	// 826B5838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B583C: 390B9438  addi r8, r11, -0x6bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -27592;
	// 826B5840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5844: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826B5848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B584C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5858: 386AEBD4  addi r3, r10, -0x142c
	ctx.r[3].s64 = ctx.r[10].s64 + -5164;
	// 826B585C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B586C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B587C: 4BDB15A5  bl 0x82466e20
	ctx.lr = 0x826B5880;
	sub_82466E20(ctx, base);
	// 826B5880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B588C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5890 size=112
    let mut pc: u32 = 0x826B5890;
    'dispatch: loop {
        match pc {
            0x826B5890 => {
    //   block [0x826B5890..0x826B5900)
	// 826B5890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B589C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B58A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B58A4: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B58A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B58AC: 390B9468  addi r8, r11, -0x6b98
	ctx.r[8].s64 = ctx.r[11].s64 + -27544;
	// 826B58B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B58B4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826B58B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B58BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B58C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B58C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B58C8: 386AEC04  addi r3, r10, -0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5116;
	// 826B58CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B58D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B58D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B58D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B58DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B58E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B58E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B58E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B58EC: 4BDB1535  bl 0x82466e20
	ctx.lr = 0x826B58F0;
	sub_82466E20(ctx, base);
	// 826B58F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B58F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B58F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B58FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5900 size=108
    let mut pc: u32 = 0x826B5900;
    'dispatch: loop {
        match pc {
            0x826B5900 => {
    //   block [0x826B5900..0x826B596C)
	// 826B5900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B590C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5914: 38EB9480  addi r7, r11, -0x6b80
	ctx.r[7].s64 = ctx.r[11].s64 + -27520;
	// 826B5918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B591C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826B5920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B592C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5930: 386AEC34  addi r3, r10, -0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5068;
	// 826B5934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B593C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B594C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5958: 4BDB14C9  bl 0x82466e20
	ctx.lr = 0x826B595C;
	sub_82466E20(ctx, base);
	// 826B595C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5970 size=112
    let mut pc: u32 = 0x826B5970;
    'dispatch: loop {
        match pc {
            0x826B5970 => {
    //   block [0x826B5970..0x826B59E0)
	// 826B5970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B597C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5980: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5984: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B598C: 390B94B0  addi r8, r11, -0x6b50
	ctx.r[8].s64 = ctx.r[11].s64 + -27472;
	// 826B5990: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B5994: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826B5998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B599C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B59A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B59A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B59A8: 386AEC64  addi r3, r10, -0x139c
	ctx.r[3].s64 = ctx.r[10].s64 + -5020;
	// 826B59AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B59B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B59B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B59B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B59BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B59C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B59C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B59C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B59CC: 4BDB1455  bl 0x82466e20
	ctx.lr = 0x826B59D0;
	sub_82466E20(ctx, base);
	// 826B59D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B59D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B59D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B59DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B59E0 size=112
    let mut pc: u32 = 0x826B59E0;
    'dispatch: loop {
        match pc {
            0x826B59E0 => {
    //   block [0x826B59E0..0x826B5A50)
	// 826B59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B59EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B59F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B59F4: 392AF914  addi r9, r10, -0x6ec
	ctx.r[9].s64 = ctx.r[10].s64 + -1772;
	// 826B59F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B59FC: 390B9548  addi r8, r11, -0x6ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -27320;
	// 826B5A00: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B5A04: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826B5A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5A18: 386AEC94  addi r3, r10, -0x136c
	ctx.r[3].s64 = ctx.r[10].s64 + -4972;
	// 826B5A1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5A20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B5A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5A3C: 4BDB13E5  bl 0x82466e20
	ctx.lr = 0x826B5A40;
	sub_82466E20(ctx, base);
	// 826B5A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5A50 size=112
    let mut pc: u32 = 0x826B5A50;
    'dispatch: loop {
        match pc {
            0x826B5A50 => {
    //   block [0x826B5A50..0x826B5AC0)
	// 826B5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5A5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5A64: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5A6C: 390B9590  addi r8, r11, -0x6a70
	ctx.r[8].s64 = ctx.r[11].s64 + -27248;
	// 826B5A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5A74: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826B5A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5A88: 386AECC4  addi r3, r10, -0x133c
	ctx.r[3].s64 = ctx.r[10].s64 + -4924;
	// 826B5A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5AAC: 4BDB1375  bl 0x82466e20
	ctx.lr = 0x826B5AB0;
	sub_82466E20(ctx, base);
	// 826B5AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5AC0 size=108
    let mut pc: u32 = 0x826B5AC0;
    'dispatch: loop {
        match pc {
            0x826B5AC0 => {
    //   block [0x826B5AC0..0x826B5B2C)
	// 826B5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5ACC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5AD4: 38EB95A8  addi r7, r11, -0x6a58
	ctx.r[7].s64 = ctx.r[11].s64 + -27224;
	// 826B5AD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B5ADC: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826B5AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5AF0: 386AECF4  addi r3, r10, -0x130c
	ctx.r[3].s64 = ctx.r[10].s64 + -4876;
	// 826B5AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5B18: 4BDB1309  bl 0x82466e20
	ctx.lr = 0x826B5B1C;
	sub_82466E20(ctx, base);
	// 826B5B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5B30 size=116
    let mut pc: u32 = 0x826B5B30;
    'dispatch: loop {
        match pc {
            0x826B5B30 => {
    //   block [0x826B5B30..0x826B5BA4)
	// 826B5B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5B3C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5B40: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826B5B44: 390A9638  addi r8, r10, -0x69c8
	ctx.r[8].s64 = ctx.r[10].s64 + -27080;
	// 826B5B48: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5B4C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B5B50: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5B54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5B58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5B64: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826B5B68: 396BF928  addi r11, r11, -0x6d8
	ctx.r[11].s64 = ctx.r[11].s64 + -1752;
	// 826B5B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5B70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5B74: 386AED24  addi r3, r10, -0x12dc
	ctx.r[3].s64 = ctx.r[10].s64 + -4828;
	// 826B5B78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B5B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5B80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B5B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5B90: 4BDB1291  bl 0x82466e20
	ctx.lr = 0x826B5B94;
	sub_82466E20(ctx, base);
	// 826B5B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5BA8 size=108
    let mut pc: u32 = 0x826B5BA8;
    'dispatch: loop {
        match pc {
            0x826B5BA8 => {
    //   block [0x826B5BA8..0x826B5C14)
	// 826B5BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5BB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5BBC: 38EB9710  addi r7, r11, -0x68f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26864;
	// 826B5BC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B5BC4: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826B5BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5BD8: 386AED54  addi r3, r10, -0x12ac
	ctx.r[3].s64 = ctx.r[10].s64 + -4780;
	// 826B5BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5C00: 4BDB1221  bl 0x82466e20
	ctx.lr = 0x826B5C04;
	sub_82466E20(ctx, base);
	// 826B5C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5C18 size=112
    let mut pc: u32 = 0x826B5C18;
    'dispatch: loop {
        match pc {
            0x826B5C18 => {
    //   block [0x826B5C18..0x826B5C88)
	// 826B5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5C24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5C2C: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5C34: 390B9758  addi r8, r11, -0x68a8
	ctx.r[8].s64 = ctx.r[11].s64 + -26792;
	// 826B5C38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B5C3C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826B5C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5C44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5C50: 386AED84  addi r3, r10, -0x127c
	ctx.r[3].s64 = ctx.r[10].s64 + -4732;
	// 826B5C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5C74: 4BDB11AD  bl 0x82466e20
	ctx.lr = 0x826B5C78;
	sub_82466E20(ctx, base);
	// 826B5C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5C88 size=112
    let mut pc: u32 = 0x826B5C88;
    'dispatch: loop {
        match pc {
            0x826B5C88 => {
    //   block [0x826B5C88..0x826B5CF8)
	// 826B5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5C94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5C9C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5CA4: 390B97D0  addi r8, r11, -0x6830
	ctx.r[8].s64 = ctx.r[11].s64 + -26672;
	// 826B5CA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5CAC: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826B5CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5CB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5CC0: 386AEDB4  addi r3, r10, -0x124c
	ctx.r[3].s64 = ctx.r[10].s64 + -4684;
	// 826B5CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5CE4: 4BDB113D  bl 0x82466e20
	ctx.lr = 0x826B5CE8;
	sub_82466E20(ctx, base);
	// 826B5CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5CF8 size=108
    let mut pc: u32 = 0x826B5CF8;
    'dispatch: loop {
        match pc {
            0x826B5CF8 => {
    //   block [0x826B5CF8..0x826B5D64)
	// 826B5CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5D04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5D0C: 38EB9800  addi r7, r11, -0x6800
	ctx.r[7].s64 = ctx.r[11].s64 + -26624;
	// 826B5D10: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B5D14: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826B5D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5D28: 386AEDE4  addi r3, r10, -0x121c
	ctx.r[3].s64 = ctx.r[10].s64 + -4636;
	// 826B5D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5D50: 4BDB10D1  bl 0x82466e20
	ctx.lr = 0x826B5D54;
	sub_82466E20(ctx, base);
	// 826B5D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5D68 size=112
    let mut pc: u32 = 0x826B5D68;
    'dispatch: loop {
        match pc {
            0x826B5D68 => {
    //   block [0x826B5D68..0x826B5DD8)
	// 826B5D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5D74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5D7C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5D84: 390B9878  addi r8, r11, -0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + -26504;
	// 826B5D88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B5D8C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826B5D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5DA0: 386AEE14  addi r3, r10, -0x11ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4588;
	// 826B5DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5DC4: 4BDB105D  bl 0x82466e20
	ctx.lr = 0x826B5DC8;
	sub_82466E20(ctx, base);
	// 826B5DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B5DD8 size=24
    let mut pc: u32 = 0x826B5DD8;
    'dispatch: loop {
        match pc {
            0x826B5DD8 => {
    //   block [0x826B5DD8..0x826B5DF0)
	// 826B5DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5DDC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5DE0: 394AC198  addi r10, r10, -0x3e68
	ctx.r[10].s64 = ctx.r[10].s64 + -15976;
	// 826B5DE4: 816B9544  lwz r11, -0x6abc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27324 as u32) ) } as u64;
	// 826B5DE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B5DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5DF0 size=116
    let mut pc: u32 = 0x826B5DF0;
    'dispatch: loop {
        match pc {
            0x826B5DF0 => {
    //   block [0x826B5DF0..0x826B5E64)
	// 826B5DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5DFC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5E00: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5E04: 390BC198  addi r8, r11, -0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + -15976;
	// 826B5E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5E0C: 392AF984  addi r9, r10, -0x67c
	ctx.r[9].s64 = ctx.r[10].s64 + -1660;
	// 826B5E10: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E14: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B5E18: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B5E1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5E24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5E34: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B5E38: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826B5E3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5E40: 386BEE44  addi r3, r11, -0x11bc
	ctx.r[3].s64 = ctx.r[11].s64 + -4540;
	// 826B5E44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B5E48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5E50: 4BDB0FD1  bl 0x82466e20
	ctx.lr = 0x826B5E54;
	sub_82466E20(ctx, base);
	// 826B5E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5E68 size=112
    let mut pc: u32 = 0x826B5E68;
    'dispatch: loop {
        match pc {
            0x826B5E68 => {
    //   block [0x826B5E68..0x826B5ED8)
	// 826B5E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5E74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5E7C: 38AAEE44  addi r5, r10, -0x11bc
	ctx.r[5].s64 = ctx.r[10].s64 + -4540;
	// 826B5E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5E84: 390B98C0  addi r8, r11, -0x6740
	ctx.r[8].s64 = ctx.r[11].s64 + -26432;
	// 826B5E88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5E8C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826B5E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5EA0: 386AEE74  addi r3, r10, -0x118c
	ctx.r[3].s64 = ctx.r[10].s64 + -4492;
	// 826B5EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5EC4: 4BDB0F5D  bl 0x82466e20
	ctx.lr = 0x826B5EC8;
	sub_82466E20(ctx, base);
	// 826B5EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5ED8 size=112
    let mut pc: u32 = 0x826B5ED8;
    'dispatch: loop {
        match pc {
            0x826B5ED8 => {
    //   block [0x826B5ED8..0x826B5F48)
	// 826B5ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5EE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5EEC: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5EF4: 390B98F0  addi r8, r11, -0x6710
	ctx.r[8].s64 = ctx.r[11].s64 + -26384;
	// 826B5EF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B5EFC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826B5F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5F10: 386AEEA4  addi r3, r10, -0x115c
	ctx.r[3].s64 = ctx.r[10].s64 + -4444;
	// 826B5F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5F34: 4BDB0EED  bl 0x82466e20
	ctx.lr = 0x826B5F38;
	sub_82466E20(ctx, base);
	// 826B5F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5F48 size=112
    let mut pc: u32 = 0x826B5F48;
    'dispatch: loop {
        match pc {
            0x826B5F48 => {
    //   block [0x826B5F48..0x826B5FB8)
	// 826B5F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5F5C: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5F64: 390B9950  addi r8, r11, -0x66b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26288;
	// 826B5F68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5F6C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826B5F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5F74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5F80: 386AEED4  addi r3, r10, -0x112c
	ctx.r[3].s64 = ctx.r[10].s64 + -4396;
	// 826B5F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5FA4: 4BDB0E7D  bl 0x82466e20
	ctx.lr = 0x826B5FA8;
	sub_82466E20(ctx, base);
	// 826B5FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5FB8 size=112
    let mut pc: u32 = 0x826B5FB8;
    'dispatch: loop {
        match pc {
            0x826B5FB8 => {
    //   block [0x826B5FB8..0x826B6028)
	// 826B5FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5FC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5FCC: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5FD4: 390B9980  addi r8, r11, -0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + -26240;
	// 826B5FD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B5FDC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826B5FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5FF0: 386AEF04  addi r3, r10, -0x10fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4348;
	// 826B5FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B600C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6014: 4BDB0E0D  bl 0x82466e20
	ctx.lr = 0x826B6018;
	sub_82466E20(ctx, base);
	// 826B6018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B601C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6028 size=108
    let mut pc: u32 = 0x826B6028;
    'dispatch: loop {
        match pc {
            0x826B6028 => {
    //   block [0x826B6028..0x826B6094)
	// 826B6028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6034: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B603C: 38EB99C8  addi r7, r11, -0x6638
	ctx.r[7].s64 = ctx.r[11].s64 + -26168;
	// 826B6040: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B6044: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826B6048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B604C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6058: 386AEF34  addi r3, r10, -0x10cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4300;
	// 826B605C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B606C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B607C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6080: 4BDB0DA1  bl 0x82466e20
	ctx.lr = 0x826B6084;
	sub_82466E20(ctx, base);
	// 826B6084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B608C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6098 size=112
    let mut pc: u32 = 0x826B6098;
    'dispatch: loop {
        match pc {
            0x826B6098 => {
    //   block [0x826B6098..0x826B6108)
	// 826B6098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B60A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B60A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B60A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B60AC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B60B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B60B4: 390B99F8  addi r8, r11, -0x6608
	ctx.r[8].s64 = ctx.r[11].s64 + -26120;
	// 826B60B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B60BC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826B60C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B60C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B60C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B60CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B60D0: 386AEF64  addi r3, r10, -0x109c
	ctx.r[3].s64 = ctx.r[10].s64 + -4252;
	// 826B60D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B60D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B60DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B60E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B60E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B60E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B60EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B60F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B60F4: 4BDB0D2D  bl 0x82466e20
	ctx.lr = 0x826B60F8;
	sub_82466E20(ctx, base);
	// 826B60F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B60FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6108 size=116
    let mut pc: u32 = 0x826B6108;
    'dispatch: loop {
        match pc {
            0x826B6108 => {
    //   block [0x826B6108..0x826B617C)
	// 826B6108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B610C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6114: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6118: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826B611C: 390A9A10  addi r8, r10, -0x65f0
	ctx.r[8].s64 = ctx.r[10].s64 + -26096;
	// 826B6120: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6128: 38AAF3E4  addi r5, r10, -0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3100;
	// 826B612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6130: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B613C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826B6140: 396BF998  addi r11, r11, -0x668
	ctx.r[11].s64 = ctx.r[11].s64 + -1640;
	// 826B6144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B614C: 386AEF94  addi r3, r10, -0x106c
	ctx.r[3].s64 = ctx.r[10].s64 + -4204;
	// 826B6150: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B6154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6158: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B615C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6168: 4BDB0CB9  bl 0x82466e20
	ctx.lr = 0x826B616C;
	sub_82466E20(ctx, base);
	// 826B616C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6180 size=100
    let mut pc: u32 = 0x826B6180;
    'dispatch: loop {
        match pc {
            0x826B6180 => {
    //   block [0x826B6180..0x826B61E4)
	// 826B6180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B618C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6194: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B619C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B61A0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826B61A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B61A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B61AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B61B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B61B4: 386AEFC4  addi r3, r10, -0x103c
	ctx.r[3].s64 = ctx.r[10].s64 + -4156;
	// 826B61B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B61BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B61C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B61C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B61C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B61CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B61D0: 4BDB0C51  bl 0x82466e20
	ctx.lr = 0x826B61D4;
	sub_82466E20(ctx, base);
	// 826B61D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B61D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B61DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B61E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B61E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B61E8 size=100
    let mut pc: u32 = 0x826B61E8;
    'dispatch: loop {
        match pc {
            0x826B61E8 => {
    //   block [0x826B61E8..0x826B624C)
	// 826B61E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B61EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B61F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B61F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B61F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B61FC: 38AAF054  addi r5, r10, -0xfac
	ctx.r[5].s64 = ctx.r[10].s64 + -4012;
	// 826B6200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6208: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826B620C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B621C: 386AEFF4  addi r3, r10, -0x100c
	ctx.r[3].s64 = ctx.r[10].s64 + -4108;
	// 826B6220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6238: 4BDB0BE9  bl 0x82466e20
	ctx.lr = 0x826B623C;
	sub_82466E20(ctx, base);
	// 826B623C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6250 size=100
    let mut pc: u32 = 0x826B6250;
    'dispatch: loop {
        match pc {
            0x826B6250 => {
    //   block [0x826B6250..0x826B62B4)
	// 826B6250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B625C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6264: 38AAEF94  addi r5, r10, -0x106c
	ctx.r[5].s64 = ctx.r[10].s64 + -4204;
	// 826B6268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6270: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826B6274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B627C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6284: 386AF024  addi r3, r10, -0xfdc
	ctx.r[3].s64 = ctx.r[10].s64 + -4060;
	// 826B6288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B628C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6290: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B6294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B629C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B62A0: 4BDB0B81  bl 0x82466e20
	ctx.lr = 0x826B62A4;
	sub_82466E20(ctx, base);
	// 826B62A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B62A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B62AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B62B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B62B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B62B8 size=104
    let mut pc: u32 = 0x826B62B8;
    'dispatch: loop {
        match pc {
            0x826B62B8 => {
    //   block [0x826B62B8..0x826B6320)
	// 826B62B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B62BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B62C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B62C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B62C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B62CC: 392AF9FC  addi r9, r10, -0x604
	ctx.r[9].s64 = ctx.r[10].s64 + -1540;
	// 826B62D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B62D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B62D8: 38AAEFC4  addi r5, r10, -0x103c
	ctx.r[5].s64 = ctx.r[10].s64 + -4156;
	// 826B62DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B62E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B62E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B62E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B62EC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826B62F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B62F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B62F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6304: 386AF054  addi r3, r10, -0xfac
	ctx.r[3].s64 = ctx.r[10].s64 + -4012;
	// 826B6308: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B630C: 4BDB0B15  bl 0x82466e20
	ctx.lr = 0x826B6310;
	sub_82466E20(ctx, base);
	// 826B6310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6320 size=108
    let mut pc: u32 = 0x826B6320;
    'dispatch: loop {
        match pc {
            0x826B6320 => {
    //   block [0x826B6320..0x826B638C)
	// 826B6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B632C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6334: 38EB9B94  addi r7, r11, -0x646c
	ctx.r[7].s64 = ctx.r[11].s64 + -25708;
	// 826B6338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B633C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826B6340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6350: 386AF084  addi r3, r10, -0xf7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3964;
	// 826B6354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B635C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B636C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6378: 4BDB0AA9  bl 0x82466e20
	ctx.lr = 0x826B637C;
	sub_82466E20(ctx, base);
	// 826B637C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6390 size=112
    let mut pc: u32 = 0x826B6390;
    'dispatch: loop {
        match pc {
            0x826B6390 => {
    //   block [0x826B6390..0x826B6400)
	// 826B6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B639C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B63A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B63A4: 38AAF054  addi r5, r10, -0xfac
	ctx.r[5].s64 = ctx.r[10].s64 + -4012;
	// 826B63A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B63AC: 390B9BC8  addi r8, r11, -0x6438
	ctx.r[8].s64 = ctx.r[11].s64 + -25656;
	// 826B63B0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B63B4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826B63B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B63BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B63C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B63C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B63C8: 386AF0B4  addi r3, r10, -0xf4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3916;
	// 826B63CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B63D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B63D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B63D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B63DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B63E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B63E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B63E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B63EC: 4BDB0A35  bl 0x82466e20
	ctx.lr = 0x826B63F0;
	sub_82466E20(ctx, base);
	// 826B63F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B63F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B63F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B63FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B6400 size=24
    let mut pc: u32 = 0x826B6400;
    'dispatch: loop {
        match pc {
            0x826B6400 => {
    //   block [0x826B6400..0x826B6418)
	// 826B6400: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6404: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6408: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 826B640C: 816B9BC4  lwz r11, -0x643c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25660 as u32) ) } as u64;
	// 826B6410: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B6414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6418 size=116
    let mut pc: u32 = 0x826B6418;
    'dispatch: loop {
        match pc {
            0x826B6418 => {
    //   block [0x826B6418..0x826B648C)
	// 826B6418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6424: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6428: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B642C: 390BC1B0  addi r8, r11, -0x3e50
	ctx.r[8].s64 = ctx.r[11].s64 + -15952;
	// 826B6430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6434: 392AFA60  addi r9, r10, -0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + -1440;
	// 826B6438: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B643C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B6440: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B644C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B645C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6460: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826B6464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6468: 386BF0E4  addi r3, r11, -0xf1c
	ctx.r[3].s64 = ctx.r[11].s64 + -3868;
	// 826B646C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6478: 4BDB09A9  bl 0x82466e20
	ctx.lr = 0x826B647C;
	sub_82466E20(ctx, base);
	// 826B647C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6490 size=100
    let mut pc: u32 = 0x826B6490;
    'dispatch: loop {
        match pc {
            0x826B6490 => {
    //   block [0x826B6490..0x826B64F4)
	// 826B6490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B649C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B64A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B64A4: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B64A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B64AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B64B0: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826B64B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B64B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B64BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B64C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B64C4: 386AF114  addi r3, r10, -0xeec
	ctx.r[3].s64 = ctx.r[10].s64 + -3820;
	// 826B64C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B64CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B64D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B64D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B64D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B64DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B64E0: 4BDB0941  bl 0x82466e20
	ctx.lr = 0x826B64E4;
	sub_82466E20(ctx, base);
	// 826B64E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B64E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B64EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B64F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B64F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B64F8 size=100
    let mut pc: u32 = 0x826B64F8;
    'dispatch: loop {
        match pc {
            0x826B64F8 => {
    //   block [0x826B64F8..0x826B655C)
	// 826B64F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B64FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B650C: 38AAF174  addi r5, r10, -0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + -3724;
	// 826B6510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6518: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826B651C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B652C: 386AF144  addi r3, r10, -0xebc
	ctx.r[3].s64 = ctx.r[10].s64 + -3772;
	// 826B6530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B653C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6548: 4BDB08D9  bl 0x82466e20
	ctx.lr = 0x826B654C;
	sub_82466E20(ctx, base);
	// 826B654C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6560 size=112
    let mut pc: u32 = 0x826B6560;
    'dispatch: loop {
        match pc {
            0x826B6560 => {
    //   block [0x826B6560..0x826B65D0)
	// 826B6560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B656C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6570: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6574: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B6578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B657C: 390B9C70  addi r8, r11, -0x6390
	ctx.r[8].s64 = ctx.r[11].s64 + -25488;
	// 826B6580: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B6584: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826B6588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B658C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6598: 386AF174  addi r3, r10, -0xe8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3724;
	// 826B659C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B65A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B65A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B65A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B65AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B65B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B65B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B65B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B65BC: 4BDB0865  bl 0x82466e20
	ctx.lr = 0x826B65C0;
	sub_82466E20(ctx, base);
	// 826B65C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B65C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B65C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B65CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B65D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B65D0 size=100
    let mut pc: u32 = 0x826B65D0;
    'dispatch: loop {
        match pc {
            0x826B65D0 => {
    //   block [0x826B65D0..0x826B6634)
	// 826B65D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B65D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B65D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B65DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B65E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B65E4: 38AAF174  addi r5, r10, -0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + -3724;
	// 826B65E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B65EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B65F0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826B65F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B65F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B65FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6604: 386AF1A4  addi r3, r10, -0xe5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3676;
	// 826B6608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B660C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B6614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6618: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B661C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6620: 4BDB0801  bl 0x82466e20
	ctx.lr = 0x826B6624;
	sub_82466E20(ctx, base);
	// 826B6624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B662C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6638 size=100
    let mut pc: u32 = 0x826B6638;
    'dispatch: loop {
        match pc {
            0x826B6638 => {
    //   block [0x826B6638..0x826B669C)
	// 826B6638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B663C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6644: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B664C: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B6650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6658: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826B665C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B666C: 386AF1D4  addi r3, r10, -0xe2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3628;
	// 826B6670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B667C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6688: 4BDB0799  bl 0x82466e20
	ctx.lr = 0x826B668C;
	sub_82466E20(ctx, base);
	// 826B668C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B66A0 size=100
    let mut pc: u32 = 0x826B66A0;
    'dispatch: loop {
        match pc {
            0x826B66A0 => {
    //   block [0x826B66A0..0x826B6704)
	// 826B66A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B66A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B66A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B66AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B66B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B66B4: 38AAF114  addi r5, r10, -0xeec
	ctx.r[5].s64 = ctx.r[10].s64 + -3820;
	// 826B66B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B66BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B66C0: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826B66C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B66C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B66CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B66D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B66D4: 386AF204  addi r3, r10, -0xdfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3580;
	// 826B66D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B66DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B66E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B66E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B66E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B66EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B66F0: 4BDB0731  bl 0x82466e20
	ctx.lr = 0x826B66F4;
	sub_82466E20(ctx, base);
	// 826B66F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B66F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B66FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6708 size=100
    let mut pc: u32 = 0x826B6708;
    'dispatch: loop {
        match pc {
            0x826B6708 => {
    //   block [0x826B6708..0x826B676C)
	// 826B6708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B671C: 38AAF1D4  addi r5, r10, -0xe2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3628;
	// 826B6720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6728: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826B672C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B673C: 386AF234  addi r3, r10, -0xdcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3532;
	// 826B6740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B674C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6758: 4BDB06C9  bl 0x82466e20
	ctx.lr = 0x826B675C;
	sub_82466E20(ctx, base);
	// 826B675C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6770 size=100
    let mut pc: u32 = 0x826B6770;
    'dispatch: loop {
        match pc {
            0x826B6770 => {
    //   block [0x826B6770..0x826B67D4)
	// 826B6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B677C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6784: 38AAF114  addi r5, r10, -0xeec
	ctx.r[5].s64 = ctx.r[10].s64 + -3820;
	// 826B6788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B678C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6790: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826B6794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B679C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B67A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B67A4: 386AF264  addi r3, r10, -0xd9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3484;
	// 826B67A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B67AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B67B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B67B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B67B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B67BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B67C0: 4BDB0661  bl 0x82466e20
	ctx.lr = 0x826B67C4;
	sub_82466E20(ctx, base);
	// 826B67C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B67C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B67CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B67D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B67D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B67D8 size=112
    let mut pc: u32 = 0x826B67D8;
    'dispatch: loop {
        match pc {
            0x826B67D8 => {
    //   block [0x826B67D8..0x826B6848)
	// 826B67D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B67DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B67E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B67E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B67E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B67EC: 38AAF2F4  addi r5, r10, -0xd0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3340;
	// 826B67F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B67F4: 390B9CA0  addi r8, r11, -0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + -25440;
	// 826B67F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B67FC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826B6800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B680C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6810: 386AF294  addi r3, r10, -0xd6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3436;
	// 826B6814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B681C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B682C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6834: 4BDB05ED  bl 0x82466e20
	ctx.lr = 0x826B6838;
	sub_82466E20(ctx, base);
	// 826B6838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B683C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6848 size=112
    let mut pc: u32 = 0x826B6848;
    'dispatch: loop {
        match pc {
            0x826B6848 => {
    //   block [0x826B6848..0x826B68B8)
	// 826B6848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6858: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B685C: 38AAF324  addi r5, r10, -0xcdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3292;
	// 826B6860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6864: 390B9CD0  addi r8, r11, -0x6330
	ctx.r[8].s64 = ctx.r[11].s64 + -25392;
	// 826B6868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B686C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826B6870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B687C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6880: 386AF2C4  addi r3, r10, -0xd3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3388;
	// 826B6884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B689C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B68A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B68A4: 4BDB057D  bl 0x82466e20
	ctx.lr = 0x826B68A8;
	sub_82466E20(ctx, base);
	// 826B68A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B68AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B68B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B68B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B68B8 size=112
    let mut pc: u32 = 0x826B68B8;
    'dispatch: loop {
        match pc {
            0x826B68B8 => {
    //   block [0x826B68B8..0x826B6928)
	// 826B68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B68C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B68C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B68CC: 38AAF3E4  addi r5, r10, -0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3100;
	// 826B68D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B68D4: 390B9CE8  addi r8, r11, -0x6318
	ctx.r[8].s64 = ctx.r[11].s64 + -25368;
	// 826B68D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B68DC: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826B68E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B68E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B68E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B68EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B68F0: 386AF2F4  addi r3, r10, -0xd0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3340;
	// 826B68F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B68F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B68FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6914: 4BDB050D  bl 0x82466e20
	ctx.lr = 0x826B6918;
	sub_82466E20(ctx, base);
	// 826B6918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6928 size=112
    let mut pc: u32 = 0x826B6928;
    'dispatch: loop {
        match pc {
            0x826B6928 => {
    //   block [0x826B6928..0x826B6998)
	// 826B6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6938: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B693C: 38AAF2F4  addi r5, r10, -0xd0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3340;
	// 826B6940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6944: 390B9D18  addi r8, r11, -0x62e8
	ctx.r[8].s64 = ctx.r[11].s64 + -25320;
	// 826B6948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B694C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826B6950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6960: 386AF324  addi r3, r10, -0xcdc
	ctx.r[3].s64 = ctx.r[10].s64 + -3292;
	// 826B6964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B696C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6984: 4BDB049D  bl 0x82466e20
	ctx.lr = 0x826B6988;
	sub_82466E20(ctx, base);
	// 826B6988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B698C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6998 size=112
    let mut pc: u32 = 0x826B6998;
    'dispatch: loop {
        match pc {
            0x826B6998 => {
    //   block [0x826B6998..0x826B6A08)
	// 826B6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B69A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B69A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B69A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B69AC: 38AAF324  addi r5, r10, -0xcdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3292;
	// 826B69B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B69B4: 390B9D30  addi r8, r11, -0x62d0
	ctx.r[8].s64 = ctx.r[11].s64 + -25296;
	// 826B69B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B69BC: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826B69C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B69C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B69C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B69CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B69D0: 386AF354  addi r3, r10, -0xcac
	ctx.r[3].s64 = ctx.r[10].s64 + -3244;
	// 826B69D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B69D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B69DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B69E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B69E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B69E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B69EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B69F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B69F4: 4BDB042D  bl 0x82466e20
	ctx.lr = 0x826B69F8;
	sub_82466E20(ctx, base);
	// 826B69F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B69FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6A08 size=116
    let mut pc: u32 = 0x826B6A08;
    'dispatch: loop {
        match pc {
            0x826B6A08 => {
    //   block [0x826B6A08..0x826B6A7C)
	// 826B6A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6A14: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6A18: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B6A1C: 390A9D48  addi r8, r10, -0x62b8
	ctx.r[8].s64 = ctx.r[10].s64 + -25272;
	// 826B6A20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6A24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6A28: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6A2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6A30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6A3C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826B6A40: 396BFA74  addi r11, r11, -0x58c
	ctx.r[11].s64 = ctx.r[11].s64 + -1420;
	// 826B6A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6A48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6A4C: 386AF384  addi r3, r10, -0xc7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3196;
	// 826B6A50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B6A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6A58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B6A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6A68: 4BDB03B9  bl 0x82466e20
	ctx.lr = 0x826B6A6C;
	sub_82466E20(ctx, base);
	// 826B6A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B6A80 size=48
    let mut pc: u32 = 0x826B6A80;
    'dispatch: loop {
        match pc {
            0x826B6A80 => {
    //   block [0x826B6A80..0x826B6AB0)
	// 826B6A80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6A84: 814B9DFC  lwz r10, -0x6204(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25092 as u32) ) } as u64;
	// 826B6A88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6A8C: 396BC228  addi r11, r11, -0x3dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -15832;
	// 826B6A90: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826B6A94: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6A98: 814A9DF8  lwz r10, -0x6208(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25096 as u32) ) } as u64;
	// 826B6A9C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826B6AA0: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6AA4: 814A9DF4  lwz r10, -0x620c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25100 as u32) ) } as u64;
	// 826B6AA8: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826B6AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6AB0 size=116
    let mut pc: u32 = 0x826B6AB0;
    'dispatch: loop {
        match pc {
            0x826B6AB0 => {
    //   block [0x826B6AB0..0x826B6B24)
	// 826B6AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6ABC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6AC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6AC4: 392BFB48  addi r9, r11, -0x4b8
	ctx.r[9].s64 = ctx.r[11].s64 + -1208;
	// 826B6AC8: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6ACC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6AD0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826B6AD4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826B6AD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6ADC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826B6AE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6AE4: 396BC228  addi r11, r11, -0x3dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -15832;
	// 826B6AE8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B6AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6AF0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B6AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6AF8: 386AF3B4  addi r3, r10, -0xc4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3148;
	// 826B6AFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826B6B00: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B6B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6B08: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B6B0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6B10: 4BDB0311  bl 0x82466e20
	ctx.lr = 0x826B6B14;
	sub_82466E20(ctx, base);
	// 826B6B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6B28 size=116
    let mut pc: u32 = 0x826B6B28;
    'dispatch: loop {
        match pc {
            0x826B6B28 => {
    //   block [0x826B6B28..0x826B6B9C)
	// 826B6B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6B34: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6B38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6B3C: 390B9E08  addi r8, r11, -0x61f8
	ctx.r[8].s64 = ctx.r[11].s64 + -25080;
	// 826B6B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6B44: 392AFC70  addi r9, r10, -0x390
	ctx.r[9].s64 = ctx.r[10].s64 + -912;
	// 826B6B48: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6B4C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B6B50: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6B54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6B5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6B6C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6B70: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826B6B74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6B78: 386BF3E4  addi r3, r11, -0xc1c
	ctx.r[3].s64 = ctx.r[11].s64 + -3100;
	// 826B6B7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6B80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6B88: 4BDB0299  bl 0x82466e20
	ctx.lr = 0x826B6B8C;
	sub_82466E20(ctx, base);
	// 826B6B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6BA0 size=112
    let mut pc: u32 = 0x826B6BA0;
    'dispatch: loop {
        match pc {
            0x826B6BA0 => {
    //   block [0x826B6BA0..0x826B6C10)
	// 826B6BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6BB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6BB4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6BBC: 390B9E98  addi r8, r11, -0x6168
	ctx.r[8].s64 = ctx.r[11].s64 + -24936;
	// 826B6BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6BC4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826B6BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6BD8: 386AF414  addi r3, r10, -0xbec
	ctx.r[3].s64 = ctx.r[10].s64 + -3052;
	// 826B6BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6BFC: 4BDB0225  bl 0x82466e20
	ctx.lr = 0x826B6C00;
	sub_82466E20(ctx, base);
	// 826B6C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6C10 size=112
    let mut pc: u32 = 0x826B6C10;
    'dispatch: loop {
        match pc {
            0x826B6C10 => {
    //   block [0x826B6C10..0x826B6C80)
	// 826B6C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6C1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6C20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6C24: 38AAD764  addi r5, r10, -0x289c
	ctx.r[5].s64 = ctx.r[10].s64 + -10396;
	// 826B6C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6C2C: 390B9EB0  addi r8, r11, -0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + -24912;
	// 826B6C30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6C34: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826B6C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6C40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6C48: 386AF444  addi r3, r10, -0xbbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3004;
	// 826B6C4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6C6C: 4BDB01B5  bl 0x82466e20
	ctx.lr = 0x826B6C70;
	sub_82466E20(ctx, base);
	// 826B6C70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6C80 size=108
    let mut pc: u32 = 0x826B6C80;
    'dispatch: loop {
        match pc {
            0x826B6C80 => {
    //   block [0x826B6C80..0x826B6CEC)
	// 826B6C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6C8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6C94: 38EB9EC8  addi r7, r11, -0x6138
	ctx.r[7].s64 = ctx.r[11].s64 + -24888;
	// 826B6C98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B6C9C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826B6CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6CB0: 386AF474  addi r3, r10, -0xb8c
	ctx.r[3].s64 = ctx.r[10].s64 + -2956;
	// 826B6CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6CD8: 4BDB0149  bl 0x82466e20
	ctx.lr = 0x826B6CDC;
	sub_82466E20(ctx, base);
	// 826B6CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6CF0 size=112
    let mut pc: u32 = 0x826B6CF0;
    'dispatch: loop {
        match pc {
            0x826B6CF0 => {
    //   block [0x826B6CF0..0x826B6D60)
	// 826B6CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6CFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D00: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6D04: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6D0C: 390B9EE0  addi r8, r11, -0x6120
	ctx.r[8].s64 = ctx.r[11].s64 + -24864;
	// 826B6D10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B6D14: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826B6D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6D28: 386AF4A4  addi r3, r10, -0xb5c
	ctx.r[3].s64 = ctx.r[10].s64 + -2908;
	// 826B6D2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6D4C: 4BDB00D5  bl 0x82466e20
	ctx.lr = 0x826B6D50;
	sub_82466E20(ctx, base);
	// 826B6D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6D60 size=108
    let mut pc: u32 = 0x826B6D60;
    'dispatch: loop {
        match pc {
            0x826B6D60 => {
    //   block [0x826B6D60..0x826B6DCC)
	// 826B6D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6D6C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6D74: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 826B6D78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B6D7C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826B6D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6D90: 386AF4D4  addi r3, r10, -0xb2c
	ctx.r[3].s64 = ctx.r[10].s64 + -2860;
	// 826B6D94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6DB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6DB8: 4BDB0069  bl 0x82466e20
	ctx.lr = 0x826B6DBC;
	sub_82466E20(ctx, base);
	// 826B6DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6DD0 size=112
    let mut pc: u32 = 0x826B6DD0;
    'dispatch: loop {
        match pc {
            0x826B6DD0 => {
    //   block [0x826B6DD0..0x826B6E40)
	// 826B6DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6DDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6DE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6DE4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6DEC: 390B9F40  addi r8, r11, -0x60c0
	ctx.r[8].s64 = ctx.r[11].s64 + -24768;
	// 826B6DF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B6DF4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826B6DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6E08: 386AF504  addi r3, r10, -0xafc
	ctx.r[3].s64 = ctx.r[10].s64 + -2812;
	// 826B6E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6E2C: 4BDAFFF5  bl 0x82466e20
	ctx.lr = 0x826B6E30;
	sub_82466E20(ctx, base);
	// 826B6E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6E40 size=112
    let mut pc: u32 = 0x826B6E40;
    'dispatch: loop {
        match pc {
            0x826B6E40 => {
    //   block [0x826B6E40..0x826B6EB0)
	// 826B6E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6E4C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6E50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6E54: 392AFCC8  addi r9, r10, -0x338
	ctx.r[9].s64 = ctx.r[10].s64 + -824;
	// 826B6E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6E5C: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826B6E60: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B6E64: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826B6E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6E78: 386AF534  addi r3, r10, -0xacc
	ctx.r[3].s64 = ctx.r[10].s64 + -2764;
	// 826B6E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6E80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6E9C: 4BDAFF85  bl 0x82466e20
	ctx.lr = 0x826B6EA0;
	sub_82466E20(ctx, base);
	// 826B6EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6EB0 size=116
    let mut pc: u32 = 0x826B6EB0;
    'dispatch: loop {
        match pc {
            0x826B6EB0 => {
    //   block [0x826B6EB0..0x826B6F24)
	// 826B6EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6EBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6EC0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6EC4: 390BA020  addi r8, r11, -0x5fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -24544;
	// 826B6EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6ECC: 392AFC9C  addi r9, r10, -0x364
	ctx.r[9].s64 = ctx.r[10].s64 + -868;
	// 826B6ED0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6ED4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B6ED8: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B6EDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6EE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6EF4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6EF8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826B6EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6F00: 386BF564  addi r3, r11, -0xa9c
	ctx.r[3].s64 = ctx.r[11].s64 + -2716;
	// 826B6F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6F08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6F10: 4BDAFF11  bl 0x82466e20
	ctx.lr = 0x826B6F14;
	sub_82466E20(ctx, base);
	// 826B6F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6F28 size=112
    let mut pc: u32 = 0x826B6F28;
    'dispatch: loop {
        match pc {
            0x826B6F28 => {
    //   block [0x826B6F28..0x826B6F98)
	// 826B6F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6F34: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6F38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6F3C: 392AFCF4  addi r9, r10, -0x30c
	ctx.r[9].s64 = ctx.r[10].s64 + -780;
	// 826B6F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6F44: 390BA038  addi r8, r11, -0x5fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24520;
	// 826B6F48: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826B6F4C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826B6F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6F60: 386AF594  addi r3, r10, -0xa6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2668;
	// 826B6F64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6F68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6F84: 4BDAFE9D  bl 0x82466e20
	ctx.lr = 0x826B6F88;
	sub_82466E20(ctx, base);
	// 826B6F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6F98 size=112
    let mut pc: u32 = 0x826B6F98;
    'dispatch: loop {
        match pc {
            0x826B6F98 => {
    //   block [0x826B6F98..0x826B7008)
	// 826B6F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6FA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6FA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6FAC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B6FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6FB4: 390BA098  addi r8, r11, -0x5f68
	ctx.r[8].s64 = ctx.r[11].s64 + -24424;
	// 826B6FB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6FBC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826B6FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6FD0: 386AF5C4  addi r3, r10, -0xa3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2620;
	// 826B6FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6FF4: 4BDAFE2D  bl 0x82466e20
	ctx.lr = 0x826B6FF8;
	sub_82466E20(ctx, base);
	// 826B6FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7008 size=112
    let mut pc: u32 = 0x826B7008;
    'dispatch: loop {
        match pc {
            0x826B7008 => {
    //   block [0x826B7008..0x826B7078)
	// 826B7008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7014: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7018: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B701C: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B7020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7024: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 826B7028: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B702C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826B7030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B703C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7040: 386AF5F4  addi r3, r10, -0xa0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2572;
	// 826B7044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B704C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B705C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7064: 4BDAFDBD  bl 0x82466e20
	ctx.lr = 0x826B7068;
	sub_82466E20(ctx, base);
	// 826B7068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B706C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7078 size=112
    let mut pc: u32 = 0x826B7078;
    'dispatch: loop {
        match pc {
            0x826B7078 => {
    //   block [0x826B7078..0x826B70E8)
	// 826B7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7088: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B708C: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B7090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7094: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 826B7098: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B709C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826B70A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B70A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B70A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B70AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B70B0: 386AF624  addi r3, r10, -0x9dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2524;
	// 826B70B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B70B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B70BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B70C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B70C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B70C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B70CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B70D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B70D4: 4BDAFD4D  bl 0x82466e20
	ctx.lr = 0x826B70D8;
	sub_82466E20(ctx, base);
	// 826B70D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B70DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B70E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B70E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B70E8 size=112
    let mut pc: u32 = 0x826B70E8;
    'dispatch: loop {
        match pc {
            0x826B70E8 => {
    //   block [0x826B70E8..0x826B7158)
	// 826B70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B70EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B70F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B70F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B70F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B70FC: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7104: 390BA158  addi r8, r11, -0x5ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -24232;
	// 826B7108: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B710C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826B7110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7120: 386AF654  addi r3, r10, -0x9ac
	ctx.r[3].s64 = ctx.r[10].s64 + -2476;
	// 826B7124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B712C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7144: 4BDAFCDD  bl 0x82466e20
	ctx.lr = 0x826B7148;
	sub_82466E20(ctx, base);
	// 826B7148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B714C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7158 size=112
    let mut pc: u32 = 0x826B7158;
    'dispatch: loop {
        match pc {
            0x826B7158 => {
    //   block [0x826B7158..0x826B71C8)
	// 826B7158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B716C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7174: 390BA1B8  addi r8, r11, -0x5e48
	ctx.r[8].s64 = ctx.r[11].s64 + -24136;
	// 826B7178: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B717C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826B7180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7190: 386AF684  addi r3, r10, -0x97c
	ctx.r[3].s64 = ctx.r[10].s64 + -2428;
	// 826B7194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B71A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B71A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B71A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B71AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B71B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B71B4: 4BDAFC6D  bl 0x82466e20
	ctx.lr = 0x826B71B8;
	sub_82466E20(ctx, base);
	// 826B71B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B71BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B71C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B71C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B71C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B71C8 size=112
    let mut pc: u32 = 0x826B71C8;
    'dispatch: loop {
        match pc {
            0x826B71C8 => {
    //   block [0x826B71C8..0x826B7238)
	// 826B71C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B71CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B71D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B71D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B71D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B71DC: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B71E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B71E4: 390BA218  addi r8, r11, -0x5de8
	ctx.r[8].s64 = ctx.r[11].s64 + -24040;
	// 826B71E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826B71EC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826B71F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B71F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B71F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B71FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7200: 386AF6B4  addi r3, r10, -0x94c
	ctx.r[3].s64 = ctx.r[10].s64 + -2380;
	// 826B7204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B720C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B721C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7224: 4BDAFBFD  bl 0x82466e20
	ctx.lr = 0x826B7228;
	sub_82466E20(ctx, base);
	// 826B7228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B722C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


